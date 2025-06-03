<script lang="ts">
  import Button from '$lib/components/ui/button/button.svelte';
  import { Badge } from "$lib/components/ui/badge/index.js";
	import type { PageProps } from './$types';
  import { getLocalTimeZone, today } from "@internationalized/date";
  import { RangeCalendar } from "$lib/components/ui/range-calendar/index.js";
  import * as Popover from "$lib/components/ui/popover/index.js";
  import { ScrollArea } from "$lib/components/ui/scroll-area/index.js";

  import CheckIcon from "@lucide/svelte/icons/check";
  import ChevronsUpDownIcon from "@lucide/svelte/icons/chevrons-up-down";
  import { tick } from "svelte";
  import * as Command from "$lib/components/ui/command/index.js";
  import { cn } from "$lib/utils.js";

  import { languageCodes, getLabelFromLanguageCode } from "$lib/utils";
  import BookingButtom from "$lib/components/BookingButton.svelte";
 
	let { data }: PageProps = $props();


  let book = data.book, book_copies = data.copies, coverImage = data.cover;
  let shelves = data.shelves;

  let shelfPopupOpen = $state(false);
  let selectedShelf = $state("");
  let shelfInput = $state("");
  let triggerRef = $state<HTMLButtonElement>(null!);
 
  // We want to refocus the trigger button when the user selects
  // an item from the list so users can continue navigating the
  // rest of the form with the keyboard.
  function closeAndFocusTrigger() {
    shelfPopupOpen = false;
    tick().then(() => {
      triggerRef.focus();
    });
  }
</script>

<div class="flex flex-col">
    <div class="grid md:grid-cols-2 grid-cols-1 gap-3">
      <div class="flex justify-center">
        <img src="{coverImage}" alt="book cover"
        class="rounded-xl h-96">
      </div>
      <div class="flex flex-col gap-3">
        <h1 class="scroll-m-20 text-4xl font-extrabold tracking-tight lg:text-5xl">
        {book.title}
        </h1>
        <div class="flex">
          {#each book.authors as author, i (author)}
            <span>{author}{i < book.authors.length - 1 ? ', ' : ''}</span>
          {/each}
        </div>
        <div class="flex flex-row gap-5 items-center">
        <div class="flex gap-3 items-center">

        <Popover.Root bind:open={shelfPopupOpen}>
          <Popover.Trigger bind:ref={triggerRef}>
            {#snippet child({ props })}
              <Button
                {...props}
                role="combobox"
                aria-expanded={shelfPopupOpen}
              >
                {selectedShelf || "Tilldela bokhylla"}
              </Button>
            {/snippet}
          </Popover.Trigger>
          <Popover.Content class="w-[200px] p-0">
            <Command.Root>
              <Command.Input bind:value={shelfInput} placeholder="Sök efter bokhylla..." />
              <Command.List>
                <Command.Empty class="flex flex-col p-1 gap-1">
                  <p>Ingen bokhylla hittades</p>
                  <Button variant="outline"> Lägg till {shelfInput.toUpperCase()}</Button>
                </Command.Empty>
                <Command.Group class="p-0" value="shelves">
                  {#each shelves as shelf}
                    <Command.Item
                      value={shelf}
                      onSelect={() => {
                        selectedShelf = shelf;
                        closeAndFocusTrigger();
                      }}
                    >
                      <CheckIcon
                        class={cn(selectedShelf !== shelf && "text-transparent")}
                      />
                      {shelf}
                    </Command.Item>
                  {/each}
                </Command.Group>
              </Command.List>
            </Command.Root>
          </Popover.Content>
        </Popover.Root>



          {#if book_copies.length == 0}
            <p>Denna bok tillhör ännu inte någon bokhylla</p>
          {/if}
          {#each book_copies as physical_copy}
            <BookingButtom physical_copy={physical_copy}/>
          {/each}
        </div>
        </div>

        <h2 class="scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight transition-colors first:mt-0">
          Information
        </h2>
        <div class="grid grid-cols-2 gap-y-3 p-3 h-fit w-fit rounded-sm border-muted-foreground border-2">
          <div class="flex flex-row items-center">
            <p><b>Publicerad:</b> {book.publication_year}</p>
          </div>
          <div class="flex flex-row items-center">
            <p><b>Språk:</b> {getLabelFromLanguageCode(book.language)}</p>
          </div>
          <div class="flex flex-row items-center">
            <p><b>Antal sidor:</b> {book.page_count}</p>
          </div>
          <div class="flex flex-row items-center">
            <p><b>ISBN:</b> {book.isbn}</p>
          </div>
        </div>
        <h2 class="scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight transition-colors first:mt-0">
          Kategori
        </h2>
        {#each book.genres as genre, i (genre)}
          <span>{genre}{i < book.genres.length - 1 ? ', ' : ''}</span>
        {/each}
      </div>
    </div>
</div>


