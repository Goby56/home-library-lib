<script lang="ts">
  import RangeCalendar from "$lib/components/custom-range-calendar/wrapper/range-calendar.svelte";
  import { CalendarDate, parseAbsoluteToLocal, parseDate, parseDateTime, parseZonedDateTime, type DateValue } from "@internationalized/date";
  import type { DateRange } from "bits-ui";
  import { MediaQuery } from "svelte/reactivity";
  import type { HighlightedRange } from "$lib/components/custom-range-calendar/base/types";
    import { isAfter, isBefore, isBetweenInclusive } from "$lib/components/custom-range-calendar/internal/date-time/utils";
    import { boolean } from "zod";

  let { reservations, numberOfMonths, value = $bindable({start: undefined, end: undefined}), disabled = false }: 
    { reservations: any[], numberOfMonths: number, value: DateRange, disabled?: boolean } = $props();

  // const isDesktop = new MediaQuery("(min-width: 768px)");

  // let numberOfMonths = $derived(isDesktop.current ? 5 : 2);

  let ranges: HighlightedRange[] = $derived(reservations.map(rsv => ({
      start: parseAbsoluteToLocal(rsv.start_date),
      end: parseAbsoluteToLocal(rsv.end_date),
      color: rsv.user.personal_color,
    })
  ));
  
  $effect(() => {
    // Reset selection if ranges overlap
    ranges.forEach(rsv => {
      if (value.start && value.end) {
        if (value.start < rsv.end && rsv.end < value.end) {
          value = {start:undefined, end: undefined};
        }
      }
      if (value.start && isBetweenInclusive(value.start, rsv.start, rsv.end)) {
        value = {start:undefined, end: undefined};
      }
    })
  })

  function onClickHightlight(date: DateValue, highlight: number) {
    console.log(reservations[highlight].user.username);
  }

</script>
<RangeCalendar bind:value class="rounded-lg" {ranges} learnMore={onClickHightlight} {disabled} fixedWeeks={false} {numberOfMonths}/>
