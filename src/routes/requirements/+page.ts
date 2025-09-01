import { arch, type } from "@tauri-apps/plugin-os";
import type { PageLoad } from "./$types";
import { isMacOSVersion15OrAbove } from "$lib/rpc/util";

export const load = (async ({ url }) => {
  // im not totally convinced by this pattern, but it works for now
  const game = url.searchParams.get("game");
  const avx = url.searchParams.get("avx");
  const openGL = url.searchParams.get("openGL");
  const disk = url.searchParams.get("disk");
  const vcc = url.searchParams.get("vcc");

  const ARMOutsideMac = arch() == "aarch64" && type() !== "macos";
  const isMacOSMin = await isMacOSVersion15OrAbove();
  return { game, avx, openGL, disk, vcc, ARMOutsideMac, isMacOSMin };
}) satisfies PageLoad;
