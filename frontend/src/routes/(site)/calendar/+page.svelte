<script lang="ts">
  import RangeCalendar from "$lib/components/custom-range-calendar/wrapper/custom-calendar.svelte";
  import { CalendarDate } from "@internationalized/date";
  import type { DateRange } from "bits-ui";
    import { MediaQuery } from "svelte/reactivity";
  let value = $state<DateRange>({
    start: new CalendarDate(2025, 6, 12),
    end: new CalendarDate(2025, 7, 15),
  });

  const isDesktop = new MediaQuery("(min-width: 768px)");

  let numberOfMonths = $derived(isDesktop.current ? 5 : 2);
</script>
<RangeCalendar bind:value class="rounded-lg" fixedWeeks={false} {numberOfMonths}/>
