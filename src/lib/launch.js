import { Command } from "@tauri-apps/api/shell";
import { resourceDir } from "@tauri-apps/api/path";

// TODO - is this set to `production` properly in release mode?
function isInDebugMode() {
  return process.env.NODE_ENV === "development";
}

// TODO - this is kind of a total hack - likely windows only currently
let debugPath;
if (isInDebugMode()) {
  let path = await resourceDir();
  debugPath = path.split("launcher")[0].split("?\\")[1];
  // debugPath += "\\launcher\\bundle-test\\data";
  debugPath += "\\launcher\\src-tauri\\data";
}

export async function launchGame() {
  let command;
  if (isInDebugMode()) {
    command = Command.sidecar(
      "bin/gk",
      ["-boot", "-fakeiso", "-debug", "-proj-path", debugPath],
      { cwd: "bin" }
    );
  } else {
    command = Command.sidecar("bin/gk", ["-boot", "-fakeiso", "-debug"], {
      cwd: "bin",
    });
  }
  await command.execute();
}
