import { getLocaleFromNavigator, init, register } from "svelte-i18n";

export interface Locale {
  id: string;
  flag: string;
  localizedName: string;
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
  },
  {
    id: "ar-SA",
    flag: "🇸🇦",
    localizedName: "العربية الفصحى",
    fontFamily: "Noto Sans Arabic",
    fontFileName: "NotoSansArabic-VariableFont.woff2",
    fontDownloadUrl:
      "https://github.com/open-goal/launcher-assets/releases/download/fonts%2Fv1.0.0/NotoSansArabic-VariableFont.woff2",
  },
  {
    id: "bs-BA",
    flag: "🇧🇦",
    localizedName: "Bosanski",
  },
  {
    id: "ca-ES",
    flag: "🇪🇸",
    localizedName: "Català",
  },
  {
    id: "cs-CZ",
    flag: "🇨🇿",
    localizedName: "Čeština",
  },
  {
    id: "da-DK",
    flag: "🇩🇰",
    localizedName: "Dansk",
  },
  {
    id: "de-DE",
    flag: "🇩🇪",
    localizedName: "Deutsch",
  },
  {
    id: "el-GR",
    flag: "🇬🇷",
    localizedName: "Ελληνικά",
  },
  {
    id: "en-US",
    flag: "🇺🇸",
    localizedName: "English",
  },
  {
    id: "en-GB",
    flag: "🇬🇧",
    localizedName: "English UK",
  },
  {
    id: "es-ES",
    flag: "🇪🇸",
    localizedName: "Español",
  },
  {
    id: "fi-FI",
    flag: "🇫🇮",
    localizedName: "Suomi",
  },
  {
    id: "fr-FR",
    flag: "🇫🇷",
    localizedName: "Français",
  },
  {
    id: "he-IL",
    flag: "🇮🇱",
    localizedName: "עברית",
  },
  {
    id: "hu-HU",
    flag: "🇭🇺",
    localizedName: "Magyar",
  },
  {
    id: "it-IT",
    flag: "🇮🇹",
    localizedName: "Italiano",
  },
  {
    id: "ja-JP",
    flag: "🇯🇵",
    localizedName: "日本語",
    fontFamily: "Noto Sans JP",
    fontFileName: "NotoSansJP-VariableFont.woff2",
    fontDownloadUrl:
      "https://github.com/open-goal/launcher-assets/releases/download/fonts%2Fv1.0.0/NotoSansJP-VariableFont.woff2",
  },
  {
    id: "ko-KR",
    flag: "🇰🇷",
    localizedName: "한국어",
    fontFamily: "Noto Sans KR",
    fontFileName: "NotoSansKR-VariableFont_wght.woff2",
    fontDownloadUrl:
      "https://github.com/open-goal/launcher-assets/releases/download/fonts%2Fv1.0.0/NotoSansKR-VariableFont_wght.woff2",
  },
  {
    id: "lt-LT",
    flag: "🇱🇹",
    localizedName: "Lietuvių kalba",
  },
  {
    id: "nl-NL",
    flag: "🇳🇱",
    localizedName: "Nederlands",
  },
  {
    id: "no-NO",
    flag: "🇳🇴",
    localizedName: "Norsk",
  },
  {
    id: "pl-PL",
    flag: "🇵🇱",
    localizedName: "Polski",
  },
  {
    id: "pt-BR",
    flag: "🇧🇷",
    localizedName: "Português",
  },
  {
    id: "pt-PT",
    flag: "🇵🇹",
    localizedName: "Português",
  },
  {
    id: "ro-RO",
    flag: "🇷🇴",
    localizedName: "Română",
  },
  {
    id: "ru-RU",
    flag: "🇷🇺",
    localizedName: "Русский",
  },
  {
    id: "sr-SP",
    flag: "🇷🇸",
    localizedName: "Srpski",
  },
  {
    id: "sv-SE",
    flag: "🇸🇪",
    localizedName: "Svenska",
  },
  {
    id: "tr-TR",
    flag: "🇹🇷",
    localizedName: "Türkçe",
  },
  {
    id: "uk-UA",
    flag: "🇺🇦",
    localizedName: "Українська",
  },
  {
    id: "vi-VN",
    flag: "🇻🇳",
    localizedName: "Tiếng Việt",
  },
  {
    id: "zh-CN",
    flag: "🇨🇳",
    localizedName: "简体中文",
    fontFamily: "Noto Sans SC",
    fontFileName: "NotoSansSC-VariableFont_wght.woff2",
    fontDownloadUrl:
      "https://github.com/open-goal/launcher-assets/releases/download/fonts%2Fv1.0.0/NotoSansSC-VariableFont_wght.woff2",
  },
  {
    id: "zh-TW",
    flag: "🇹🇼",
    localizedName: "繁體中文",
    fontFamily: "Noto Sans TC",
    fontFileName: "NotoSansTC-VariableFont_wght.woff2",
    fontDownloadUrl:
      "https://github.com/open-goal/launcher-assets/releases/download/fonts%2Fv1.0.0/NotoSansTC-VariableFont_wght.woff2",
  },
];

export async function initLocales() {
  AVAILABLE_LOCALES.forEach((locale) => {
    register(
      locale.id,
      () => import(`../../assets/translations/${locale.id}.json`),
    );
  });

  await init({
    fallbackLocale: "en-US",
    initialLocale: getLocaleFromNavigator(),
  });
}
