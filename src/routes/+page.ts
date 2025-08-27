import { redirect } from "@sveltejs/kit";
import { init, register } from "svelte-i18n";

export const ssr = false; // client-only

export const load = () => {
  register("en-US", () => import("$lib/translations/en-US.json"));
  init({ fallbackLocale: "en-US" });

  // load from settings.json
  let hasInstallDir = false;

  if (!hasInstallDir) {
    throw redirect(307, '/setup');
  }

  const last = localStorage.getItem("lastGame") as
    | "jak1"
    | "jak2"
    | "jak3"
    | null;
  const target = last ?? "jak1";
  throw redirect(307, `/game/${target}`);
};
