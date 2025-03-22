import { initLocales } from "$lib/i18n/i18n";
import { mount } from 'svelte';
import App from "./Splash.svelte";
import { polyfillCountryFlagEmojis } from "country-flag-emoji-polyfill";
import "./splash.postcss";

// Register Translations
export default (async () => {
  polyfillCountryFlagEmojis();
  await initLocales(false);
  return mount(App, {
    target: document.getElementById("app"),
  });
})();
