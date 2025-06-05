<script lang="ts">
  import Button from '$lib/components/ui/button/button.svelte';
  import { Badge } from "$lib/components/ui/badge/index.js";
	import type { PageProps } from './$types';
  import { getLocalTimeZone, today } from "@internationalized/date";
  import { RangeCalendar } from "$lib/components/ui/range-calendar/index.js";
  import * as Popover from "$lib/components/ui/popover/index.js";
  import { ScrollArea } from "$lib/components/ui/scroll-area/index.js";
  import AddToShelfIcon from "@lucide/svelte/icons/between-horizontal-start";
  import PlusIcon from "@lucide/svelte/icons/plus";
  import LoaderCircleIcon from "@lucide/svelte/icons/loader-circle";
  import { invalidateAll } from '$app/navigation';

  import CheckIcon from "@lucide/svelte/icons/check";
  import ChevronsUpDownIcon from "@lucide/svelte/icons/chevrons-up-down";
  import { tick } from "svelte";
  import * as Command from "$lib/components/ui/command/index.js";
  import { cn } from "$lib/utils.js";
  import { buttonVariants } from "$lib/components/ui/button/index.js";
  import * as Tooltip from "$lib/components/ui/tooltip/index.js";
  import axios from "axios";

  import { languageCodes, getLabelFromLanguageCode } from "$lib/utils";
  import PhysicalBookManagerButton from "$lib/components/PhysicalBookManagerButton.svelte";
    import ChevronsUpDown from '@lucide/svelte/icons/chevrons-up-down';
 
	let { data }: PageProps = $props();

  let coverImage = data.cover;
  let shelves = $derived(data.shelves);

  let shelfPopupOpen = $state(false);
  let selectedShelf = $state("");
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

  let shelfInput = $state("");

  let pendingAddToShelf = $state(false);

  async function addToShelf() {
    if (selectedShelf != "") {
      pendingAddToShelf = true;
      
      let physical_copy = {
          isbn: data.book.isbn, name: selectedShelf
      }
      let response = await axios.post("http://192.168.1.223:8080/add_physical_book", physical_copy);

      pendingAddToShelf = false;
      selectedShelf = ""
      invalidateAll();
    }
  }

  export const hydrate = false;

</script>

<div class="flex flex-col">
    <div class="grid md:grid-cols-2 grid-cols-1 gap-3">
      <div class="flex justify-center">
        <img src="{coverImage}" alt="book cover"
        class="rounded-xl h-96">
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
          <div class="flex gap-3 items-center">
            {#each data.copies as physical_copy}
              <PhysicalBookManagerButton book={data.book} physicalCopy={physical_copy} shelves={shelves}/>
            {/each}

            <Popover.Root bind:open={shelfPopupOpen}>
              <Popover.Trigger bind:ref={triggerRef}>
                {#snippet child({ props })}
                  <div class="flex">
                    <Button
                      {...props}
                      class={selectedShelf ? 'rounded-r-none' : ''}
                      role="combobox"
                      variant={selectedShelf ? "secondary" : "default"}
                      aria-expanded={shelfPopupOpen}
                    >
                    {#if selectedShelf}
                      {selectedShelf}
                      <ChevronsUpDownIcon/>
                    {:else if data.copies.length == 0}
                      Placera i bokylla
                    {:else}
                      <PlusIcon/>
                    {/if}
                    </Button>
                    {#if selectedShelf}
                      <Tooltip.Provider>
                        <Tooltip.Root>
                          <Tooltip.Trigger>
                            {#if pendingAddToShelf}
                              <Button disabled variant="outline" class="rounded-l-none">
                                <LoaderCircleIcon class="animate-spin" />
                              </Button>
                            {:else} 
                              <Button onclick={addToShelf} class="rounded-l-none" size="icon">
                                <PlusIcon />
                              </Button>
                            {/if}
                          </Tooltip.Trigger>
                          <Tooltip.Content>
                            <p>Placera i bokhylla</p>
                          </Tooltip.Content>
                        </Tooltip.Root>
                      </Tooltip.Provider>
                    {/if}
                  </div>
                {/snippet}
              </Popover.Trigger>
              <Popover.Content class="w-[200px] p-0">
                <Command.Root>
                  <Command.Input placeholder="Sök efter bokhyllor..." />
                  <Command.List>
                    <Command.Empty class="flex flex-col p-1 gap-1">
                      <p>Bokhyllan hittades inte</p>
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
          </div>
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
    </div>
</div>


