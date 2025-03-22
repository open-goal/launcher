import { initLocales } from "$lib/i18n/i18n";
import "./app.postcss";
import { mount } from 'svelte';
import App from "./App.svelte";
import { polyfillCountryFlagEmojis } from "country-flag-emoji-polyfill";

polyfillCountryFlagEmojis();

// Register Translations
initLocales(true);

const app = mount(App, {target: document.getElementById("app")});

export default app;
