<script lang="ts">
  import RangeCalendar from "$lib/components/custom-range-calendar/wrapper/range-calendar.svelte";
  import { CalendarDate, type DateValue } from "@internationalized/date";
  import type { DateRange } from "bits-ui";
  import { MediaQuery } from "svelte/reactivity";
  import type { HighlightedRange } from "$lib/components/custom-range-calendar/base/types";
  let value = $state<DateRange>({
    start: undefined,
    end: undefined,
  });

  const isDesktop = new MediaQuery("(min-width: 768px)");

  let numberOfMonths = $derived(isDesktop.current ? 5 : 2);

  let reservations: HighlightedRange[] = [
    {
      start: new CalendarDate(2025, 6, 11),
      end: new CalendarDate(2025, 6, 28),
    },
    {
      start: new CalendarDate(2025, 7, 5),
      end: new CalendarDate(2025, 7, 13),
    }
  ]

  function onClickHightlight(date: DateValue, highlight: number) {
    console.log(highlight);
  }

</script>
<RangeCalendar bind:value class="rounded-lg" ranges={reservations} learnMore={onClickHightlight} fixedWeeks={false} {numberOfMonths}/>
