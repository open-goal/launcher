<script lang="ts">
  import OfficialVersions from "./versions/OfficialVersions.svelte";
  import { _ } from "svelte-i18n";
</script>

<div>
  <p class="text-sm mt-2 mb-2">{$_("settings_versions_header")}</p>
  <OfficialVersions />
</div>
