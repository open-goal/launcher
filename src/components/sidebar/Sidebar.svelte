<script lang="ts">
  import logoJak1 from "$assets/images/jak-tpl.webp";
  import logoJak2 from "$assets/images/jak-2.webp";
  // import logoJak3 from "$assets/images/jak-3.webp";
  import { link, useLocation } from "svelte-navigator";
  import { Tooltip } from "flowbite-svelte";

  const location = useLocation();
  $: pathname = $location.pathname;
</script>

<div
  class="w-20 h-full bg-[#101010] px-1 absolute left-0 {pathname.startsWith(
    '/settings'
  )
    ? ''
    : 'opacity-75 hover:opacity-100 duration-500 '} z-10"
>
  <ul class="flex flex-col space-y-12 px-1 py-5">
    <li>
      <a
        class="flex items-center {pathname == '/jak1'
          ? ''
          : 'grayscale'} hover:grayscale-0 hover:opacity-100 duration-500"
        href="/jak1"
        use:link
        data-mdb-ripple="true"
        data-mdb-ripple-color="dark"
      >
        <img src={logoJak1} alt="Jak - The Precursor Legacy" />
        <Tooltip placement="right"
          >Jak&nbsp;and&nbsp;Daxter:&nbsp;The&nbsp;Precursor&nbsp;Legacy</Tooltip
        >
      </a>
    </li>
    <li>
      <a
        class="flex items-center {pathname == '/jak2'
          ? ''
          : 'grayscale'} hover:grayscale-0 hover:opacity-100 duration-500"
        href="/jak2"
        use:link
        data-mdb-ripple="true"
        data-mdb-ripple-color="dark"
      >
        <img src={logoJak2} alt="Jak 2" />
        <Tooltip placement="right" style="dark">Jak 2</Tooltip>
      </a>
    </li>
    <!--
    <li>
      <a
        class="flex items-center {pathname == '/jak3'
          ? ''
          : 'grayscale'} hover:grayscale-0 hover:opacity-100 duration-500"
        href="/jak3"
        use:link
        data-mdb-ripple="true"
        data-mdb-ripple-color="dark"
        disabled
      >
        <img src={logoJak3} alt="Jak3" />
        <Tooltip
          rounded={false}
          arrow={false}
          placement="right"
          style="dark"
          tipClass="px-2 truncate">Jak 3</Tooltip
        >
      </a>
    </li> -->
    <li class="fixed bottom-24 left-6">
      <a
        class="flex items-center justify-center opacity-75 {pathname.startsWith(
          '/settings'
        )
          ? 'opacity-100'
          : 'grayscale'} hover:grayscale-0 hover:opacity-100 text-orange-400 duration-500"
        href="/settings/general"
        use:link
        data-mdb-ripple="true"
        data-mdb-ripple-color="dark"
      >
        <i class="fa-solid fa-gear text-3xl" />
        <Tooltip rounded={false} arrow={false} placement="right" style="dark"
          >Settings</Tooltip
        >
      </a>
    </li>

    <li class="fixed bottom-5 left-6">
      <a
        class="flex items-center justify-center opacity-75 {pathname == '/faq'
          ? 'opacity-100'
          : 'grayscale'} hover:grayscale-0 hover:opacity-100 text-orange-400 duration-500"
        href="/faq"
        use:link
        data-mdb-ripple="true"
        data-mdb-ripple-color="dark"
      >
        <i class="fa fa-question-circle text-3xl" />
        <Tooltip rounded={false} arrow={false} placement="right" style="dark"
          >FAQ</Tooltip
        >
      </a>
    </li>
  </ul>
</div>
