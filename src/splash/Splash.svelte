<script lang="ts">
  import { onMount } from "svelte";
  import logo from "$assets/images/icon.webp";
  import {
    getInstallationDirectory,
    getLocale,
    setLocale,
  } from "$lib/rpc/config";
  import { locale as svelteLocale, _ } from "svelte-i18n";
  import SelectLocale from "./components/SelectLocale.svelte";
  import ChooseInstallFolder from "./components/ChooseInstallFolder.svelte";
  import LocaleQuickChanger from "./components/LocaleQuickChanger.svelte";
  import { openMainWindow } from "$lib/rpc/window";

  let loaded = false;
  let timeStartedAt = 0;
  let minimumTime = 500;
  let stepsToDo = [
    {
      statusText: "splash_step_loadingTranslations",
      func: async () => {
        await checkLocale();
      },
      waitingForInteraction: false,
    },
    {
      statusText: "splash_step_checkingDirectories",
      func: async () => {
        await checkDirectories();
      },
      waitingForInteraction: false,
    },
  ];
  let currentStepIndex = 0;
  let errorText = "";

  // Events
  onMount(async () => {
    // Ensure a default locale is set
    await svelteLocale.set("en-US");
    timeStartedAt = Date.now();
    stepsToDo.push({
      statusText: "splash_step_finishingUp",
      func: async () => {
        let currentTime = Date.now();
        if (currentTime - timeStartedAt < minimumTime) {
          await new Promise((res) =>
            setTimeout(res, minimumTime - (currentTime - timeStartedAt)),
          );
        }
        const errorClosing = await openMainWindow();
        if (!errorClosing) {
          errorText = $_("splash_step_errorOpening");
        }
      },
      waitingForInteraction: false,
    });
    loaded = true;
    await proceedInSteps(false, false);
  });

  async function proceedInSteps(stepForward: boolean, stepBackward: boolean) {
    if (stepForward) {
      currentStepIndex++;
      if (currentStepIndex >= stepsToDo.length) {
        currentStepIndex = stepsToDo.length - 1;
      }
    }
    if (stepBackward) {
      currentStepIndex--;
      if (currentStepIndex < 0) {
        currentStepIndex = 0;
      }
    }
    // Process as many steps as we can
    while (
      currentStepIndex < stepsToDo.length &&
      !stepsToDo[currentStepIndex].waitingForInteraction
    ) {
      await new Promise((res) => setTimeout(res, 125));
      await stepsToDo[currentStepIndex].func();
      currentStepIndex++;
    }
    if (currentStepIndex >= stepsToDo.length) {
      currentStepIndex = stepsToDo.length - 1;
    }
  }

  async function checkLocale() {
    const locale = await getLocale();
    if (locale === null) {
      // Prompt the user to select a locale
      stepsToDo.splice(currentStepIndex + 1, 0, {
        statusText: "splash_selectLocale",
        waitingForInteraction: true,
        func: async () => {},
      });
    } else {
      // Set locale and continue
      setLocale(locale);
    }
  }

  async function checkDirectories() {
    // Check to see if the install dir has been setup or not
    const install_dir = await getInstallationDirectory();
    if (install_dir === null) {
      // If not -- let's ask the user to set one up
      stepsToDo.splice(currentStepIndex + 1, 0, {
        statusText: "splash_noInstallDirSet",
        waitingForInteraction: true,
        func: async () => {},
      });
    }
  }

  async function handleLocaleChange(event: any, forStep: boolean) {
    await setLocale(event.detail.newLocale);
    if (forStep) {
      await proceedInSteps(true, false);
    }
  }
</script>

<div class="content" data-tauri-drag-region>
  {#if loaded}
    <div class="splash-logo pointer-events-none">
      <img
        src={logo}
        data-testId="splash-logo"
        alt="OpenGOAL logo"
        aria-label="OpenGOAL logo"
        draggable="false"
      />
    </div>
    <div class="splash-contents pointer-events-none">
      {#if errorText !== ""}
        <div class="splash-status-text">
          {errorText}
        </div>
      {:else if stepsToDo[currentStepIndex].statusText === "splash_selectLocale"}
        <div class="splash-status-text">
          {$_(stepsToDo[currentStepIndex].statusText)}
        </div>
        <SelectLocale on:change={(evt) => handleLocaleChange(evt, true)} />
      {:else if stepsToDo[currentStepIndex].statusText === "splash_noInstallDirSet"}
        <ChooseInstallFolder
          on:complete={async () => {
            await proceedInSteps(true, false);
          }}
        />
      {:else}
        <div class="splash-status-text">
          {$_(stepsToDo[currentStepIndex].statusText)}
        </div>
      {/if}
    </div>
    <div class="splash-bar">
      <div
        data-tauri-drag-region
        class="splash-status-bar fg"
        style="width: {((currentStepIndex + 1) / stepsToDo.length) * 100}%"
      ></div>
      <div data-tauri-drag-region class="splash-status-bar bg"></div>
    </div>
    {#if stepsToDo[currentStepIndex].statusText === "splash_noInstallDirSet"}
      <LocaleQuickChanger on:change={(evt) => handleLocaleChange(evt, false)} />
    {/if}
  {/if}
</div>

<style>
  .content {
    color: white;
    height: 100%;
    padding-top: 10px;
    padding-bottom: 10px;
  }

  .splash-contents {
    height: 35%;
    align-items: center;
    justify-content: center;
    font-family: "Twemoji Country Flags", "Noto Sans Mono", monospace;
    font-size: 10pt;
    text-align: center;
    padding-left: 10px;
    padding-right: 10px;
    display: flex;
    flex-direction: column;
  }

  .splash-bar {
    height: 10%;
  }

  .splash-logo {
    height: 50%;
  }

  .splash-logo img {
    object-fit: contain;
    height: 100%;
    width: 100%;
  }

  .splash-status-bar {
    width: 100%;
    height: 15px;
    margin-top: auto;
  }

  .splash-status-bar.bg {
    background-color: #775500;
    position: absolute;
  }

  .splash-status-bar.fg {
    background-color: #ffb807;
    position: absolute;
    z-index: 999;
  }
</style>
