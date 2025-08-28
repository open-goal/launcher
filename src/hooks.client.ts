import type { ServerInit } from "@sveltejs/kit";
import { initLocales } from "$lib/i18n/i18n";

export const init: ServerInit = async () => {
  await initLocales();
};
