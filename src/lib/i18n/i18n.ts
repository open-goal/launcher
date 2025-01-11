import { addMessages, init, register } from "svelte-i18n";

export interface Locale {
  id: string;
  flag: string;
  localizedName: string;
  rtl: boolean;
  fontFamily?: string;
  fontFileName?: string;
  fontDownloadUrl?: string;
}

// https://omniglot.com/language/names.htm
export const AVAILABLE_LOCALES: Locale[] = [
  {
    id: "af-ZA",
    flag: "🇿🇦",
    localizedName: "Afrikaans",
    rtl: false
  },
  {
    id: "ar-SA",
    flag: "🇸🇦",
    localizedName: "العربية الفصحى",
    fontFamily: "Noto Sans Arabic",
    fontFileName: "NotoSansArabic-VariableFont.woff2",
    fontDownloadUrl:
      "https://github.com/open-goal/launcher-assets/releases/download/fonts%2Fv1.0.0/NotoSansArabic-VariableFont.woff2",
    rtl: true
  },
  {
    id: "ca-ES",
    flag: "🇪🇸",
    localizedName: "Català",
    rtl: false
  },
  {
    id: "cs-CZ",
    flag: "🇨🇿",
    localizedName: "Čeština",
    rtl: false
  },
  {
    id: "da-DK",
    flag: "🇩🇰",
    localizedName: "Dansk",
    rtl: false
  },
  {
    id: "de-DE",
    flag: "🇩🇪",
    localizedName: "Deutsch",
    rtl: false
  },
  {
    id: "el-GR",
    flag: "🇬🇷",
    localizedName: "Ελληνικά",
    rtl: false
  },
  {
    id: "en-US",
    flag: "🇺🇸",
    localizedName: "English",
    rtl: false
  },
  {
    id: "en-GB",
    flag: "🇬🇧",
    localizedName: "English UK",
    rtl: false
  },
  {
    id: "es-ES",
    flag: "🇪🇸",
    localizedName: "Español",
    rtl: false
  },
  {
    id: "fi-FI",
    flag: "🇫🇮",
    localizedName: "Suomi",
    rtl: false
  },
  {
    id: "fr-FR",
    flag: "🇫🇷",
    localizedName: "Français",
    rtl: false
  },
  {
    id: "he-IL",
    flag: "🇮🇱",
    localizedName: "עברית",
    rtl: true
  },
  {
    id: "hu-HU",
    flag: "🇭🇺",
    localizedName: "Magyar",
    rtl: false
  },
  {
    id: "it-IT",
    flag: "🇮🇹",
    localizedName: "Italiano",
    rtl: false
  },
  {
    id: "ja-JP",
    flag: "🇯🇵",
    localizedName: "日本語",
    fontFamily: "Noto Sans JP",
    fontFileName: "NotoSansJP-VariableFont.woff2",
    fontDownloadUrl:
      "https://github.com/open-goal/launcher-assets/releases/download/fonts%2Fv1.0.0/NotoSansJP-VariableFont.woff2",
    rtl: false
  },
  {
    id: "ko-KR",
    flag: "🇰🇷",
    localizedName: "한국어",
    fontFamily: "Noto Sans KR",
    fontFileName: "NotoSansKR-VariableFont_wght.woff2",
    fontDownloadUrl:
      "https://github.com/open-goal/launcher-assets/releases/download/fonts%2Fv1.0.0/NotoSansKR-VariableFont_wght.woff2",
    rtl: false
  },
  {
    id: "lt-LT",
    flag: "🇱🇹",
    localizedName: "Lietuvių kalba",
    rtl: false
  },
  {
    id: "nl-NL",
    flag: "🇳🇱",
    localizedName: "Nederlands",
    rtl: false
  },
  {
    id: "no-NO",
    flag: "🇳🇴",
    localizedName: "Norsk",
    rtl: false
  },
  {
    id: "pl-PL",
    flag: "🇵🇱",
    localizedName: "Polski",
    rtl: false
  },
  {
    id: "pt-BR",
    flag: "🇧🇷",
    localizedName: "Português",
    rtl: false
  },
  {
    id: "pt-PT",
    flag: "🇵🇹",
    localizedName: "Português",
    rtl: false
  },
  {
    id: "ro-RO",
    flag: "🇷🇴",
    localizedName: "Română",
    rtl: false
  },
  {
    id: "ru-RU",
    flag: "🇷🇺",
    localizedName: "Русский",
    rtl: false
  },
  {
    id: "sr-SP",
    flag: "🇷🇸",
    localizedName: "Srpski",
    rtl: false
  },
  {
    id: "sv-SE",
    flag: "🇸🇪",
    localizedName: "Svenska",
    rtl: false
  },
  {
    id: "tr-TR",
    flag: "🇹🇷",
    localizedName: "Türkçe",
    rtl: false
  },
  {
    id: "uk-UA",
    flag: "🇺🇦",
    localizedName: "Українська",
    rtl: false
  },
  {
    id: "vi-VN",
    flag: "🇻🇳",
    localizedName: "Tiếng Việt",
    rtl: false
  },
  {
    id: "zh-CN",
    flag: "🇨🇳",
    localizedName: "简体中文",
    fontFamily: "Noto Sans SC",
    fontFileName: "NotoSansSC-VariableFont_wght.woff2",
    fontDownloadUrl:
      "https://github.com/open-goal/launcher-assets/releases/download/fonts%2Fv1.0.0/NotoSansSC-VariableFont_wght.woff2",
    rtl: false
  },
  {
    id: "zh-TW",
    flag: "🇹🇼",
    localizedName: "繁體中文",
    fontFamily: "Noto Sans TC",
    fontFileName: "NotoSansTC-VariableFont_wght.woff2",
    fontDownloadUrl:
      "https://github.com/open-goal/launcher-assets/releases/download/fonts%2Fv1.0.0/NotoSansTC-VariableFont_wght.woff2",
    rtl: false
  },
];

export async function initLocales(async: boolean) {
  if (async) {
    for (const locale of AVAILABLE_LOCALES) {
      register(
        locale.id,
        () => import(`../../assets/translations/${locale.id}.json`),
      );
    }
  } else {
    for (const locale of AVAILABLE_LOCALES) {
      addMessages(
        locale.id,
        await import(`../../assets/translations/${locale.id}.json`),
      );
    }
  }
  const initPromise = init({
    fallbackLocale: "en-US",
    initialLocale: "en-US",
  });
  if (!async) {
    await initPromise;
  } else {
    return initPromise;
  }
}
