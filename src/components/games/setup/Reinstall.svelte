<script>
  import { Alert } from "flowbite-svelte";
</script>

<Alert dismissable color="red" rounded={false} accent>
  <span class="font-medium text-2xl">Notice:</span>
  <ul slot="extra" class="mt-0 ml-8 list-disc list-inside text-lg">
    <li>Game installed with a previous version of OpenGOAL</li>
    <li>The game must be updated before you can proceed</li>
    <li>Save files and settings will not be modified</li>
  </ul>
</Alert>
