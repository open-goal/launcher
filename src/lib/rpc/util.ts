import { invoke_rpc } from "./rpc";

export async function pathExists(directory: string): Promise<boolean> {
  return await invoke_rpc(
    "path_exists",
    { directory },
    () => false,
    "Unable to check if directory exists",
  );
}

export async function isMacOSVersion15OrAbove(): Promise<boolean | undefined> {
  return await invoke_rpc("is_macos_version_15_or_above", { }, () => undefined);
}