import { os } from "@tauri-apps/api";
import { Command } from "@tauri-apps/api/shell";

export async function isOpenGLVersionSupported(
  version: string
): Promise<boolean> {
  if ((await os.platform()) === "darwin") {
    console.log("[OG]: MacOS isn't supported, OpenGL won't work here!");
    return false;
  }
  // Otherwise, query for the version
  let command = Command.sidecar("bin/glewinfo", ["-version", version]);
  const output = await command.execute();
  if (output.code === 0) {
    return true;
  }
  console.log("opengl requirement check failed", {
    version: version,
    statusCode: output.code,
    stdout: output.stdout,
    stderr: output.stderr,
  });
  return false;
}
