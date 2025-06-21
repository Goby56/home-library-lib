<script>
  import UserIcon from "@lucide/svelte/icons/user";
  import QuickActionMenu from "./QuickActionMenu.svelte";
  import CheckIcon from "@lucide/svelte/icons/check";
  import HouseIcon from "@lucide/svelte/icons/house";
  import Searchbar from "./Searchbar.svelte";
  import Button from "./ui/button/button.svelte";
  import SidebarToggleButton from "./SidebarToggleButton.svelte";
  import { Separator } from "$lib/components/ui/separator/index.js";
  import AppLogo from "$lib/assets/rose_icon.svelte";

  let { user } = $props();
</script>

<header class="z-50 flex w-full fixed h-16 items-center gap-2 bg-background p-1 px-2">
  <div class="w-full flex items-center justify-center md:justify-between gap-2">
    <div class="flex justify-start items-center gap-2 md:pl-2">
      <a href="/" class="flex md:hidden justify-center items-center rounded-md size-12 hover:bg-secondary">
        <HouseIcon/>
      </a>
      <a class="hidden md:flex wjustify-center items-center gap-3 text-3xl font-semibold" href="/">
        <AppLogo/>
        Rosenport
      </a>
    </div>
    <div class="flex justify-center w-full md:w-1/2">
      <Searchbar/>
    </div>
    <div class="flex justify-end items-center gap-2 md:pr-2">
      <a href="/profile" class="flex justify-center items-center rounded-md gap-2 p-1 size-fit hover:bg-secondary">
        <div class="flex justify-center items-center rounded-md size-10" style="background-color: #{user.personal_color}">
          <UserIcon/>
        </div>
        <div class="hidden md:flex flex-col items-start">
          <span>{user.username} </span> 
          <span class="flex items-center text-muted-foreground text-xs">Inloggad <CheckIcon size="12"/></span> 
        </div>
      </a>
      <Button class="hidden md:flex text-foreground" variant="link">Logga ut</Button>
    </div>
  </div>
</header>


