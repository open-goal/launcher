import { initLocales } from "$lib/i18n/i18n";
import "./app.postcss";
import { mount } from "svelte";
import App from "./App.svelte";
import { polyfillCountryFlagEmojis } from "country-flag-emoji-polyfill";

export default (async () => {
  polyfillCountryFlagEmojis();
  await initLocales(false);
  const elem = document.getElementById("app");
  if (elem) {
    return mount(App, {
      target: elem,
    });
  }
  console.error("Unable to locate #app element");
  return;
})();
