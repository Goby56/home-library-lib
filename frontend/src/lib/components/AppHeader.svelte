<script>
  import User from "@lucide/svelte/icons/user";
  import QuickActionMenu from "./QuickActionMenu.svelte";
  import Searchbar from "./Searchbar.svelte";
  import Button from "./ui/button/button.svelte";
  import SidebarToggleButton from "./SidebarToggleButton.svelte";
  import AppLogo from "$lib/assets/rose_icon.svelte";
</script>

<header class="z-50 flex w-full fixed h-16 items-center gap-2 bg-popover border-b p-2">
  <div class="w-full flex items-center justify-between">
    <div class="flex justify-start items-center gap-2 z-10">
      <SidebarToggleButton/>
      <a class="hidden md:flex justify-center items-center gap-3 text-3xl font-semibold" href="/">
        <AppLogo/>
        Rosenport
      </a>
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


