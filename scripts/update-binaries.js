/**
 * This script is just a fast way of copying over the `jak-project` binaries built locally into here
 * This is useful if you are locally developing both
 *
 * Assumes that `jak-project` is one directory up
 */

const fs = require('fs');

// Clear our current binaries
if (fs.existsSync("./src-tauri/bin")) {
  fs.rmdirSync("./src-tauri/bin", {recursive: true, force: true});
}
// Recreate the directory
fs.mkdirSync("./src-tauri/bin");
// Copy over the necessary binaries
// - Assumes Windows!
fs.copyFileSync("../jak-project/out/build/Release/bin/extractor.exe", "./src-tauri/bin/extractor.exe");
fs.copyFileSync("../jak-project/out/build/Release/bin/gk.exe", "./src-tauri/bin/gk.exe");
fs.copyFileSync("../jak-project/out/build/Release/bin/goalc.exe", "./src-tauri/bin/goalc.exe");
