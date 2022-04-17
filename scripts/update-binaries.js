/**
 * This script is just a fast way of copying over the `jak-project` binaries built locally into here
 * This is useful if you are locally developing both
 *
 * Assumes that `jak-project` is one directory up
 */

import { existsSync, rmdirSync, mkdirSync, copyFileSync } from "fs";

// Clear our current binaries
if (existsSync("./src-tauri/bin")) {
  rmdirSync("./src-tauri/bin", { recursive: true, force: true });
}
// Recreate the directory
mkdirSync("./src-tauri/bin");
// Copy over the necessary binaries
// - Assumes Windows!
copyFileSync(
  "../jak-project/out/build/Release/bin/extractor.exe",
  "./src-tauri/bin/extractor.exe"
);
copyFileSync(
  "../jak-project/out/build/Release/bin/gk.exe",
  "./src-tauri/bin/gk.exe"
);
copyFileSync(
  "../jak-project/out/build/Release/bin/goalc.exe",
  "./src-tauri/bin/goalc.exe"
);
copyFileSync(
  "./third-party/glew/windows/glewinfo.exe",
  "./src-tauri/bin/glewinfo.exe"
);
