import { Convert } from "./translation_schema";
import type { TranslationSchema } from "./translation_schema";
import english from "$assets/translations/english.json";

let supportedTranslations = ["english"];

export let TranslatedStrings: TranslationSchema;

export function loadTranslations(language: string) {
  if (!supportedTranslations.includes(language)) {
    console.log("Language not supported!");
  }
  // TODO - would prefer to import this by a raw path but have to import
  // for vite reasons -- maybe there is a different way?
  if (language === "english") {
    TranslatedStrings = Convert.toTranslationSchema(JSON.stringify(english));
  }
}
