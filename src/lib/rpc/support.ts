import { saveFilePrompt } from "$lib/utils/file-dialogs";
import { invoke_rpc } from "./rpc";

/**
 * @returns the path the support package was saved to, or `undefined` if it was cancelled or failed
 */
export async function generateSupportPackage(): Promise<string | undefined> {
  const userPath = await saveFilePrompt(
    "ZIP",
    ["zip"],
    "opengoal-support-package.zip",
  );
  if (!userPath) return undefined;
  return await invoke_rpc<string | undefined>(
    "generate_support_package",
    { userPath },
    () => undefined,
    () => userPath,
  );
}
