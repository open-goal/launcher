import { open, save } from "@tauri-apps/api/dialog";

export async function filePromptNoFilters(
  title: string,
): Promise<string | null> {
  const path = await open({
    title: title,
    multiple: false,
    directory: false,
    filters: undefined,
  });

  if (Array.isArray(path) || path === null) {
    return null;
  }

  return path;
}

export async function filePrompt(
  extensions: string[],
  name: string,
  title: string,
): Promise<string | null> {
  const path = await open({
    title: title,
    multiple: false,
    directory: false,
    filters: [{ extensions: extensions, name: name }],
  });

  if (Array.isArray(path) || path === null) {
    return null;
  }

  return path;
}

export async function saveFilePrompt(
  fileType: string,
  fileExtensions: string[],
  fileName: string,
): Promise<string | null> {
  return await save({
    filters: [{ name: fileType, extensions: fileExtensions }],
    defaultPath: fileName,
  });
}

export async function saveFolderPrompt(
  fileType: string,
  fileExtensions: string[],
): Promise<string | null> {
  return await save({
    filters: [{ name: fileType, extensions: fileExtensions }],
  });
}

export async function isoPrompt(
  fileExplanation: string,
  title: string,
): Promise<string | undefined> {
  const path = await filePrompt(["ISO, zip", "iso"], fileExplanation, title);
  if (path === null) {
    return undefined;
  }
  return path;
}

export async function folderPrompt(title: string): Promise<string | undefined> {
  const path = await open({
    title: title,
    multiple: false,
    directory: true,
  });

  if (Array.isArray(path) || path === null) {
    return undefined;
  }

  return path;
}
