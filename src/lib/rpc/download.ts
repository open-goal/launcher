import { invoke_rpc } from "./rpc";

export async function downloadFile(
  url: String,
  destination: String,
): Promise<void> {
  await invoke_rpc(
    "download_file",
    { url, destination },
    () => {},
    "Unable to download file",
  );
}
