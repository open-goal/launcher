<script lang="ts">
  import logoJak1 from "$assets/images/jak-tpl.webp";
  import logoJak2 from "$assets/images/jak-2.webp";
  import logoJak3 from "$assets/images/jak-3.webp";
  import { link } from "svelte-navigator";
  import { Tooltip } from "flowbite-svelte";
</script>

<div
  class="w-20 h-full bg-[#222222] px-1 absolute left-0 opacity-75 hover:opacity-100 duration-500"
>
  <ul class="flex flex-col space-y-6 relative px-1 py-5">
    <li class="relative">
      <a
        class="flex items-center grayscale hover:grayscale-0 hover:opacity-100 duration-500"
        href="/jak1"
        use:link
        data-mdb-ripple="true"
        data-mdb-ripple-color="dark"
      >
        <img src={logoJak1} alt="Jak - TPL" />
        <Tooltip
          arrow={false}
          placement="right"
          style="dark"
          tipClass="px-2 truncate">Jak and Daxter: The Precursor Legacy</Tooltip
        >
      </a>
    </li>
    <li class="relative">
      <a
        class="flex items-center grayscale hover:grayscale-0 hover:opacity-100 duration-500"
        href="/jak2"
        use:link
        data-mdb-ripple="true"
        data-mdb-ripple-color="dark"
        disabled
      >
        <img src={logoJak2} alt="Jak2" />
        <Tooltip
          arrow={false}
          placement="right"
          style="dark"
          tipClass="px-2 truncate">Jak 2</Tooltip
        >
      </a>
    </li>
    <li class="relative">
      <a
        class="flex items-center grayscale hover:grayscale-0 hover:opacity-100 duration-500"
        href="/jak3"
        use:link
        data-mdb-ripple="true"
        data-mdb-ripple-color="dark"
        disabled
      >
        <img src={logoJak3} alt="Jak3" />
        <Tooltip
          arrow={false}
          placement="right"
          style="dark"
          tipClass="px-2 truncate">Jak 3</Tooltip
        >
      </a>
    </li>
    <li class="relative">
      <a
        class="flex items-center justify-center opacity-75 grayscale hover:grayscale-0 hover:opacity-100 text-orange-400 duration-500"
        href="/settings"
        use:link
        data-mdb-ripple="true"
        data-mdb-ripple-color="dark"
      >
        <i class="fa-solid fa-gear text-3xl" />
        <Tooltip
          arrow={false}
          placement="right"
          style="dark"
          tipClass="px-2 truncate">Settings</Tooltip
        >
      </a>
    </li>
  </ul>
</div>
