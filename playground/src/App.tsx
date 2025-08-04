import { useState } from "react";
import { Editor } from "./Editor";
import { exec } from "./api";

export default function App() {
  const [code, setCode] = useState('print("Hello, Tundra!")\n');
  const [out, setOut] = useState("");

  const run = async () => {
    setOut("⏳ running…");
    const { stdout, stderr } = await exec(code);
    setOut(stderr || stdout);
  };

  return (
    <div style={{ display: "flex", flexDirection: "column", height: "100vh" }}>
      <Editor code={code} setCode={setCode} />
      <button onClick={run} style={{ padding: "8px 16px" }}>
        ▶ Run
      </button>
      <pre style={{ flex: 1, margin: 0, padding: "12px", background: "#111", color: "#0f0" }}>
        {out}
      </pre>
    </div>
  );
}
