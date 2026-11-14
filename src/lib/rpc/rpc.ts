import { toastStore } from "$lib/stores/ToastStore";
import { invoke, type InvokeArgs } from "@tauri-apps/api/core";
import { errorLog, exceptionLog } from "./logging";

/**
 * Invokes a Tauri command, logging and displaying errors before rethrowing.
 * @param cmd The command to send to the backend
 * @param args The arguments for the command
 */
export async function invoke_rpc<T>(
  cmd: string,
  args?: InvokeArgs,
): Promise<T> {
  try {
    return await invoke<T>(cmd, args);
  } catch (e: unknown) {
    if (typeof e === "string") {
      errorLog(`Error calling '${cmd}': ${e}`);
    } else {
      exceptionLog(`Error calling '${cmd}'`, e);
    }
    toastStore.makeToast(String(e), "error");
    throw e;
  }
}

// TODO: TEMPORARY SECOND INVOKE WRAPPER WHILE I EXPERIMENT WITH THE SYSTEM
// used for any tauri::command with the return type: Result<(), Error>

/**
 * Wrapper around Tauri invoke that logs all errors.
 * If the error is a string, it is also displayed as a toast.
 *
 * @param cmd The command to send to the backend
 * @param args The arguments for the command
 * @returns `null` on success, `string` on error
 */
export async function invoke_rpc2(
  cmd: string,
  options?: {
    args?: InvokeArgs;
  },
): Promise<string | null> {
  try {
    await invoke(cmd, options?.args);
    return null;
  } catch (e: any) {
    if (typeof e === "string") {
      errorLog(`Error calling '${cmd}': ${e}`);
    } else {
      exceptionLog(`Error calling '${cmd}'`, e);
    }
    toastStore.makeToast(String(e), "error");
    return String(e);
  }
}
