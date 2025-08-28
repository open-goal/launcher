import { getInstallationDirectory } from "$lib/rpc/config.js";
import { redirect } from "@sveltejs/kit";

export const ssr = false; // client-only

export const load = () => {
  let hasInstallDir = getInstallationDirectory();
  let testing = true; // TODO: remove this after im done developing splash
  if (!hasInstallDir || testing) {
    throw redirect(307, "/setup");
  }

  const last = localStorage.getItem("lastGame") as
    | "jak1"
    | "jak2"
    | "jak3"
    | null;
  const target = last ?? "jak1";
  throw redirect(307, `/game/${target}`);
};
