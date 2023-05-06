import { initLocales } from "$lib/i18n/i18n";
import App from "./Splash.svelte";
import { polyfillCountryFlagEmojis } from "country-flag-emoji-polyfill";

polyfillCountryFlagEmojis();

// Register Translations
await initLocales(false);

const app = new App({
  target: document.getElementById("app"),
});

export default app;
