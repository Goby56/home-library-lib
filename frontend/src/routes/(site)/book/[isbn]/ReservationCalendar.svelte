<script lang="ts">
  import RangeCalendar from "$lib/components/custom-range-calendar/wrapper/range-calendar.svelte";
  import { CalendarDate, parseAbsoluteToLocal, parseDate, parseDateTime, parseZonedDateTime, type DateValue } from "@internationalized/date";
  import type { DateRange } from "bits-ui";
  import { MediaQuery } from "svelte/reactivity";
  import type { HighlightedRange } from "$lib/components/custom-range-calendar/base/types";

  let { reservations }: { reservations: any[] } = $props();

  let value = $state<DateRange>({
    start: undefined,
    end: undefined,
  });

  const isDesktop = new MediaQuery("(min-width: 768px)");

  let numberOfMonths = $derived(isDesktop.current ? 5 : 2);

  let ranges: HighlightedRange[] = $derived(reservations.map(rsv => ({
      start: parseAbsoluteToLocal(rsv.start_date),
      end: parseAbsoluteToLocal(rsv.end_date),
      color: rsv.user.personal_color,
    })
  ));

  function onClickHightlight(date: DateValue, highlight: number) {
    console.log(reservations[highlight].user.username);
  }

</script>
<RangeCalendar bind:value class="rounded-lg" {ranges} learnMore={onClickHightlight} fixedWeeks={false} {numberOfMonths}/>
