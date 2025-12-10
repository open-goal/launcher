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

export const { p, navigate, isActive, route } = createRouter({
  "/": Game,
  "/:game_name": Game,
  "/:game_name/mods": ModSelection,
  "/:game_name/mods/:source_name/:mod_name": Game,
  "/:game_name/texture_packs": TexturePacks,
  "/settings": Settings,
  "/settings/:tab": Settings,
  "/faq": Help,
  "/update/launcher": UpdateLauncher,
});
