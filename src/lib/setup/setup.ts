import { Command } from "@tauri-apps/api/shell";
import { os } from "@tauri-apps/api";
import { resolveErrorCode } from "./setup_errors";
import { installLog, log } from "$lib/utils/log";

// We'll leave this check here since it's the only thing still using a sidecar
//
// TODO - longterm, we should build this validation checking into the `extractor`
// because other things like Vulkan don't have a glewinfo equivalent
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
  log.error("opengl requirement check failed", {
    version: version,
    statusCode: output.code,
    stdout: output.stdout,
    stderr: output.stderr,
  });
  return false;
}

async function handleErrorCode(code: number, stepName: string) {
  // isInstalling.update(() => false);
  // const explaination = await resolveErrorCode(code);
  // if (explaination === undefined) {
  //   throw new Error(`${stepName} exited with unexpected code: ${code}`);
  // }
  // throw new Error(explaination);
}
