<script>
  import UserIcon from "@lucide/svelte/icons/user";
  import CheckIcon from "@lucide/svelte/icons/check";
  import HouseIcon from "@lucide/svelte/icons/house";
  import Searchbar from "./Searchbar.svelte";
  import Button from "./ui/button/button.svelte";
  import { Separator } from "$lib/components/ui/separator/index.js";
  import AppLogo from "$lib/assets/rose_icon.svelte";
  import DarkModeToggle from "$lib/components/DarkModeToggle.svelte";
  import { MediaQuery } from "svelte/reactivity";
  import { mode } from "mode-watcher";

  let { user } = $props();

  let searchInput = $state("");
  const isDesktop = new MediaQuery("(min-width: 768px)");
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
      <Searchbar bind:value={searchInput}/>
    </div>
    <div class="flex justify-end items-center h-8 gap-2 md:pr-2">
      {#if isDesktop.current}
        <DarkModeToggle/>
      {/if}
      <Separator class="hidden md:contents" orientation="vertical" />
      <a href="/profile" class="flex justify-center items-center rounded-md gap-2 p-1 size-fit hover:bg-secondary">
        <div class="flex justify-center items-center rounded-md size-10" style="background-color: #{user.personal_color}">
          <UserIcon class={mode.current == "light" ? "text-background" : "text-foreground"}/>
        </div>
        <div class="hidden md:flex flex-col items-start">
          <span>{user.username} </span> 
          <span class="flex items-center text-muted-foreground text-xs">Inloggad <CheckIcon size="12"/></span> 
        </div>
      </a>
      <Separator class="hidden md:contents" orientation="vertical" />
      <form method="POST" action="/api/logout">
        <Button type="submit" class="hidden md:flex text-foreground p-1" variant="link">Logga ut</Button>
      </form>
    </div>
  </div>
</header>


