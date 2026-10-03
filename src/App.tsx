import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

type WindowName = "Five-hour" | "Seven-day";
type Snapshot = { window: WindowName; used?: number; resetsAt?: string; observedAt: string };
type NativeSnapshot = {
  poolId: "five-hour" | "seven-day";
  usedPercentage?: number;
  resetsAt?: string;
  observedAt: string;
};
type TrackingSetup = { status: "ready" | "installed" | "manualConfigurationRequired"; message: string };

const initialSnapshots: Snapshot[] = [
  { window: "Five-hour", observedAt: "No local reading yet" },
  { window: "Seven-day", observedAt: "No local reading yet" }
];

export default function App() {
  const [snapshots, setSnapshots] = useState(initialSnapshots);
  const [payload, setPayload] = useState("");
  const [message, setMessage] = useState("Enable automatic tracking or paste a Claude Code status-line payload to create your first reading.");
  const [trackingStatus, setTrackingStatus] = useState("Not connected");

  function applySnapshots(stored: NativeSnapshot[]) {
    setSnapshots(stored.map((snapshot) => ({
      window: snapshot.poolId === "five-hour" ? "Five-hour" : "Seven-day",
      used: snapshot.usedPercentage,
      resetsAt: snapshot.resetsAt,
      observedAt: new Date(snapshot.observedAt).toLocaleString()
    })));
  }

  useEffect(() => {
    const unlisten = listen<NativeSnapshot[]>("claude-quota-updated", (event) => {
      applySnapshots(event.payload);
      setTrackingStatus("Receiving local Claude Code updates");
      setMessage("Saved a fresh Claude Code quota snapshot locally.");
    });
    return () => { void unlisten.then((remove) => remove()); };
  }, []);

  async function importPayload() {
    try {
      JSON.parse(payload);
      const stored = await invoke<NativeSnapshot[]>("collect_claude_statusline", { payload });
      applySnapshots(stored);
      setMessage("Saved a provider-reported quota snapshot locally.");
    } catch {
      setMessage("Unable to save this reading. Check that the complete Claude Code status-line JSON is valid.");
    }
  }

  async function enableAutomaticTracking() {
    try {
      const result = await invoke<TrackingSetup>("setup_claude_code_tracking");
      setTrackingStatus(result.status === "manualConfigurationRequired" ? "Existing status line detected" : "Ready for Claude Code");
      setMessage(result.message);
    } catch {
      setTrackingStatus("Setup needs attention");
      setMessage("Unable to configure Claude Code tracking. Check the app has permission to write ~/.claude/settings.json.");
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
    <section className="importer setup"><div><p className="eyebrow">AUTOMATIC TRACKING</p><h2>Connect Claude Code</h2><p>{trackingStatus}. Token Usage adds its local bridge only when Claude Code does not already use a custom status line.</p></div><button className="primary" onClick={enableAutomaticTracking}>Enable automatic tracking</button></section>
    <section className="importer"><div><p className="eyebrow">MANUAL IMPORT</p><h2>Import a Claude Code status line</h2><p>Use provider-reported usage. Token Usage does not collect credentials, prompts, or transcripts.</p></div><textarea aria-label="Claude Code status-line JSON" value={payload} onChange={(event) => setPayload(event.target.value)} placeholder='{"rate_limits":{"five_hour":{"used_percentage":35}}}' /><button className="primary" onClick={importPayload}>Save local reading</button></section>
  </main>;
}
