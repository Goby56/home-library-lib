<script>
  import User from "@lucide/svelte/icons/user";
  import QuickActionMenu from "./QuickActionMenu.svelte";
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import Searchbar from "./Searchbar.svelte";
</script>

<header class="flex w-full fixed h-16 items-center gap-2 bg-sidebar border-b px-4">
  <div class="w-full px-4 py-3 flex items-center justify-between">
    <div class="flex justify-center items-center gap-3 z-10">
	    <Sidebar.Trigger/>
      <h3 class="hidden md:flex scroll-m-20 text-2xl font-semibold tracking-tight">
        <a href="/">Rosenport</a>
      </h3>
    </div>
    <div class="flex justify-center w-full md:w-1/2">
      <Searchbar/>
    </div>
    <div class="flex justify-center items-center gap-8">
      <div class="hidden md:flex">
        <QuickActionMenu/>
      </div>
      <User/>
    </div>
  </div>
</header>

