import { saveFilePrompt } from "$lib/utils/file-dialogs";
import { invoke_rpc } from "./rpc";

export async function generateSupportPackage(): Promise<void> {
  const userPath = await saveFilePrompt(
    "ZIP",
    ["zip"],
    "opengoal-support-package.zip",
  );
  return await invoke_rpc(
    "generate_support_package",
    { userPath },
    () => {},
    "Unable to create support package",
  );
}
