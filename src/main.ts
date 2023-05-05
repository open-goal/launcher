import { init, register } from "svelte-i18n";
import "./app.postcss";
import App from "./App.svelte";

// Register Translations
register("en-US", () => import("$assets/translations/en-US.json"));
register("fr-FR", () => import("$assets/translations/fr-FR.json"));
init({
  fallbackLocale: "en-US",
  initialLocale: "en-US", // TODO - get this from the config
});

const app = new App({
  target: document.getElementById("app"),
});

export default app;
