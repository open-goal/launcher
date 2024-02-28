<script lang="ts">
  import { type ProgressStatus } from "$lib/stores/ProgressStore";
  import IconCheck from "~icons/mdi/check";
  import IconDots from "~icons/mdi/dots-horizontal";
  import IconAlert from "~icons/mdi/stop-alert";
  import IconHourglass from "~icons/mdi/hourglass";

  export let status: ProgressStatus = "pending";
</script>

{#if status === "success"}
  <IconCheck color="#22c55e" />
{:else if status === "pending"}
  <IconDots class="animate-pulse" color="#facc15" />
{:else if status === "failed"}
  <IconAlert color="#ef4444" />
{:else}
  <IconHourglass color="#737373" />
{/if}
