<script lang="ts">
  import Button from '$lib/components/ui/button/button.svelte';
  import { getLocalTimeZone, today } from "@internationalized/date";
  import { RangeCalendar } from "$lib/components/ui/range-calendar/index.js";
  import * as Popover from "$lib/components/ui/popover/index.js";

  let { physical_copy } = $props();
 
  const start = today(getLocalTimeZone());
  const end = start.add({ days: 7 });
 
  let reservation_dates = $state({
    start,
    end
  });

</script>

<Popover.Root>
<Popover.Trigger>
  <Button>{physical_copy.name}</Button>
</Popover.Trigger>
<Popover.Content class="p-0">
<RangeCalendar bind:value={reservation_dates} class="rounded-md border" />
</Popover.Content>
</Popover.Root>

