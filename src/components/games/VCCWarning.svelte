<script lang="ts">
  import { Alert } from "flowbite-svelte";
  import { _ } from "svelte-i18n";
</script>

<Alert rounded={false} class="border-t-4 text-red-400">
  <span class="font-bold">{$_("gameControls_warning_vccVersion_headerA")}</span>
  <em>{$_("gameControls_warning_vccVersion_headerB")}</em>
  <br />
  <ul>
    <li>
      <a
        class="font-bold text-blue-500"
        target="_blank"
        rel="noreferrer"
        href="https://aka.ms/vs/17/release/vc_redist.x64.exe"
        >{$_("requirements_windows_vccRuntimeExplanation_downloadLink")}</a
      >
    </li>
  </ul>
</Alert>
