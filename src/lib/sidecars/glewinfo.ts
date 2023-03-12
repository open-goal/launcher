import { errorLog, exceptionLog, warnLog } from "$lib/rpc/logging";
import { os } from "@tauri-apps/api";
import { Command } from "@tauri-apps/api/shell";

export async function isOpenGLVersionSupported(
  version: string
): Promise<boolean | undefined> {
  if ((await os.platform()) === "darwin") {
    warnLog("MacOS isn't supported, OpenGL won't work here!");
    return false;
  }
  // Otherwise, query for the version
  let command = Command.sidecar("bin/glewinfo", ["-version", version]);
  try {
    const output = await command.execute();
    if (output.code === 0) {
      return true;
    }
    errorLog(
      `opengl requirement check failed - ${{
        version: version,
        statusCode: output.code,
        stdout: output.stdout,
        stderr: output.stderr,
      }}`
    );
    return false;
  } catch (e) {
    exceptionLog(`Unable to check for OpenGL support via glewinfo`, e);
    return undefined;
  }
}
