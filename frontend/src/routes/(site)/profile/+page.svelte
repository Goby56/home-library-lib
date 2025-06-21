<script lang="ts">
	import type { PageProps } from './$types';
  import { invalidateAll } from '$app/navigation';
  import axios from "axios";
  import EditIcon from "@lucide/svelte/icons/pencil";
  import CheckIcon from "@lucide/svelte/icons/check";
  import Button from '$lib/components/ui/button/button.svelte';
  import { MediaQuery } from 'svelte/reactivity';
  import Input from '$lib/components/ui/input/input.svelte';
  import DarkModeToggle from '$lib/components/DarkModeToggle.svelte';
 
	let { data }: PageProps = $props();
  
  const initialColor = `#${data.user.personal_color}`;
  let selectedHex = $state(initialColor);

  let hasChangedColor = $derived(selectedHex != initialColor);

  let usernameInput = $state(data.user.username);

  const isDesktop = new MediaQuery("(min-width: 768px)");

</script>

<div class="flex gap-5">
  <div class="flex flex-col items-center size-24 md:size-32 rounded-3xl relative group" style="background-color: {selectedHex}">
    <input type="color" class="opacity-0 aspect-square size-full" bind:value={selectedHex}>
    <EditIcon class="group-hover:opacity-100 opacity-0 pointer-events-none size-12 absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2"/>
    {#if hasChangedColor}
      <Button class="text-muted-foreground p-0 pt-1 h-fit" variant="link">Byt till vald färg</Button>
    {/if}
  </div>
  <div class="flex flex-col gap-2 items-start">
    <h1 class="scroll-m-20 text-4xl font-extrabold tracking-tight lg:text-5xl">
      {data.user.username}
    </h1>
    <p class="text-muted-foreground text-sm hidden md:flex">Här kan du ändra din personliga färg och inloggningsuppgifter</p>
    <form method="POST" action="/api/logout">
      <Button type="submit" class="text-foreground" variant="secondary">Logga ut</Button>
    </form>
  </div>
</div>
<h2 class="scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight transition-colors mt-5 mb-2">
  Reservationer
</h2>
<div class="flex">
  {#each data.user_reservations as reservation}
    {reservation.start_date} 
  {/each} 
</div>
<h2 class="scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight transition-colors mt-5 mb-2">
  Inställningar
</h2>
<div class="flex flex-col gap-3 items-start">
  <div class="flex gap-2 items-center">
    <p class="text-muted-foreground">Använd mörkt läge: </p>
    <DarkModeToggle/>
  </div>
  <div class="flex gap-2 items-center">
    <p class="text-muted-foreground">Personlig färg: </p>
    <div class="flex justify-center items-center w-20 text-muted-foreground border size-fit p-1 px-2 rounded-sm relative">
      {selectedHex}
      <input class="absolute size-full opacity-0" type="color" bind:value={selectedHex}>
    </div>
    {#if hasChangedColor}
      <Button variant="secondary" class="h-full">Byt färg</Button> 
    {/if}
  </div>
  <div class="flex flex-wrap md:flex-nowrap gap-2 items-center">
    <p class="text-muted-foreground">Användarnamn: </p>
    <Input type="search" bind:value={usernameInput} class="w-full rounded-md"/>
    {#if usernameInput != data.user.username}
      <Button variant="default">Byt användarnamn</Button>
    {:else} 
      <!-- TODO: Give feedback of available names as the user types -->
      <Button disabled variant="default">Byt användarnamn</Button>
    {/if}
  </div>
  <div class="flex gap-3">
    <Button href="/change-password" variant="secondary">Byt lösenord</Button>
    <Button variant="destructive">Ta bort profil</Button>
  </div>
</div>
