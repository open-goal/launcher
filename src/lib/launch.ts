import { Command } from "@tauri-apps/api/shell";
import { resourceDir } from "@tauri-apps/api/path";

function isInDebugMode() {
  return process.env.NODE_ENV === "development";
}

// NOTE - this is kind of a total hack - likely windows only currently
let debugPath: string;
if (isInDebugMode()) {
  let path = await resourceDir();
  debugPath = path.split("launcher")[0].split("?\\")[1];
  debugPath += "launcher\\src-tauri\\data";
}

export async function launchGame() {
  let command: Command;
  if (isInDebugMode()) {
    console.log(debugPath);
    command = Command.sidecar(
      "bin/gk",
      ["-boot", "-fakeiso", "-debug", "-proj-path", debugPath],
      { cwd: "bin" }
    );
  } else {
    command = Command.sidecar("bin/gk", ["-boot", "-fakeiso", "-debug"]);
  }
  command.spawn();
}
