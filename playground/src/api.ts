export const exec = async (code: string): Promise<{ stdout: string; stderr: string }> => {
  const resp = await fetch(import.meta.env.VITE_EXECUTOR_URL + "/execute", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ code }),
  });
  return resp.json();
};
export const disassemble = async (code: string): Promise<{ bytecode: string; stderr: string }> => {
  const resp = await fetch(import.meta.env.VITE_EXECUTOR_URL + "/disassemble", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ code }),
  });
  return resp.json();
};