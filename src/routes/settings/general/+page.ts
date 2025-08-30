import { appDataDir, join } from "@tauri-apps/api/path";
import type { PageLoad } from "./$types";
import { downloadFile } from "$lib/rpc/download";

// const localeFontForDownload = await localeSpecificFontAvailableForDownload($currentLocale); // TODO: FIND A BETTER HOME FOR THIS CODE (root route?)
// let localeFontForDownload: Locale | undefined = undefined;

async function downloadLocaleFont() {
  const fontPath = await join(
    await appDataDir(),
    "fonts",
    localeFont.fontFileName,
  );
  await downloadFile(localeFont.fontDownloadUrl, fontPath);
}

export const load = (async () => {
  const localeFont = {};
  return { localeFont };
}) satisfies PageLoad;
