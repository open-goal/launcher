<script>
  import { progressTracker } from "$lib/stores/ProgressStore";
  import Icon from "@iconify/svelte";
  import { Accordion, AccordionItem } from "flowbite-svelte";
  import { ansiSpan } from "ansi-to-span";
  import escapeHtml from "escape-html";
  import { _ } from "svelte-i18n";

  function convertLogColors(text) {
    return ansiSpan(escapeHtml(text)).replaceAll("\n", "<br/>");
  }
</script>

<Accordion class="log-accordian" defaultClass="p-0">
  <AccordionItem class="bg-slate-900 rounded p-[1rem]">
    <span slot="header" class="text-sm font-semibold text-white flex gap-2">
      <Icon icon="mdi:file-document-outline" width={18} />
      <span>{$_("setup_logs_header")}</span>
    </span>
    <div
      slot="default"
      class="bg-slate-900 px-4 max-h-60 overflow-y-scroll scrollbar"
    >
      <p class="py-4 text-clip overflow-hidden font-mono log-output">
        ...{$_("setup_logs_truncation")}:
        <br />
        {@html convertLogColors($progressTracker.logs)}
      </p>
    </div>
  </AccordionItem>
</Accordion>
