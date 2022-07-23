import { Convert } from "./translation_schema";
import type { TranslationSchema } from "./translation_schema";
import english from "$assets/translations/english.json";
import { log } from "$lib/utils/log";

let supportedTranslations = ["english"];

export let TranslatedStrings: TranslationSchema;

export function loadTranslations(language: string) {
  if (!supportedTranslations.includes(language)) {
    log.error("Language not supported!", {
      language: language,
    });
  }
  // TODO - would prefer to import this by a raw path but have to import
  // for vite reasons -- maybe there is a different way to ensure they are bundled?
  if (language === "english") {
    TranslatedStrings = Convert.toTranslationSchema(JSON.stringify(english));
  }
}
