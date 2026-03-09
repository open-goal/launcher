import { invoke_rpc } from "./rpc";

export async function isMacOSVersion15OrAbove(): Promise<boolean | undefined> {
  return await invoke_rpc("is_macos_version_15_or_above", {});
}
