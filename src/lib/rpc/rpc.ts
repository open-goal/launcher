import { toastStore } from "$lib/stores/ToastStore";
import { invoke, type InvokeArgs } from "@tauri-apps/api/core";
import { errorLog, exceptionLog } from "./logging";

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
  handleError: (error: unknown) => T,
  toastOnError?: string,
  onSuccess?: (result: T) => T,
): Promise<T> {
  try {
    // this assumes the call is made in a way that does not trick the type inference
    const result: T = await invoke(cmd, args);
    if (onSuccess) {
      return onSuccess(result);
    }
    return result;
  } catch (e: any) {
    if (typeof e === "string") {
      errorLog(`Error calling '${cmd}': ${e}`);
    } else {
      exceptionLog(`Error calling '${cmd}'`, e);
    }
    // TODO - this is a dumb hack but whatever for now
    if (toastOnError === "_mirror_") {
      toastStore.makeToast(e, "error");
    } else {
      const toastMessage = toastOnError ?? "An unexpected error occurred";
      toastStore.makeToast(toastMessage, "error");
    }
    return handleError(e);
  }
}
