<script lang="ts">
	let { data } = $props();
  import { languageCodes, getLabelFromLanguageCode } from "$lib/utils";
  import * as Drawer from "$lib/components/ui/drawer/index.js";
  import { buttonVariants } from "$lib/components/ui/button/index.js";
  import Button from "$lib/components/ui/button/button.svelte";
  import { getLocalTimeZone, today } from "@internationalized/date";
  import { Header, RangeCalendar } from "$lib/components/ui/range-calendar/index.js";
  import ShelfSelector from "$lib/components/ShelfSelector.svelte";
  import * as Popover from "$lib/components/ui/popover/index.js";
  import * as Tooltip from "$lib/components/ui/tooltip/index.js";
  import * as Tabs from "$lib/components/ui/tabs/index.js";
  import * as Command from "$lib/components/ui/command/index.js";
  import ChevronsUpDownIcon from "@lucide/svelte/icons/chevrons-up-down";
  import { tick } from "svelte";
  import { cn } from "$lib/utils.js";

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

  let selectedShelf = $state("");

  async function reserve(shelf: string) {

  }

</script>


<div class="flex flex-col gap-3">
  <h1 class="scroll-m-20 text-4xl font-extrabold tracking-tight lg:text-5xl">
  {data.book.title}
  </h1>
  <div class="flex">
    {#each data.book.authors as author, i (author)}
      <span>{author}{i < data.book.authors.length - 1 ? ', ' : ''}</span>
    {/each}
  </div>

  <h2 class="scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight transition-colors first:mt-0">
    Information
  </h2>
  <div class="grid grid-cols-2 gap-y-3 p-3 h-fit w-fit rounded-sm border-muted-foreground border-2">
    <div class="flex flex-row items-center">
      <p><b>Publicerad:</b> {data.book.publication_year}</p>
    </div>
    <div class="flex flex-row items-center">
      <p><b>Språk:</b> {getLabelFromLanguageCode(data.book.language)}</p>
    </div>
    <div class="flex flex-row items-center">
      <p><b>Antal sidor:</b> {data.book.page_count}</p>
    </div>
    <div class="flex flex-row items-center">
      <p><b>ISBN:</b> {data.book.isbn}</p>
    </div>
  </div>
  <h2 class="scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight transition-colors first:mt-0">
    Kategori
  </h2>
  {#each data.book.genres as genre, i (genre)}
    <span>{genre}{i < data.book.genres.length - 1 ? ', ' : ''}</span>
  {/each}
</div>


<Drawer.Root>
  <Drawer.Trigger class="flex justify-center">
    <Button class="w-3/4 fixed bottom-0 mb-10">
      Reservera
    </Button>
  </Drawer.Trigger>
  <Drawer.Content>
    <Drawer.Header>
      <Drawer.Title>Reservera bok</Drawer.Title>
    </Drawer.Header>
    <div class="flex flex-col justify-start items-center h-full">

      <RangeCalendar bind:value={reservationDates} />
      <div class="flex flex-col gap-2 items-center">
          
          <ShelfSelector bind:value={selectedShelf} action={reserve} shelves={data.shelves}>
            {#snippet actionTrigger(performAction)}
              {#if reservationDuration}
                <Button onclick={performAction} class="rounded-l-none">Reservera</Button>
              {:else}   
                <Button disabled class="rounded-l-none">Reservera</Button>
              {/if}
            {/snippet}
            {#snippet noShelfSelected()}
              Välj bokhylla
            {/snippet}
          </ShelfSelector>

        {#if reservationDuration && selectedShelf}
          <p class="text-center text-muted-foreground text-sm px-2">Du är påväg att reservera <b>{data.book.title}</b> på hyllan {selectedShelf} i <u>{reservationDuration} dagar</u></p>
        {:else}
          {#if !reservationDuration}
            <p class="text-center text-muted-foreground text-sm px-2">Ange två datum som du vill reservera boken mellan</p>
          {/if}
          {#if !selectedShelf}   
            <p class="text-center text-muted-foreground text-sm px-2">Välj vilken bokhylla du vill låna boken från</p>
          {/if}
        {/if}
      </div>

      <Drawer.Footer class="bottom-0">
        <Drawer.Close class={buttonVariants({ variant: "outline" })}>
          Tillbaka
        </Drawer.Close>
      </Drawer.Footer>
    </div>
  </Drawer.Content>
</Drawer.Root>
