<script lang="ts">
  import Button from '$lib/components/ui/button/button.svelte';
  import { getLocalTimeZone, today } from "@internationalized/date";
  import { RangeCalendar } from "$lib/components/ui/range-calendar/index.js";
  import * as Popover from "$lib/components/ui/popover/index.js";
  import * as Tooltip from "$lib/components/ui/tooltip/index.js";
  import * as Tabs from "$lib/components/ui/tabs/index.js";

  let { physicalCopy, book } = $props();
 
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
          <p class="text-center">Du kommer reservera <b>{book.title}</b> på hyllan {physicalCopy.shelf.name} i <u>{reservationDuration} dagar</u></p>
        {:else}   
          <Button disabled>Reservera</Button>
          <p class="text-center">Ange två datum som du vill reservera boken mellan</p>
        {/if}
      </div>
    </Tabs.Content>
    <Tabs.Content value="edit">Byt hylla eller ta bort bok</Tabs.Content>
  </Tabs.Root>


</Popover.Content>
</Popover.Root>

