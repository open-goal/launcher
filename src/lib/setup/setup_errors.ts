import { fileExists } from "../utils/file";
import { appDir, join } from "@tauri-apps/api/path";
import { readTextFile } from "@tauri-apps/api/fs";
import { log } from "$lib/utils/log";

interface ErrorCodeMetadataEntry {
  msg: string;
}

let errorMetadata = new Map<string, ErrorCodeMetadataEntry>();

export async function resolveErrorCode(
  code: number
): Promise<string | undefined> {
  if (errorMetadata === undefined || errorMetadata.size == 0) {
    // first time, load the metadata
    const errorMetadataPath = await join(
      await appDir(),
      "data",
      "launcher",
      "error-code-metadata.json"
    );
    if (!(await fileExists(errorMetadataPath))) {
      log.warn("could not locate error metadata file at path", {
        path: errorMetadataPath,
      });
      return undefined;
    }
    const jsonData = JSON.parse(await readTextFile(errorMetadataPath));
    for (var value in jsonData) {
      errorMetadata.set(value, jsonData[value]);
    }
  }

  if (errorMetadata.has(code.toString())) {
    return errorMetadata.get(code.toString()).msg;
  }

  return undefined;
}
