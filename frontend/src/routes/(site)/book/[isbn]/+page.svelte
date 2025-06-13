<script lang="ts">
	import type { PageProps } from './$types';
  import { MediaQuery } from "svelte/reactivity";
  import PhysicalBookManagerButton from './PhysicalBookManagerButton.svelte';
  import { getLabelFromLanguageCode } from "$lib/utils";
  import * as Drawer from "$lib/components/ui/drawer/index.js";
  import { buttonVariants } from "$lib/components/ui/button/index.js";
  import Button from "$lib/components/ui/button/button.svelte";
  import PlusIcon from "@lucide/svelte/icons/plus";
  import { getLocalTimeZone, today } from "@internationalized/date";
  import { RangeCalendar } from "$lib/components/ui/range-calendar/index.js";
  import ShelfSelector from "$lib/components/ShelfSelector.svelte";
  import axios from "axios";
  import { BACKEND_URL } from "$lib/utils.js";
  import PhysicalBookSelector from './PhysicalBookSelector.svelte';
  import { enhance } from '$app/forms';
 
	let { data }: PageProps = $props();

  let coverImage = data.cover;

  let shelves = $derived(data.shelves);

  let selectedShelf = $state("");

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

  let selectedCopy: any = $state(undefined);
  let selectedCopyID = $derived(selectedCopy?.id);

  let pendingReservation = $state(false);

</script>

<div class="flex flex-col mb-20">
    <div class="grid md:grid-cols-2 grid-cols-1 gap-3">
      <div class="flex justify-center">
        <img src="{coverImage}" alt="book cover"
        class="rounded-xl md:h-96 h-72">
      </div>

      <div class="flex flex-col gap-3">
        <h1 class="scroll-m-20 text-4xl font-extrabold tracking-tight lg:text-5xl">
        {data.book.title}
        </h1>
        <div class="flex">
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
            <ShelfSelector bind:value={selectedShelf} shelves={data.shelves}>
              {#snippet actionTrigger()}
                <Button formaction="?/add_copy" class="rounded-l-none" size="icon"><PlusIcon/></Button>
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

<Drawer.Root>
  <Drawer.Trigger class="flex justify-center">
    <div class="md:hidden flex justify-center w-full px-5 fixed bottom-5">
      <Button class="w-full text-lg font-semibold">
        Reservera
      </Button>
    </div>
  </Drawer.Trigger>
  <Drawer.Content>
    <Drawer.Header>
      <Drawer.Title>Reservera bok</Drawer.Title>
    </Drawer.Header>
    <form method="POST" use:enhance={() => {
      pendingReservation = true;
      return async ({ update }) => {
        await update();
        pendingReservation = false;
      }
    }}>
      <div class="flex flex-col justify-start items-center h-full">

        <RangeCalendar bind:value={reservationDates} />
        <input type="hidden" name="reservationStart" bind:value={reservationStart}/>
        <input type="hidden" name="reservationEnd" bind:value={reservationEnd}/>
        <div class="flex flex-col gap-2 items-center">
          
          <PhysicalBookSelector bind:selectedCopy={selectedCopy} physicalCopies={data.copies}/>
          <input type="hidden" name="physicalCopyID" bind:value={selectedCopyID}/>

          {#if reservationDuration && selectedCopy}
            <p class="text-center text-muted-foreground text-sm px-2">Du är påväg att reservera <b>{data.book.title}</b> på hyllan {selectedCopy.shelf.name} i <u>{reservationDuration} dagar</u></p>
          {:else}
            {#if !reservationDuration}
              <p class="text-center text-muted-foreground text-sm px-2">Ange två datum som du vill reservera boken mellan</p>
            {/if}
            {#if !selectedCopy}   
              <p class="text-center text-muted-foreground text-sm px-2">Välj vilken bokhylla du vill låna boken från</p>
            {/if}
          {/if}
        </div>

        <Drawer.Footer class="w-full bottom-0">
          {#if reservationDuration && selectedCopy}
            <Button type="submit" formaction="?/reserve_copy">Reservera</Button>
          {:else}
            <Button disabled>Reservera</Button>
          {/if}
          <Drawer.Close class={buttonVariants({ variant: "outline" })}>
            Tillbaka
          </Drawer.Close>
        </Drawer.Footer>
      </div>
    </form>
  </Drawer.Content>
</Drawer.Root>
