<script lang="ts">
  import Button from '$lib/components/ui/button/button.svelte';
  import { getLocalTimeZone, today } from "@internationalized/date";
  import { RangeCalendar } from "$lib/components/ui/range-calendar/index.js";
  import * as Popover from "$lib/components/ui/popover/index.js";
  import * as Tooltip from "$lib/components/ui/tooltip/index.js";
  import * as Tabs from "$lib/components/ui/tabs/index.js";
  import * as Command from "$lib/components/ui/command/index.js";
  import { invalidateAll } from '$app/navigation';
  import axios from "axios";
  import { tick } from "svelte";
  import { cn } from "$lib/utils.js";

  import ArrowRightLeftIcon from "@lucide/svelte/icons/arrow-right-left";
  import LoaderCircleIcon from "@lucide/svelte/icons/loader-circle";
  import CheckIcon from "@lucide/svelte/icons/check";
  import ChevronsUpDownIcon from '@lucide/svelte/icons/chevrons-up-down';

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

  let pendingShelfChange = $state(false);

  async function changeShelf() {
    if (selectedShelf != "") {
      pendingShelfChange = true;
      
      let edit_data = {
          copy_id: physicalCopy.id, new_shelf_name: selectedShelf
      }
      console.log(physicalCopy.id, selectedShelf)
      let response = await axios.post("http://192.168.1.223:8080/edit_physical_book", edit_data);

      pendingShelfChange = false;
      selectedShelf = ""
      invalidateAll();
    }
  }

  let pendingRemoval = $state(false);

  async function removePhysicalBook() {
    pendingRemoval = true;
    let edit_data = {
        copy_id: physicalCopy.id, new_shelf_name: ""
    }
    let response = await axios.post("http://192.168.1.223:8080/edit_physical_book", edit_data);
    invalidateAll();

    pendingRemoval = false;
  }


</script>

<Popover.Root>
<Popover.Trigger>
  <Tooltip.Provider>
    <Tooltip.Root>
      <Tooltip.Trigger>
        <Button variant="secondary">{physicalCopy.shelf.name}</Button>
      </Tooltip.Trigger>
      <Tooltip.Content>
        <p>Reservera</p>
      </Tooltip.Content>
    </Tooltip.Root>
  </Tooltip.Provider>
</Popover.Trigger>
<Popover.Content>
  <Tabs.Root value="reserve" class="flex flex-col items-center">
    <Tabs.List>
      <Tabs.Trigger value="reserve">Reservera</Tabs.Trigger>
      <Tabs.Trigger value="edit">Redigera</Tabs.Trigger>
    </Tabs.List>
    <Tabs.Content value="reserve">
      <RangeCalendar bind:value={reservationDates} class="" />
      <div class="flex flex-col gap-2 items-center">
        {#if reservationDuration}
          <Button>Reservera</Button>
          <p class="text-center text-muted-foreground text-sm px-2">Du är påväg att reservera <b>{book.title}</b> på hyllan {physicalCopy.shelf.name} i <u>{reservationDuration} dagar</u></p>
        {:else}   
          <Button disabled>Reservera</Button>
          <p class="text-center text-muted-foreground text-sm px-2">Ange två datum som du vill reservera boken mellan</p>
        {/if}
      </div>
    </Tabs.Content>
    <Tabs.Content class="w-full" value="edit">
      <div class="flex justify-center gap-3">
        <Popover.Root bind:open={shelfPopupOpen}>
          <Popover.Trigger bind:ref={triggerRef}>
            {#snippet child({ props })}
              <div class="flex">
                <Button
                  {...props}
                  class="rounded-r-none"
                  role="combobox"
                  variant="secondary"
                  aria-expanded={shelfPopupOpen}
                >
                {selectedShelf || physicalCopy.shelf.name}
                <ChevronsUpDownIcon class="ml-2 size-4 shrink-0 opacity-50" />
                </Button>
                <Tooltip.Provider>
                  <Tooltip.Root>
                    <Tooltip.Trigger>
                      {#if pendingShelfChange}
                        <Button disabled variant="outline" class="rounded-l-none">
                          <LoaderCircleIcon class="animate-spin" />
                        </Button>
                      {:else} 
                        {#if selectedShelf != "" && selectedShelf != physicalCopy.shelf.name}
                        <Button onclick={changeShelf} class="rounded-l-none" size="icon">
                          <ArrowRightLeftIcon />
                        </Button>
                        {:else}
                        <Button disabled variant="outline" class="rounded-l-none" size="icon">
                          <ArrowRightLeftIcon />
                        </Button>
                        {/if}
                      {/if}
                    </Tooltip.Trigger>
                    <Tooltip.Content>
                      <p>Flytta bok</p>
                    </Tooltip.Content>
                  </Tooltip.Root>
                </Tooltip.Provider>
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
        {#if pendingRemoval}
          <Button disabled variant="destructive" onclick={removePhysicalBook}>
            Tar bort bok
            <LoaderCircleIcon class="animate-spin"/>
          </Button>
        {:else} 
          <Button variant="destructive" onclick={removePhysicalBook}>Ta bort bok</Button>
        {/if}
      </div>
    </Tabs.Content>
  </Tabs.Root>


</Popover.Content>
</Popover.Root>

