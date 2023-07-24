import { addMessages, init, register } from "svelte-i18n";

interface Locale {
  id: string;
  flag: string;
  localizedName: string;
  fontFamily?: string;
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
  },
  {
    id: "ko-KR",
    flag: "🇰🇷",
    localizedName: "한국어",
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
  },
  {
    id: "zh-TW",
    flag: "🇹🇼",
    localizedName: "繁體中文",
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
  }
}
