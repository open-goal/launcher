/**
 * This script is used to rename the binary with the platform specific postfix.
 * When `tauri build` is ran, it looks for the binary name appended with the platform specific postfix.
 */

import { execa } from "execa";
import { copyFileSync, existsSync, mkdirSync, renameSync } from "fs";

if (!existsSync("src-tauri/bin")) {
  mkdirSync(`src-tauri/bin`);
}

let extension = "";
if (process.platform === "win32") {
  extension = ".exe";
}

const rustInfo = (await execa("rustc", ["-vV"])).stdout;
const targetTriple = /host: (\S+)/g.exec(rustInfo)[1];
if (!targetTriple) {
  console.error("Failed to determine platform target triple");
}
if (existsSync(`src-tauri/bin/glewinfo${extension}`)) {
  renameSync(
    `src-tauri/bin/glewinfo${extension}`,
    `src-tauri/bin/glewinfo-${targetTriple}${extension}`
  );
} else {
  if (process.platform === "win32") {
    copyFileSync(
      `third-party/glew_2.2.0/windows/glewinfo.exe`,
      `src-tauri/bin/glewinfo-${targetTriple}${extension}`
    );
  } else {
    copyFileSync(
      `third-party/glew_2.2.0/linux/glewinfo`,
      `src-tauri/bin/glewinfo-${targetTriple}${extension}`
    );
  }
}
