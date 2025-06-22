<script lang="ts">
  import Button from '$lib/components/ui/button/button.svelte';
  import { getLocalTimeZone, now, parseAbsoluteToLocal, type DateValue, type ZonedDateTime } from "@internationalized/date";
  import { buttonVariants } from "$lib/components/ui/button/index.js";
  import { RangeCalendar } from "$lib/components/ui/range-calendar/index.js";
  import * as Drawer from "$lib/components/ui/drawer/index.js";
  import * as Popover from "$lib/components/ui/popover/index.js";
  import * as Tabs from "$lib/components/ui/tabs/index.js";
  import { MediaQuery } from "svelte/reactivity";
  import { invalidateAll } from "$app/navigation";
  import LoaderCircleIcon from "@lucide/svelte/icons/loader-circle";
  import ShelfSelector from '$lib/components/ShelfSelector.svelte';

  let { physicalCopy, book, shelves } = $props();
  
  const start = now(getLocalTimeZone()).set({hour: 12, minute: 0, second: 0, millisecond: 0});
  const end = start.add({ days: 7 });
 
  let reservationDates = $state({
    start,
    end
  });

  function isDateUnavailable(date: DateValue): boolean {
    if (physicalCopy.reservation) {
      const start = parseAbsoluteToLocal(physicalCopy.reservation.start_date);
      const end = parseAbsoluteToLocal(physicalCopy.reservation.end_date);
      if (start && end) {
        return date.compare(start) >= 0 && date.compare(end) <= 0;
      }
    }
    return false;
  }

  let reservationDuration = $derived.by(() => {
    if (reservationDates.start && reservationDates.end) {
      return Math.round(reservationDates.end.compare(reservationDates.start) / (24 * 60 * 60 * 1000));
    }
    return null;
  })

  let selectedShelf = $state("");

  let drawerOpen = $state(false);

  let pendingReservation = $state(false);

  async function reserveCopy() {
    pendingReservation = true;
    let start_date = reservationDates?.start?.toAbsoluteString();
    let end_date = reservationDates?.end?.toAbsoluteString();

    if (!start_date || !end_date) {
      return 
    }

    const response = await fetch('/api/book-operations/reserve-copy', {
  		method: 'POST',
  		body: JSON.stringify({
        copy_id: physicalCopy.id,
        start_date,
        end_date,
        }),
  		headers: {
  			'content-type': 'application/json'
  		}
  	});
    pendingReservation = false;
    invalidateAll();
  }

  async function moveCopy() {
    const response = await fetch('/api/book-operations/edit-copy', {
  		method: 'POST',
  		body: JSON.stringify({
        copy_id: physicalCopy.id,
        new_shelf_name: selectedShelf,
        }),
  		headers: {
  			'content-type': 'application/json'
  		}
  	});
  }

  let pendingRemoval = $state(false);

  async function removeCopy() {
    pendingRemoval = true;
    const response = await fetch('/api/book-operations/edit-copy', {
  		method: 'POST',
  		body: JSON.stringify({
        copy_id: physicalCopy.id,
        new_shelf_name: "",
        }),
  		headers: {
  			'content-type': 'application/json'
  		}
  	});
    pendingRemoval = false;
    invalidateAll();
  }

  const isDesktop = new MediaQuery("(min-width: 768px)");

</script>

{#snippet editDeleteButtons()}
  <div class="flex justify-center gap-3">
    <ShelfSelector bind:value={selectedShelf} action={moveCopy} shelves={shelves}>
      {#snippet actionTrigger(performAction)}
        {#if selectedShelf != "" && selectedShelf != physicalCopy.shelf.name}
          <Button onclick={performAction} class="rounded-l-none">
            Flytta
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
      {#if pendingRemoval}
        <Button disabled variant="destructive">
          Tar bort bok
          <LoaderCircleIcon class="animate-spin"/>
        </Button>
      {:else} 
        <Button variant="destructive" onclick={removeCopy}>Ta bort</Button>
      {/if}
  </div>
{/snippet}

{#if isDesktop.current}
  <Popover.Root>
  <Popover.Trigger>
    <Button variant="secondary">{physicalCopy.shelf.name}</Button>
  </Popover.Trigger>
  <Popover.Content>
    <p class="text-muted-foreground text-sm text-center pb-3">Vad vill du göra med <b>{book.title}</b> på hylla {physicalCopy.shelf.name}?</p>
    {@render editDeleteButtons()}
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
      {@render editDeleteButtons()}
      <Drawer.Footer>
        <Drawer.Close class={buttonVariants({ variant: "outline" })}>
          Tillbaka
        </Drawer.Close>
      </Drawer.Footer>
    </Drawer.Content>
  </Drawer.Root>
{/if}
