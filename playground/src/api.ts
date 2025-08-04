export const exec = async (code: string): Promise<{ stdout: string; stderr: string }> => {
  const resp = await fetch(import.meta.env.VITE_EXECUTOR_URL + "/execute", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ code }),
  });
  return resp.json();
};
