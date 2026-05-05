import { initLocales } from "$lib/i18n/i18n";
import { mount } from "svelte";
import App from "./App.svelte";
import { polyfillCountryFlagEmojis } from "country-flag-emoji-polyfill";
import { initConfig } from "./state/config.svelte";

export default (async () => {
  // Initialize config before anything else so that we can use it in the rest of the app
  await initConfig();
  polyfillCountryFlagEmojis();
  await initLocales();
  const elem = document.getElementById("app");
  if (elem) {
    return mount(App, {
      target: elem,
    });
  }
  console.error("Unable to locate #app element");
  return;
})();
