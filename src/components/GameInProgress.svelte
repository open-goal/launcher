<script lang="ts">
  import { Alert, Button } from "flowbite-svelte";
  import { _ } from "svelte-i18n";
</script>

<div class="flex flex-col justify-center p-4">
  <Alert>
    <div class="flex items-center gap-3 justify-center">
      <span class="text-lg font-medium">{$_("temp_jak3_indev_header")}</span>
    </div>
    <p class="mt-2 mb-4 text-sm">
      {$_("temp_jak2_indev_subheader")}
    </p>
    <div class="flex gap-2 justify-center">
      <Button
        href="https://opengoal.dev/blog/"
        target="_blank"
        size="xs"
        outline>{$_("temp_jak2_indev_progressReports")}</Button
      >
    </div>
  </Alert>
</div>
