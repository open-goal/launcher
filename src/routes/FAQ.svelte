<script>
  import { Button } from "flowbite-svelte";
  import Icon from "@iconify/svelte";
</script>

<div class="ml-20">
  <!-- TODO - the static height here is kinda a hack, it's because the
    header and the rest of the layout aren't within a shared container -->
  <div class="flex flex-col h-[544px] bg-slate-900 p-4 gap-3">
    <h1 class="font-semibold text-xl text-orange-500">Support & FAQ</h1>
    <p class="text-sm">
      If you are reporting an issue or asking for help, download the following
      support package and attach it in your Discord thread or GitHub issue.
    </p>
    <div class="flex flex-row mt-1">
      <Button
        btnClass="border-solid rounded bg-orange-400 hover:bg-orange-600 text-sm text-slate-900 font-semibold px-5 py-2"
        href="https://github.com/open-goal/launcher/issues/new"
        target="_blank"
        rel="noreferrer noopener">Download Support Package</Button
      >
    </div>
    <p class="mt-3 text-sm">
      You can either ask a question on our Discord, or create a GitHub issue
      with as much detail as possible.
    </p>
    <p class="text-sm">
      In either location, please do a quick search to see if the question has
      already been answered before
    </p>
    <div class="flex flex-row gap-2">
      <Button
        btnClass="flex items-center border-solid rounded bg-white hover:bg-orange-400 text-sm text-slate-900 font-semibold px-5 py-2"
        href="https://discord.gg/VZbXMHXzWv"
        target="_blank"
        rel="noreferrer noopener"
        ><Icon
          class="inline-block"
          icon="ic:baseline-discord"
          width={20}
        />&nbsp;Discord</Button
      >
      <Button
        btnClass="flex items-center border-solid rounded bg-white hover:bg-orange-400 text-sm text-slate-900 font-semibold px-5 py-2"
        href="https://github.com/open-goal/launcher/issues/new"
        target="_blank"
        rel="noreferrer noopener"
        ><Icon class="inline-block" icon="mdi:github" width={20} />&nbsp;Report
        Launcher Issue</Button
      >
      <Button
        btnClass="flex items-center border-solid rounded bg-white hover:bg-orange-400 text-sm text-slate-900 font-semibold px-5 py-2"
        href="https://github.com/open-goal/jak-project/issues/new/choose"
        target="_blank"
        rel="noreferrer noopener"
        ><Icon class="inline-block" icon="mdi:github" width={20} />&nbsp;Report
        Game Issue</Button
      >
    </div>
  </div>
</div>
