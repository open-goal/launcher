import { initLocales } from "$lib/i18n/i18n";
import "./app.postcss";
import App from "./App.svelte";
import { polyfillCountryFlagEmojis } from "country-flag-emoji-polyfill";

polyfillCountryFlagEmojis();

// Register Translations
initLocales(true);

const app = new App({
  target: document.getElementById("app"),
});

export default app;
