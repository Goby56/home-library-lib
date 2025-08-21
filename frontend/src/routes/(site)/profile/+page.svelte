<script lang="ts">
	import type { PageProps } from './$types';
  import { goto, invalidate, invalidateAll } from '$app/navigation';
  import EditIcon from "@lucide/svelte/icons/pencil";
  import LoaderCircleIcon from "@lucide/svelte/icons/loader-circle";
  import RemoveIcon from "@lucide/svelte/icons/x";
  import Button, { buttonVariants } from '$lib/components/ui/button/button.svelte';
  import { MediaQuery } from 'svelte/reactivity';
  import Input from '$lib/components/ui/input/input.svelte';
  import DarkModeToggle from '$lib/components/DarkModeToggle.svelte';
  import * as Drawer from "$lib/components/ui/drawer/index.js";
  import { reservationDuration } from "$lib/utils";
  import { mode } from 'mode-watcher';

 
	let { data }: PageProps = $props();
  
  const initialColor = `#${data.user.personal_color}`;
  let selectedHex = $state(initialColor);

  let hasChangedColor = $derived(selectedHex != initialColor);

  let usernameInput = $state(data.user.username);

  const isDesktop = new MediaQuery("(min-width: 768px)");

  let drawerOpen = $state(false);

  let currentReservation: any = $state(undefined);
  function handleReservationClick(reservation: any) {
    if (isDesktop.current) {
      goto(`/book/${reservation.isbn || reservation.uuid}?copy=${reservation.copy_id}`)
    } else {
      drawerOpen = true; 
      currentReservation = reservation;
    }
  }

  let pendingReservationRemoval = $state(false);

  async function removeReservation(event: any, reservation: any) {
    event.stopPropagation();
    pendingReservationRemoval = true;
    let response = await fetch("/api/remove-reservation?id=" + reservation.reservation.id, { method: "POST" });
    invalidateAll();
    console.log(await response.text());
    pendingReservationRemoval = false;
    drawerOpen = false;
  }

  let pendingUsernameChange = $state(false);
  let usernameTaken = $state(false);

  $effect(() => {
    if (usernameInput)  {
      usernameTaken = false;
    }
  })

  async function changeUsername(newUsername: string) {
    pendingUsernameChange = true;
    let response = await fetch("/api/change-username?new_username=" + newUsername, { method: "POST" });
    if (!response.ok) {
      if (response.status == 409) {
        usernameTaken = true; 
      }
      console.log(await response.text());
    } else {
      invalidateAll();
    }
    pendingReservationRemoval = false;
  }

  async function changeColor(newColor: string) {
    // Remove # in the beginning as requests ignore everything after raw # in url
    let response = await fetch("/api/change-personal-color?new_color=" + newColor.slice(1), { method: "POST" });
    if (response.ok) {
      hasChangedColor = false; 
      invalidateAll();
    } else {
      console.log(response)
    }
  }
</script>

<div class="flex gap-5">
  <div class="flex flex-col items-center size-24 md:size-32 rounded-3xl relative group" style="background-color: {selectedHex}">
    <input type="color" class="opacity-0 aspect-square size-full" bind:value={selectedHex}>
    <EditIcon class="{mode.current == "light" ? "text-background" : "text-foreground"} group-hover:opacity-100 opacity-0 pointer-events-none size-12 absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2"/>
    {#if hasChangedColor}
      <Button onclick={() => changeColor(selectedHex)} class="text-muted-foreground p-0 pt-1 h-fit" variant="link">Byt till vald färg</Button>
    {/if}
  </div>
  <div class="flex flex-col gap-2 items-start">
    <h1 class="scroll-m-20 text-4xl font-extrabold tracking-tight lg:text-5xl">
      {data.user.username}
    </h1>
    <p class="text-muted-foreground text-sm hidden md:flex">Här kan du ändra dina inloggningsuppgifter och din personliga färg</p>
    <form method="POST" action="/api/logout">
      <Button type="submit" class="text-foreground" variant="secondary">Logga ut</Button>
    </form>
  </div>
</div>
<h2 class="scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight transition-colors mt-5 mb-4">
  Reservationer
</h2>
<div class="flex flex-wrap gap-3">
  {#each data.user_reservations as rsv}
    <button onclick={() => handleReservationClick(rsv)} class="flex relative hover:bg-muted bg-muted/50 p-1 px-2 rounded-md group">
    <!--
      {#await getBookCover(book.isbn)}
        <img src={placeHolderImage} alt="{book.title} book cover" class="rounded-md h-48">
      {:then coverImage} 
        <img src={coverImage.url} alt="{book.title} book cover" class="rounded-md h-48">
      {:catch}
        <img src={placeHolderImage} alt="{book.title} book cover" class="rounded-md h-48">
      {/await}
    -->
      <div class="flex flex-col items-start">
        <p class="font-semibold">{rsv.title}</p>
        <p class="text-muted-foreground">Hylla: {rsv.shelf.name}</p>
        <div class="flex flex-col md:flex-row">
        <p>{reservationDuration(rsv.reservation)}</p>
        <Button
          onclick={(e) => removeReservation(e, rsv)}
          variant="destructive"
          class="hidden md:group-hover:flex size-6 rounded-full absolute top-0 right-0 transform translate-x-1/2 -translate-y-1/2"
          size="icon"
          >
          {#if pendingReservationRemoval}
            <LoaderCircleIcon/>
          {:else}
            <RemoveIcon/>
          {/if}
        </Button>
      </div>
    </button>
  {:else}
    <p class="text-muted-foreground">Du har inga reservationer</p>
  {/each} 
</div>
<h2 class="scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight transition-colors mt-5 mb-2">
  Inställningar
</h2>
<div class="flex flex-col gap-3 items-start">
  <div class="flex gap-2 items-center">
    <p class="text-muted-foreground">Mörkt läge: </p>
    <DarkModeToggle/>
  </div>
  <div class="flex gap-2 items-center">
    <p class="text-muted-foreground">Personlig färg: </p>
    <div class="flex justify-center items-center w-20 text-muted-foreground border size-fit p-1 px-2 rounded-sm relative">
      {selectedHex}
      <input class="absolute size-full opacity-0" type="color" bind:value={selectedHex}>
    </div>
    {#if hasChangedColor}
      <Button onclick={() => changeColor(selectedHex)} variant="secondary" class="h-full">Byt färg</Button> 
    {/if}
  </div>
  <div class="flex flex-wrap md:flex-nowrap gap-2 items-center">
    <p class="text-muted-foreground">Användarnamn: </p>
    <Input type="search" bind:value={usernameInput} class="w-full rounded-md {usernameTaken ? 'border-destructive' : ''}"/>
    {#if usernameInput != data.user.username && usernameInput}
      {#if usernameTaken}
        <Button disabled variant="default">Användarnamnet är taget</Button>
      {:else}
        <Button onclick={() => changeUsername(usernameInput)} variant="default">Byt användarnamn</Button>
      {/if}
    {:else} 
      <Button disabled variant="default">Byt användarnamn</Button>
    {/if}
  </div>
  <div class="flex gap-3">
    <Button href="/help#change-password" variant="secondary">Byt lösenord</Button>
    <Button variant="destructive">Ta bort profil</Button>
  </div>
</div>

{#if !isDesktop.current && currentReservation != undefined}
  <Drawer.Root bind:open={drawerOpen}>
    <Drawer.Content>
      <Drawer.Header>
        <Drawer.Title>{currentReservation.title}</Drawer.Title>
        <Drawer.Description>
          <p>{reservationDuration(currentReservation.reservation)}</p>
        </Drawer.Description>
      </Drawer.Header>
      <Drawer.Footer class="pt-0 gap-5">
        <div class="flex justify-center gap-3">
          <Button href="/book/{currentReservation.isbn || currentReservation.uuid}?copy={currentReservation.copy_id}" variant="secondary">
            Gå till boken (hylla {currentReservation.shelf.name})
          </Button>
          <Button onclick={(e) => removeReservation(e, currentReservation)} variant="destructive">
            Ta bort reservation
            {#if pendingReservationRemoval}
              <LoaderCircleIcon/>
            {/if}
          </Button>
        </div>
        <Drawer.Close class={buttonVariants({ variant: "outline" })}>Tillbaka</Drawer.Close>
      </Drawer.Footer>
    </Drawer.Content>
  </Drawer.Root>
{/if}
