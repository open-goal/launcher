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
// @ts-ignore
import Job from "./routes/Job.svelte";
import { versionState } from "./state/VersionState.svelte";

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
  "/update/launcher": UpdateLauncher,
  "/faq": Help,
  "/job/:job_type": Job,
  "/settings": {
    "/": Settings,
    "/:tab": Settings,
  },
  // has to go last because it can match some of the above
  "/:game_name": {
    "/": Game,
    "/mods": {
      "/": ModSelection,
      "/:source_name/:mod_name": {
        "/": Game,
        hooks: {
          beforeLoad() {
            versionState.displayModVersion = true;
          },
        },
      },
    },
    "/texture_packs": {
      "/": TexturePacks,
    },
  },
  layout: Layout,
});
