<script>
  import { ProcessLogs } from "$lib/stores/AppStore";
  import { AccordionItem } from "flowbite-svelte";
</script>

<AccordionItem
  class="bg-[#222222] border-[#222222] !rounded-none"
  slotClass="py-0 border-[#222222]"
  flush
  id="1"
>
  <h2 slot="header" class="px-4 text-white text-xl">Installation Logs</h2>
  <div
    slot="body"
    class="bg-[#222222] px-4 max-h-64 overflow-y-scroll scrollbar"
  >
    <p class="py-4 text-clip overflow-hidden">
      {$ProcessLogs}
    </p>
  </div>
</AccordionItem>

<style>
  @tailwind base;
  @tailwind components;
  @tailwind utilities;

  @layer utilities {
    .scrollbar::-webkit-scrollbar {
      width: 20px;
      height: 20px;
    }

    .scrollbar::-webkit-scrollbar-track {
      background: #222222;
    }

    .scrollbar::-webkit-scrollbar-thumb {
      background: #e0cbcb;
      border: 3px solid #222222;
    }

    .scrollbar::-webkit-scrollbar-thumb:hover {
      background: #c0a0b9;
    }
  }
</style>
