import { createRouter } from "sv-router";
// @ts-ignore
import Game from "./routes/Game.svelte";
// @ts-ignore
import Settings from "./routes/Settings.svelte";
// @ts-ignore
import Help from "./routes/Help.svelte";
// @ts-ignore
import ModSelection from "./components/games/features/mods/ModSelection.svelte";
// @ts-ignore
import TexturePacks from "./components/games/features/texture-packs/TexturePacks.svelte";
// @ts-ignore
import UpdateLauncher from "./routes/UpdateLauncher.svelte";
// @ts-ignore
import Layout from "./routes/layouts/Layout.svelte";

export const { p, navigate, isActive, route } = createRouter({
  "/": Game,
  hooks: {
    // simplify things -- stop having to worry about treating '/' differently
    beforeLoad({ pathname }) {
      if (pathname === "/") {
        throw navigate("/jak1");
      }
    },
  },
  "/:game_name": {
    "/": Game,
    "/mods": {
      "/": ModSelection,
      "/:source_name/:mod_name": {
        "/": Game,
      },
    },
    "/texture_packs": {
      "/": TexturePacks,
    },
  },
  "/settings": {
    "/": Settings,
    "/:tab": Settings,
  },
  "/faq": Help,
  "/update/launcher": UpdateLauncher,
  layout: Layout,
});
