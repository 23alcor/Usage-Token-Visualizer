import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

type WindowName = "Five-hour" | "Seven-day";
type Snapshot = { window: WindowName; used?: number; resetsAt?: string; observedAt: string };
type NativeSnapshot = {
  poolId: "five-hour" | "seven-day";
  usedPercentage?: number;
  resetsAt?: string;
  observedAt: string;
};

const initialSnapshots: Snapshot[] = [
  { window: "Five-hour", observedAt: "No local reading yet" },
  { window: "Seven-day", observedAt: "No local reading yet" }
];

export default function App() {
  const [snapshots, setSnapshots] = useState(initialSnapshots);
  const [payload, setPayload] = useState("");
  const [message, setMessage] = useState("Paste a Claude Code status-line payload to create your first local reading.");

  async function importPayload() {
    try {
      JSON.parse(payload);
      const stored = await invoke<NativeSnapshot[]>("collect_claude_statusline", { payload });
      setSnapshots(stored.map((snapshot) => ({
        window: snapshot.poolId === "five-hour" ? "Five-hour" : "Seven-day",
        used: snapshot.usedPercentage,
        resetsAt: snapshot.resetsAt,
        observedAt: new Date(snapshot.observedAt).toLocaleString()
      })));
      setMessage("Saved a provider-reported quota snapshot locally.");
    } catch {
      setMessage("Unable to save this reading. Check that the complete Claude Code status-line JSON is valid.");
    }
  }

  return <main>
    <nav><strong>Token Usage</strong><span>Local-first AI capacity planner</span><button>Settings</button></nav>
    <section className="hero"><p className="eyebrow">OVERVIEW</p><h1>Know your available capacity.</h1><p>{message}</p></section>
    <section className="cards">
      {snapshots.map((snapshot) => <article key={snapshot.window}>
        <p className="label">CLAUDE CODE · {snapshot.window.toUpperCase()}</p>
        {snapshot.used === undefined ? <h2>Unavailable</h2> : <><h2>{100 - snapshot.used}% <small>remaining</small></h2><div className="meter"><i style={{ width: `${snapshot.used}%` }} /></div><p>{snapshot.used}% used</p></>}
        <footer>Observed: {snapshot.observedAt}{snapshot.resetsAt ? ` · Resets: ${snapshot.resetsAt}` : ""}</footer>
      </article>)}
    </section>
    <section className="importer"><div><p className="eyebrow">FIRST CONNECTION</p><h2>Import a Claude Code status line</h2><p>Use provider-reported usage. Token Usage does not collect credentials, prompts, or transcripts.</p></div><textarea aria-label="Claude Code status-line JSON" value={payload} onChange={(event) => setPayload(event.target.value)} placeholder='{"rate_limits":{"five_hour":{"used_percentage":35}}}' /><button className="primary" onClick={importPayload}>Save local reading</button></section>
  </main>;
}
