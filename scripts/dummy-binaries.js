/**
 * Used for builds / CI to basically mock the install process
 *
 * If Tauri can't find the binaries, it will fail (maybe there is a better way to ignore this though)
 */

import { existsSync, rmdirSync, mkdirSync, writeFileSync } from "fs";

// Clear our current binaries
if (existsSync("./src-tauri/bin")) {
  rmdirSync("./src-tauri/bin", { recursive: true, force: true });
}
// Recreate the directory
mkdirSync("./src-tauri/bin");
mkdirSync("./src-tauri/data");
// Create empty executables
let extension = "";
if (process.platform === "win32") {
  extension = ".exe";
}
writeFileSync(`./src-tauri/bin/extractor${extension}`, "dummy");
writeFileSync(`./src-tauri/bin/gk${extension}`, "dummy");
writeFileSync(`./src-tauri/bin/goalc${extension}`, "dummy");
writeFileSync(`./src-tauri/bin/glewinfo${extension}`, "dummy");
writeFileSync(`./src-tauri/data/something.txt`, "dummy");
