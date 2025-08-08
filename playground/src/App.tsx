import { useState } from "react";
import { Editor } from "./Editor";
import { exec, disassemble } from "./api";
import { Analytics } from "@vercel/analytics/react"
import "./styles.css";

import { Panel, PanelGroup, PanelResizeHandle } from "react-resizable-panels";

const PlayIcon = () => (
  <svg viewBox="0 0 24 24" fill="currentColor" style={{ marginRight: 8, height: 16, width: 16 }}>
    <path d="M8 5v14l11-7z" />
  </svg>
);

export default function App() {
  const [code, setCode] = useState('print("Hello, Tundra!")');
  const [output, setOutput] = useState("");
  const [bytecode, setBytecode] = useState("");
  const [isRunning, setIsRunning] = useState(false);

  const runCode = async () => {
    setIsRunning(true);
    setOutput("⏳ running…");
    setBytecode("⏳ compiling…");
    try {
      const [execRes, disRes] = await Promise.all([exec(code), disassemble(code)]);
      setOutput(execRes.stderr || execRes.stdout);
      setBytecode(disRes.stderr || disRes.bytecode);
    } catch (error) {
      setOutput(`Failed to connect to executor: ${error}`);
      setBytecode("");
    } finally {
      setIsRunning(false);
    }
  };

  const wikiUrl = import.meta.env.VITE_WIKI_URL ?? "https://aayush-tripathi.github.io/tundra/";

  return (
    <div style={{ height: "100vh", display: "flex", flexDirection: "column" }}>
      {/* Header / Toolbar */}
      <header style={{ display: "flex", alignItems: "center", gap: 12, padding: 8, borderBottom: "1px solid #30363d" }}>
        <img src="/tundra_base_logo.png" alt="Tundra Logo" className="logo" />
        <h1 style={{ fontSize: "1.1rem", fontWeight: 600, marginRight: "auto" }}>Tundra Playground</h1>

        <a href={wikiUrl} target="_blank" rel="noreferrer" className="wiki-button">Wiki</a> 

        <button onClick={runCode} disabled={isRunning} className="run-button">
          <PlayIcon /> Run
        </button>
      </header>

      {/* Resizable Editor and Console */}
      <PanelGroup direction="vertical" style={{ flex: 1 }}>
        <Panel defaultSize={65} minSize={20}>
          <Editor code={code} setCode={setCode} />
        </Panel>

        <PanelResizeHandle className="resize-handle"><div className="resize-handle-bar" /></PanelResizeHandle>

        <Panel defaultSize={35} minSize={10}>
          {/*  Output | Bytecode  */}
          <PanelGroup direction="horizontal" style={{ height: "100%" }}>
            <Panel defaultSize={50} minSize={10}>
              <div style={{ height: "100%", display: "flex", flexDirection: "column" }}>
                <div style={{ padding: 8, borderBottom: "1px solid #30363d", backgroundColor: "#161b22" }}>
                  <h2 style={{ margin: 0, fontSize: "0.9rem", fontWeight: 600 }}>Output</h2>
                </div>
                <pre style={{ flex: 1, margin: 0, padding: 12, fontFamily: "monospace", whiteSpace: "pre-wrap", overflow: "auto" }}>
                  {output}
                </pre>
              </div>
            </Panel>

            <PanelResizeHandle className="resize-handle vertical"><div className="resize-handle-bar" /></PanelResizeHandle>

            <Panel defaultSize={50} minSize={10}>
              <div style={{ height: "100%", display: "flex", flexDirection: "column" }}>
                <div style={{ padding: 8, borderBottom: "1px solid #30363d", backgroundColor: "#161b22" }}>
                  <h2 style={{ margin: 0, fontSize: "0.9rem", fontWeight: 600 }}>Bytecode</h2>
                </div>
                <pre style={{ flex: 1, margin: 0, padding: 12, fontFamily: "monospace", whiteSpace: "pre", overflow: "auto" }}>
                  {bytecode}
                </pre>
              </div>
            </Panel>
          </PanelGroup>
        </Panel>
      </PanelGroup>
      <Analytics />
    </div>
  );
}
