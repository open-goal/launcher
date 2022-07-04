import { loadTranslations } from "$lib/translations/translations";
import App from "./App.svelte";

loadTranslations("english");

const app = new App({
  target: document.getElementById("app"),
});

export default app;
