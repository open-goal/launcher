<script>
  import { progressTracker } from "$lib/stores/ProgressStore";
  import Icon from "@iconify/svelte";
  import { Accordion, AccordionItem } from "flowbite-svelte";
</script>

<Accordion class="log-accordian" defaultClass="p-0">
  <AccordionItem class="bg-slate-900 rounded p-[1rem]">
    <span slot="header" class="text-sm font-semibold text-white flex gap-2">
      <Icon icon="mdi:file-document-outline" width={18} />
      <span>Logs</span>
    </span>
    <div
      slot="default"
      class="bg-slate-900 px-4 max-h-60 overflow-y-scroll scrollbar"
    >
      <p class="py-4 text-clip overflow-hidden font-mono">
        {$progressTracker.logs}
      </p>
    </div>
  </AccordionItem>
</Accordion>
