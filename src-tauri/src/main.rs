// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use async_trait::async_trait;

use tokio::runtime::Runtime;

use std::time::{Duration, Instant};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command(rename_all = "snake_case")]
fn run_http_bench(
    name: String,
    desciption: Option<String>,
    url: String,
    duration: u64,
) -> Vec<TotalCalls> {
    format!("Starting naive http bench");

    let step = HttpGetStep {
        url,
        duration: Duration::from_secs(duration),
    };

    let bench = Benchmark {
        name,
        description: desciption,
        senarios: vec![step],
    };

    let results = bench.execute();

    results
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![run_http_bench])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

struct Benchmark<T>
where
    T: Step,
{
    name: String,
    description: Option<String>,
    senarios: Vec<T>,
}

impl<T> Benchmark<T>
where
    T: Step + Clone,
{
    fn execute(&self) -> Vec<TotalCalls> {
        let s = self.senarios.clone();

        let rt = Runtime::new().unwrap();

        let bench_output = rt.block_on(async {
            let mut results = Vec::<TotalCalls>::new();
            for senario in s {
                let calls = senario.run().await;
                results.push(calls);
            }

            results
        });

        rt.shutdown_background();

        bench_output
    }
}

type TotalDuration = u32;
type TotalCalls = u32;
#[async_trait]
trait Step {
    async fn run(&self) -> TotalCalls;
}

#[derive(Clone)]
struct HttpGetStep {
    url: String,
    duration: Duration,
}

#[async_trait]
impl Step for HttpGetStep {
    async fn run(&self) -> TotalCalls {
        let step_start = Instant::now();
        let mut amount_of_requests = 0;

        while step_start.elapsed() < self.duration {
            let start = Instant::now();
            let resp = reqwest::get(self.url.clone()).await.unwrap().status();

            let duration = start.elapsed().as_millis() as u32;
            amount_of_requests += 1;
            println!("{}: {}ms", self.url, duration);
        }

        amount_of_requests
    }
}
