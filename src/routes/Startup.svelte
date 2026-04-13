<script>
  import { onMount } from "svelte";
  import {
    getInstallationDirectory,
    setInstallationDirectory,
    setLocale,
  } from "$lib/rpc/config";
  import { locale as svelteLocale, _ } from "svelte-i18n";
  import LocaleSelector from "../components/startup/LocaleSelector.svelte";
  import ChooseInstallFolder from "../components/startup/ChooseInstallFolder.svelte";
  import { navigate } from "../router";
  import { blockNavigation } from "sv-router";
  import { initLocales } from "$lib/i18n/i18n";

  let locale = $state("");
  let installDir = $state("");
  let navigationBlocked = $state(true);

  onMount(async () => {
    await svelteLocale.set("en-US");
    await initLocales(false);
    installDir = (await getInstallationDirectory()) || "";
  });

  $effect(() => {
    blockNavigation(() => !navigationBlocked);
  });

  $effect(() => {
    if (!locale) return;
    svelteLocale.set(locale);
    setLocale(locale);
  });

  $effect(() => {
    if (!installDir) return;
    setInstallationDirectory(installDir);
  });

  $effect(() => {
    if (locale && installDir) {
      navigationBlocked = false;
      navigate("/:game_name/", { params: { game_name: "jak1" } });
    }
  });
</script>

<div
  class="flex flex-col items-center justify-center h-full bg-[#141414] gap-4"
>
  <LocaleSelector bind:locale />

  {#if !installDir}
    <ChooseInstallFolder bind:installDir />
  {/if}
</div>
