<script lang="ts">
  import { _ } from "svelte-i18n";
</script>

<div class="w-full px-56 my-5">
  <h4 class="sr-only">Status</h4>
  <h1 class="text-2xl font-medium text-background stepper">Setup OpenGOAL</h1>
  <div class="mt-6" aria-hidden="true">
    <div class="overflow-hidden rounded-full bg-background">
      <div class="h-3 rounded-full bg-primary" style="width: 12.5%"></div>
    </div>
    <div class="mt-6  grid-cols-3 hidden text-lg font-medium text-gray-600 sm:grid">
      <div class="stepper text-secondary">{$_("splash_selectLocale")}</div>
      <div class="text-center stepper text-background">{$_("splash_button_setInstallFolder")}</div>
      <div class="text-right stepper text-background">{$_("setup_done")}</div>
    </div>
  </div>
</div>

<style lang="postcss">
  .stepper {
    text-shadow: 1px 1px 2px black;
  }
</style>