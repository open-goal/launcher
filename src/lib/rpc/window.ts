import { invoke_rpc } from "./rpc";

export async function openDir(directory: string): Promise<void> {
  return await invoke_rpc(
    "open_dir_in_os",
    { directory },
    () => {},
    "Unable to open directory",
  );
}

export async function openMainWindow(): Promise<boolean> {
  return await invoke_rpc("open_main_window", {}, () => false);
}
