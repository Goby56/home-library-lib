<script lang="ts">
  import Button from '$lib/components/ui/button/button.svelte';
  import { getLocalTimeZone, today } from "@internationalized/date";
  import { buttonVariants } from "$lib/components/ui/button/index.js";
  import { RangeCalendar } from "$lib/components/ui/range-calendar/index.js";
  import * as Drawer from "$lib/components/ui/drawer/index.js";
  import * as Popover from "$lib/components/ui/popover/index.js";
  import * as Tabs from "$lib/components/ui/tabs/index.js";
  import { MediaQuery } from "svelte/reactivity";

  import LoaderCircleIcon from "@lucide/svelte/icons/loader-circle";
  import ShelfSelector from '$lib/components/ShelfSelector.svelte';
  import { enhance } from '$app/forms';

  let { physicalCopy, book, shelves } = $props();
 
  const start = today(getLocalTimeZone());
  const end = start.add({ days: 7 });
 
  let reservationDates = $state({
    start,
    end
  });

  let reservationDuration = $derived.by(() => {
    if (reservationDates.start && reservationDates.end) {
      return reservationDates.end.compare(reservationDates.start)
    }
    return null;
  })

  let reservationStart = $derived(reservationDates?.start.toString());
  let reservationEnd = $derived(reservationDates?.end.toString());

  let selectedShelf = $state("");

  let pendingRemoval = $state(false);

  let drawerOpen = $state(false);

  let pendingReservation = $state(false);

  const isDesktop = new MediaQuery("(min-width: 768px)");

</script>

{#if isDesktop.current}
  <Popover.Root>
  <Popover.Trigger>
    <Button variant="secondary">{physicalCopy.shelf.name}</Button>
  </Popover.Trigger>
  <Popover.Content>
    <Tabs.Root value="reserve" class="flex flex-col items-center">
      <Tabs.List>
        <Tabs.Trigger value="reserve">Reservera</Tabs.Trigger>
        <Tabs.Trigger value="edit">Redigera</Tabs.Trigger>
      </Tabs.List>
      <Tabs.Content value="reserve">
        <form method="POST" use:enhance={() => {
            pendingReservation = true;
            return async ({ update }) => {
              await update();
              pendingReservation = false;
            }
        }}>
          <input type="hidden" name="reservationStart" bind:value={reservationStart}/>
          <input type="hidden" name="reservationEnd" bind:value={reservationEnd}/>
          <input type="hidden" name="physicalCopyID" bind:value={physicalCopy.id}/>
          <RangeCalendar bind:value={reservationDates} class="" />
          <div class="flex flex-col gap-2 items-center">
            {#if reservationDuration}
              <Button formaction="?/reserve_copy">Reservera</Button>
              <p class="text-center text-muted-foreground text-sm px-2">Du är påväg att reservera <b>{book.title}</b> på hyllan {physicalCopy.shelf.name} i <u>{reservationDuration} dagar</u></p>
            {:else}   
              <Button disabled>Reservera</Button>
              <p class="text-center text-muted-foreground text-sm px-2">Ange två datum som du vill reservera boken mellan</p>
            {/if}
          </div>
        </form>
      </Tabs.Content>
      <Tabs.Content class="w-full" value="edit">
        <div class="flex justify-center gap-3">
          <ShelfSelector bind:value={selectedShelf} shelves={shelves}>
            {#snippet actionTrigger()}
              {#if selectedShelf != "" && selectedShelf != physicalCopy.shelf.name}
                <Button formaction="?/move_copy" class="rounded-l-none">
                  Byt
                </Button>
              {:else}
                <Button disabled variant="outline" class="rounded-l-none">
                Byt
                </Button>
              {/if}
            {/snippet}
            {#snippet noShelfSelected()}
              Byt bokhylla
            {/snippet}
          </ShelfSelector>
          <form method="POST" use:enhance={() => {
              pendingRemoval = true;
              return async ({ update }) => {
                await update();
                pendingRemoval = false;
              }

          }}>
            <input type="hidden" name="physicalCopyID" bind:value={physicalCopy.id}/>
            {#if pendingRemoval}
              <Button disabled variant="destructive">
                Tar bort bok
                <LoaderCircleIcon class="animate-spin"/>
              </Button>
            {:else} 
              <Button variant="destructive" formaction="?/remove_copy">Ta bort bok</Button>
            {/if}
          </form>
        </div>
      </Tabs.Content>
    </Tabs.Root>
  </Popover.Content>
  </Popover.Root>
{:else}
  <Drawer.Root bind:open={drawerOpen}>
    <Drawer.Trigger>
      <Button variant="secondary">{physicalCopy.shelf.name}</Button>
    </Drawer.Trigger>
    <Drawer.Content>
      <Drawer.Header>
        <Drawer.Description>Vad vill du göra med <b>{book.title}</b> på hylla {physicalCopy.shelf.name}?</Drawer.Description>
      </Drawer.Header>

        <div class="flex justify-center gap-3">
  
        <ShelfSelector bind:value={selectedShelf} shelves={shelves}>
          {#snippet actionTrigger()}
            {#if selectedShelf != "" && selectedShelf != physicalCopy.shelf.name}
              <Button formaction="?/move_copy" class="rounded-l-none">
                Byt
              </Button>
            {:else}
              <Button disabled variant="outline" class="rounded-l-none">
              Byt
              </Button>
            {/if}
          {/snippet}
          {#snippet noShelfSelected()}
            Flytta
          {/snippet}
        </ShelfSelector>
        <form method="POST" use:enhance={() => {
            pendingRemoval = true;
            return async ({ update }) => {
              await update();
              pendingRemoval = false;
            }

        }}>
          <input type="hidden" name="physicalCopyID" bind:value={physicalCopy.id}/>
          {#if pendingRemoval}
            <Button disabled variant="destructive">
              Tar bort bok
              <LoaderCircleIcon class="animate-spin"/>
            </Button>
          {:else} 
            <Button variant="destructive" formaction="?/remove_copy">Ta bort bok</Button>
          {/if}
        </form>
        </div>
      <Drawer.Footer>
        <Drawer.Close class={buttonVariants({ variant: "outline" })}>
          Tillbaka
        </Drawer.Close>
      </Drawer.Footer>
    </Drawer.Content>
  </Drawer.Root>
{/if}
