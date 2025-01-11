import { locale as svelteLocale } from "svelte-i18n";
import { invoke_rpc } from "./rpc";
import { AVAILABLE_LOCALES } from "$lib/i18n/i18n";
import { exists } from "@tauri-apps/plugin-fs";
import { appDataDir, join } from "@tauri-apps/api/path";
import { convertFileSrc } from "@tauri-apps/api/core";

export async function setLocale(localeId: string): Promise<void> {
  return await invoke_rpc(
    "update_setting_value",
    { key: "locale", val: localeId },
    () => {},
    undefined, // no toast
    async () => {
      svelteLocale.set(localeId);
      // Update CSS variable if needed
      let localeInfo = AVAILABLE_LOCALES.find(
        (locale) => locale.id === localeId,
      );
      if (
        localeInfo !== undefined &&
        localeInfo.fontFamily !== undefined &&
        localeInfo.fontFileName !== undefined
      ) {
        // Dynamically get the font
        const fontPath = await join(
          await appDataDir(),
          "fonts",
          localeInfo.fontFileName,
        );
        const fontExists = await exists(fontPath);
        if (fontExists) {
          const assetUrl = convertFileSrc(fontPath);
          var newFontStyle = document.createElement("style");
          newFontStyle.appendChild(
            document.createTextNode(
              `@font-face {\nfont-family: "${localeInfo.fontFamily}";\nsrc: url('${assetUrl}');\n}\n`,
            ),
          );
          document.head.appendChild(newFontStyle);
          document.documentElement.style.setProperty(
            "--launcher-font-family",
            localeInfo.fontFamily,
          );
        } else {
          document.documentElement.style.setProperty(
            "--launcher-font-family",
            "Noto Sans",
          );
        }
      } else {
        document.documentElement.style.setProperty(
          "--launcher-font-family",
          "Noto Sans",
        );
      }
    },
  );
}
