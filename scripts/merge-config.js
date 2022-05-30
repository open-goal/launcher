// The tauri-action for Github Actions doesn't let you merge configs
// like can be done with --config on the `tauri` command
// instead it replaces the file
//
// So for now, do the merge ourselves, why? because we don't want to maintain two configs
// that will just end up drifting and breaking on release time.

import { readFileSync, writeFileSync } from "fs";

let releaseOptions = JSON.parse(readFileSync("./.tauri/release-config.json"));
let existingConfig = JSON.parse(readFileSync("./src-tauri/tauri.conf.json"));

existingConfig.tauri.updater = releaseOptions.tauri.updater;

writeFileSync(
  "./.tauri/release-config.combined.json",
  JSON.stringify(existingConfig, null, 2)
);
