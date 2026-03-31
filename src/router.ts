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
import Layout from "./layouts/Layout.svelte";
// @ts-ignore
import Job from "./routes/Job.svelte";
// @ts-ignore
import GameToolsNotSet from "./components/games/GameToolsNotSet.svelte";
// @ts-ignore
import GameNotSupportedByTooling from "./components/games/GameNotSupportedByTooling.svelte";
// @ts-ignore
import GameSetup from "./components/job/GameSetup.svelte";
// @ts-ignore
import GameUpdate from "./components/job/GameUpdate.svelte";
import { versionState } from "./state/VersionState.svelte";
import {
  doesActiveToolingVersionSupportGame,
  getInstalledVersion,
  isGameInstalled,
} from "$lib/rpc/config";
import {
  ensureActiveVersionStillExists,
  getActiveVersion,
} from "$lib/rpc/versions";
import { toSupportedGame } from "$lib/rpc/bindings/utils/SupportedGame";
import Requirements from "./components/job/Requirements.svelte";
import { requirementsStore } from "./state/requirements-store";

export const { p, navigate, isActive, route } = createRouter({
  "/": Game,
  hooks: {
    // simplify things -- stop having to worry about treating '/' differently
    beforeLoad({ pathname }) {
      if (pathname === "/") {
        // simple redirect to default game
        throw navigate("/:game_name/", { params: { game_name: "jak1" } });
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
    "/": {
      "/": Game,
      hooks: {
        async beforeLoad(context) {
          const { params } = context as any;
          if (!params || !params.game_name) {
            return;
          }

          const activeGame = toSupportedGame(params.game_name);
          if (!activeGame) {
            return;
          }

          versionState.activeToolingVersion = await getActiveVersion();
          if (!versionState.activeToolingVersion) {
            throw navigate("/:game_name/tools-not-set", {
              params: { game_name: params.game_name },
            });
          }

          const currentRequirements =
            await requirementsStore.refresh(activeGame);
          if (!currentRequirements.requirementsMet) {
            throw navigate("/:game_name/requirements", {
              params: { game_name: params.game_name },
            });
          }

          const activeVersionExists = await ensureActiveVersionStillExists();
          if (activeVersionExists) {
            const supported =
              await doesActiveToolingVersionSupportGame(activeGame);
            if (!supported) {
              throw navigate("/:game_name/not-supported", {
                params: { game_name: params.game_name },
              });
            }
          }

          const installed = await isGameInstalled(activeGame);
          if (!installed) {
            throw navigate("/:game_name/setup", {
              params: { game_name: params.game_name },
            });
          }

          const installedVersion = await getInstalledVersion(activeGame);
          if (installedVersion !== versionState.activeToolingVersion) {
            throw navigate("/:game_name/update", {
              params: { game_name: params.game_name },
            });
          }
        },
      },
    },
    "/tools-not-set": GameToolsNotSet,
    "/not-supported": GameNotSupportedByTooling,
    "/setup": GameSetup,
    "/update": GameUpdate,
    "/requirements": Requirements,
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
