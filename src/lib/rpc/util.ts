import { invoke_rpc } from "./rpc";

export async function pathExists(directory: string): Promise<boolean> {
    return await invoke_rpc(
      "path_exists",
      { directory },
      () => false,
      "Unable to check if directory exists",
    );
  }