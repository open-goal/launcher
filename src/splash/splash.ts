import { initLocales } from "$lib/i18n/i18n";
import App from "./Splash.svelte";
import { polyfillCountryFlagEmojis } from "country-flag-emoji-polyfill";
import "../app.postcss";

// Register Translations
export default (async () => {
  polyfillCountryFlagEmojis();
  await initLocales(false);
  return new App({
    target: document.getElementById("app"),
  });
})();
