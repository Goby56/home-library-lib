<script lang="ts">
	import type { PageProps } from './$types';
  import { MediaQuery } from "svelte/reactivity";
  import PhysicalBookManagerButton from './PhysicalBookManagerButton.svelte';
  import LoaderCircleIcon from "@lucide/svelte/icons/loader-circle";
  import ArrowRightIcon from "@lucide/svelte/icons/arrow-right";
  import { getLabelFromLanguageCode } from "$lib/utils";
  import * as Drawer from "$lib/components/ui/drawer/index.js";
  import { buttonVariants } from "$lib/components/ui/button/index.js";
  import Button from "$lib/components/ui/button/button.svelte";
  import PlusIcon from "@lucide/svelte/icons/plus";
  import { getLocalTimeZone, now, parseAbsoluteToLocal, today, ZonedDateTime, type DateValue } from "@internationalized/date";
  import ShelfSelector from "$lib/components/ShelfSelector.svelte";
  import PhysicalBookSelector from './PhysicalBookSelector.svelte';
  import ReservationCalendar from '$lib/components/ReservationCalendar.svelte';
  import { invalidateAll } from '$app/navigation';
 
	let { data }: PageProps = $props();

  let coverImage = $derived(data.cover);

  let shelves = $derived(data.shelves);

  let selectedShelf = $state("");

  const start = now(getLocalTimeZone()).set({hour: 12, minute: 0, second: 0, millisecond: 0});
  const end = start.add({ days: 7 });
 
  let reservationDates = $state({
    start,
    end
  });

  let reservationDuration = $derived.by(() => {
    if (reservationDates.start && reservationDates.end) {
      return Math.round(reservationDates.end.compare(reservationDates.start) / (24 * 60 * 60 * 1000));
    }
    return null;
  })

  let selectedCopy: any = $state(undefined);
  let selectedCopyID = $derived(selectedCopy?.id);

  let reservations = $derived(selectedCopy?.reservations ?? []);

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
        copy_id: selectedCopyID,
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

  let pendingAddToShelf = $state(false);

  async function addToShelf() {
    pendingAddToShelf = true;
    const response = await fetch('/api/book-operations/add-copy', {
  		method: 'POST',
  		body: JSON.stringify({
        uuid: data.book.uuid,
        shelf_name: selectedShelf
      }),
  		headers: {
  			'content-type': 'application/json'
  		}
  	});
    pendingAddToShelf = false;
  }

  const isDesktop = new MediaQuery("(min-width: 768px)");

</script>

<div class="flex flex-col pb-3 md:mb-0 mb-20">
  <div class="grid md:grid-cols-2 grid-cols-1 gap-3">
    <div class="flex justify-center items-center">
      <img src="{coverImage}" alt="book cover"
      class="rounded-xl md:h-80 h-60">
    </div>

    <div class="flex flex-col gap-3">
      <h1 class="scroll-m-20 text-4xl font-extrabold tracking-tight lg:text-5xl">
      {data.book.title}
      </h1>
      <div class="flex gap-1">
        {#each data.book.authors as author, i (author)}
          <span>{author}{i < data.book.authors.length - 1 ? ', ' : ''}</span>
        {/each}
      </div>
      <div class="flex flex-col gap-1">
        {#if data.copies.length == 0}
          <p class="text-muted-foreground text-sm">Denna bok tillhör ännu inte någon bokhylla</p>
        {:else}
          <p class="text-muted-foreground text-sm">Finns i följande bokhyllor:</p>
        {/if}
        <div class="flex gap-3 items-center flex-wrap">
          {#each data.copies as physical_copy}
            <PhysicalBookManagerButton book={data.book} physicalCopy={physical_copy} shelves={shelves}/>
          {/each}
          <ShelfSelector bind:value={selectedShelf} action={addToShelf} shelves={data.shelves}>
            {#snippet actionTrigger(performAction)}
              <Button onclick={performAction} class="rounded-l-none" size="icon"><PlusIcon/></Button>
            {/snippet}
            {#snippet noShelfSelected()}
              {#if data.copies.length == 0}
                Placera i bokylla
              {:else} 
                <PlusIcon/>
              {/if}
            {/snippet}
          </ShelfSelector>
        </div>
      </div>
      <div class="flex flex-col gap-2">
          <div class="flex flex-wrap items-center gap-1">
            <p class="text-sm">Kategori:</p> 
            {#each data.book.genres as genre, i (genre)}
              <span class="bg-muted relative rounded p-1 w-fit text-sm font-semibold">{genre}{i < data.book.genres.length - 1 ? ', ' : ''}</span>
            {/each}
          </div>
          <p class="text-sm">Publicerad: <b>{data.book.publication_year}</b></p>
          <p class="text-sm">Språk: <b>{getLabelFromLanguageCode(data.book.language)}</b></p>
          <p class="text-sm">Antal sidor: <b>{data.book.page_count}</b></p>
          <p class="text-sm">ISBN: <b>{data.book.isbn}</b></p>
      </div>
    </div>
  </div>
</div>

{#snippet reservationButton()}
  {#if pendingReservation}
    <Button disabled>
      Reservera
      <LoaderCircleIcon class="animate-spin"/>
    </Button>
  {:else}
    {#if reservationDuration && selectedCopy}
      <Button onclick={reserveCopy}>Reservera</Button>
    {:else}
      <Button disabled>Reservera</Button>
    {/if}
  {/if}
{/snippet}

<div class="md:flex flex-col justify-start items-center mb-20 hidden">
  <ReservationCalendar bind:value={reservationDates} user={data.user} numberOfMonths={4} {reservations}/>
  <div class="flex justify-center items-center gap-3">
    <div class="flex gap-2 items-center">
      <p class="text-center text-muted-foreground text-sm">Välj en bokhylla:</p>
      <PhysicalBookSelector bind:selectedCopy={selectedCopy} physicalCopies={data.copies}/>
    </div>
    <ArrowRightIcon class="size-5 text-muted-foreground"/>
    {@render reservationButton()}
  </div>
</div>

<Drawer.Root>
  {#if data.copies.length != 0}
    <Drawer.Trigger class="flex justify-center">
      <div class="md:hidden flex justify-center w-full px-5 fixed bottom-5">
        <Button class="w-full text-lg font-semibold">
          Reservera
        </Button>
      </div>
    </Drawer.Trigger>
  {/if}
  <Drawer.Content>
    <Drawer.Header>
      <Drawer.Title>Reservera bok</Drawer.Title>
    </Drawer.Header>
    <div class="flex flex-col justify-start items-center h-full">
      <ReservationCalendar bind:value={reservationDates} user={data.user} numberOfMonths={1} {reservations}/>
      <div class="flex flex-col gap-2 items-center">
        
        <PhysicalBookSelector bind:selectedCopy={selectedCopy} physicalCopies={data.copies}/>
        <div class="flex flex-col justify-start items-center h-12 gap-1">
          {#if reservationDuration && selectedCopy}
            <p class="text-center text-muted-foreground text-sm px-2">Du är påväg att reservera <b>{data.book.title}</b> på hyllan {selectedCopy.shelf.name} i <u>{reservationDuration} dagar</u></p>
          {:else}
            {#if !reservationDuration}
              <p class="text-center text-muted-foreground text-sm px-2">Ange två datum som du vill reservera boken mellan</p>
            {/if}
            {#if !selectedCopy}
              {#if data.copies.length != 0}
                <p class="text-center text-muted-foreground text-sm px-2">Välj vilken bokhylla du vill låna boken från</p>
              {:else}
                <p class="text-center text-destructive text-sm px-2">Denna bok kan inte reserveras</p>
              {/if}
            {/if}
          {/if}
        </div>
      </div>
      <Drawer.Footer class="w-full bottom-0">
        {@render reservationButton()}
        <Drawer.Close class={buttonVariants({ variant: "outline" })}>
          Tillbaka
        </Drawer.Close>
      </Drawer.Footer>
    </div>
  </Drawer.Content>
</Drawer.Root>
