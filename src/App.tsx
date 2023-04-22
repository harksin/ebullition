import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [benchResult, setBenchResult] = useState([0]);
  const [name, setName] = useState("");
  const [desciption, setDescription] = useState("");
  const [url, setUrl] = useState("");
  const [duration_in_seconds, setDurationInSeconds] = useState(0);

  async function runBench() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setBenchResult(await invoke('run_http_bench', { name:name,desciption:desciption,url:url,duration:duration_in_seconds }));
  }

  return (
    <div className="container">
      <h1>Welcome Ebullition</h1>

      <p>Create your first naive bench</p>

      <div className="row">
        <form
          onSubmit={(e) => {
            e.preventDefault();
            runBench();
          }}
        >
          <input
            id="bench-name"
            onChange={(e) => setName(e.currentTarget.value)}
            placeholder="Enter a name..."
          />
          <input id="bench-description" onChange={(e) => setDescription(e.currentTarget.value)} placeholder="Enter a description..." />
          <input id="bench-url" onChange={(e) => setUrl(e.currentTarget.value)} placeholder="Enter a url..." />
          <input id="bench-duration" onChange={(e) => setDurationInSeconds(Number(e.currentTarget.value))} placeholder="Enter a duration in seconds..." />

          <button type="submit">Run the bench</button>
        </form>
      </div>
      <p>{benchResult}</p>
    </div>
  );
}

export default App;
