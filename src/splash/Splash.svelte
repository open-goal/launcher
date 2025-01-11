<script lang="ts">
  import { onMount } from "svelte";
  import jak19am from "$assets/images/jak1/9am.webp";
  import jak29am from "$assets/images/jak2/9am.webp";
  import textLogo from "$assets/images/text-logo.webp";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { type Locale } from "$lib/i18n/i18n";
  import { setLocale } from "$lib/rpc/config";
  import { getVersion } from "@tauri-apps/api/app";
  import LanguageSelect from "/src/splash/components/LanguageSelect.svelte";
  import ProgressStepper from "/src/splash/components/ProgressStepper.svelte";
  import { type } from "@tauri-apps/plugin-os";
  import { _ } from "svelte-i18n"; 

  let loaded = false;
  let clientVersion: string;
  let isWindowControlsLeft = false;

  onMount(async () => {
    clientVersion = await getVersion();
    isWindowControlsLeft = (await type()) === "macos";
    const a = getCurrentWindow();
    console.log(a);

    loaded = true;
  });

  async function selectLocale(locale: Locale) {
    await setLocale(locale.id);
  }
</script>

<div class="header" data-tauri-drag-region>
  <img class="h-full" src={textLogo} />
</div>

<div class="splash-container">
  <!-- Background With two images -->
  <div class="images pointer-events-none">
    <div class="gradient-overlay"></div>
    <img id="jak1" src={jak19am} />
    <img id="jak2" src={jak29am} />
  </div>

  {#if loaded}
    <div class="splash-content">
      <div class="splash-content__centered">
        <ProgressStepper />

        <LanguageSelect
          on:change={(locale) => selectLocale(locale.detail.locale)}
        />

        <div class="self-start text-background font-default-shadow">{$_("header_launcherVersionLabel")} v{clientVersion}</div>
      </div>
    </div>
  {/if}
</div>

<style lang="postcss">
  @import "./splash.css";

  .header {
    @apply h-16 p-4 flex flex-row justify-center items-center bg-background text-background;
  }

  .header > :not(:last-child) {
    @apply pointer-events-none select-none;
  }

  .header .window-controls {
    @apply flex flex-row gap-3 text-background text-2xl cursor-pointer justify-center items-center;
  }

  .header .window-controls .control-icon {
    @apply hover:text-background-hover;
  }

  .splash-content {
    @apply absolute inset-0 z-20;
  }

  .splash-container {
    @apply relative flex-grow text-background;
  }

  .splash-container .images {
    @apply relative flex w-full h-full ;
  }

  .splash-container .images img#jak1 {
    transition: all 0.3s ease;
    @apply absolute top-0 left-0 h-full w-[55%];
    clip-path: polygon(0% 0%, 99.5% 0%, 81.5% 100%, 0% 100%);
  }

  .splash-container .images img#jak2 {
    transition: all 0.3s ease;
    @apply absolute top-0 right-0 h-full w-[55%];
    clip-path: polygon(18.5% 0%, 100% 0%, 100% 100%, 0.5% 100%);
  }

  .splash-container .images .gradient-overlay {
    @apply absolute left-0 top-0 w-full h-full z-10;
    background-image: radial-gradient(
      circle at center,
      rgb(27, 27, 29, 1) 0%,
      rgb(27, 27, 29, 0.75) 30%,
      rgba(27, 27, 29, 0.15) 60%
    );
  }

  .splash-container .images img {
    object-fit: cover;
    transition: opacity 0.5s ease;
    opacity: 1;
  }

  .splash-content__centered {
    @apply flex flex-col justify-center items-center;
    @apply h-full p-2 px-4 pt-16 gap-5;
  }
</style>
