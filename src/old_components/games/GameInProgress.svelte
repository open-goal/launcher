<script lang="ts">
  import { _ } from "svelte-i18n";
</script>

<div class="flex flex-col h-full justify-center items-center p-5 text-center">
  <h1 class="text-2xl font-black mb-5 text-outline">
    {$_("temp_jak3_indev_header")}
  </h1>
  <p class="mb-10">
    {$_("temp_jak2_indev_subheader")}
  </p>
  <a
    class="font-bold text-orange-400 hover:text-orange-600"
    href="https://opengoal.dev/blog/tags/progress-report"
    target="_blank"
    rel="noreferrer">{$_("temp_jak2_indev_progressReports")}</a
  >
</div>
