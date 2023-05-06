import { addMessages, init, register } from "svelte-i18n";

interface Locale {
  id: string;
  flag: string;
  localizedName: string;
}

export const AVAILABLE_LOCALES: Locale[] = [
  {
    id: "en-US",
    flag: "🇺🇸",
    localizedName: "English",
  },
];

export async function initLocales(async: boolean) {
  if (async) {
    for (const locale of AVAILABLE_LOCALES) {
      register(
        locale.id,
        () => import(`../../assets/translations/${locale.id}.json`)
      );
    }
  } else {
    for (const locale of AVAILABLE_LOCALES) {
      addMessages(
        locale.id,
        await import(`../../assets/translations/${locale.id}.json`)
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
