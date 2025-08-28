<script lang="ts">
  import type { PageProps } from "./$types";
  import { locale as svelteLocale, _ } from "svelte-i18n";
  import LocaleSelector from "../../splash/components/LocaleSelector.svelte";
  import ChooseInstallFolder from "../../splash/components/ChooseInstallFolder.svelte";
  import { Button } from "flowbite-svelte";
  // import { polyfillCountryFlagEmojis } from "country-flag-emoji-polyfill";
  import "./splash.postcss";

  let { data }: PageProps = $props();
</script>

<div
  class="flex flex-col justify-center w-screen h-screen bg-gray-900 px-40 gap-8"
>
  <LocaleSelector />
  <ChooseInstallFolder />
  <Button>Continue</Button>
</div>
