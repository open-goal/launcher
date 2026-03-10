import { toastStore } from "$lib/stores/ToastStore";
import { invoke, type InvokeArgs } from "@tauri-apps/api/core";
import { errorLog, exceptionLog } from "./logging";

// TODO: make args optional, add optional object encapsulating the optional params, this makes the call sites easier to read

/**
 * Wrapper around Tauri invoke that logs all errors.
 * If the error is a string, it is also displayed as a toast.
 *
 * @param cmd The command to send to the backend
 * @param args The arguments for the command
 * @param onError If an error occurs, this is called with the error before returning
 * @param onSuccess If the call succeeds, this is called with the result before returning
 */
export async function invoke_rpc<T>(
  cmd: string,
  args: InvokeArgs,
  onError?: (error: unknown) => T,
  onSuccess?: (result: T) => T,
): Promise<T> {
  try {
    // this assumes the call is made in a way that does not trick the type inference
    const result: T = await invoke(cmd, args);
    return onSuccess ? onSuccess(result) : result;
  } catch (e: any) {
    if (typeof e === "string") {
      errorLog(`Error calling '${cmd}': ${e}`);
    } else {
      exceptionLog(`Error calling '${cmd}'`, e);
    }
    toastStore.makeToast(String(e), "error");
    return onError ? onError(e) : e;
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
