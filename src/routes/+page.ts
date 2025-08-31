import type { SupportedGame } from "$lib/rpc/bindings/SupportedGame.js";
import { redirect } from "@sveltejs/kit";

export const ssr = false; // client-only

export const load = async ({ parent }) => {
  const { config } = await parent();
  const hasInstallDir = config.installationDir;
  const testing = false; // TODO: remove this after im done developing splash
  if (!hasInstallDir || testing) {
    throw redirect(307, "/setup");
  }

  const last = localStorage.getItem("lastGame") as SupportedGame;
  const target = last ?? "jak1";
  throw redirect(307, `/${target}`);
};
