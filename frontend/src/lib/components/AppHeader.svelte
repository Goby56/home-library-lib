<script>
  import User from "@lucide/svelte/icons/user";
  import QuickActionMenu from "./QuickActionMenu.svelte";
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import Searchbar from "./Searchbar.svelte";
  import Button from "./ui/button/button.svelte";
  import SidebarToggleButton from "./SidebarToggleButton.svelte";
</script>

<header class="flex w-full fixed h-16 items-center gap-2 bg-sidebar border-b p-2">
  <div class="w-full flex items-center justify-between">
    <div class="flex justify-start items-center gap-2 z-10">
      <SidebarToggleButton/>
      <h3 class="hidden md:flex scroll-m-20 text-2xl font-semibold tracking-tight">
        <a href="/">Rosenport</a>
      </h3>
    </div>
    <div class="flex justify-center w-full md:w-1/2">
      <Searchbar/>
    </div>
    <div class="flex justify-end items-center gap-2">
      <div class="hidden md:flex">
        <QuickActionMenu/>
      </div>
      <Button
      	type="button"
      	variant="ghost"
      	size="icon"
      ><User/></Button>
    </div>
  </div>
</header>


