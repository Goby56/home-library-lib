<script lang="ts">
	import { type Month, type WithoutChildrenOrChild } from "bits-ui";
	import { RangeCalendar as RangeCalendarPrimitive } from "../base";
	import * as RangeCalendar from "./index.js";
	import { cn } from "$lib/utils.js";
  import { MediaQuery } from "svelte/reactivity";
  import { type DateValue, type DateTimeDuration, isSameDay } from "@internationalized/date";
    import { isBetweenInclusive } from "../internal/date-time/utils";

	let {
		ref = $bindable(null),
		value = $bindable(),
		placeholder = $bindable(),
    numberOfMonths = $bindable(2),
		weekdayFormat = "short",
    weekStartsOn = 1,
    locale= "sv",
    fixedWeeks = true,
    ranges = [],
		class: className,
		...restProps
	}: WithoutChildrenOrChild<RangeCalendarPrimitive.RootProps> = $props();
  
  const isDesktop = new MediaQuery("(min-width: 768px)");

  const MONTHS = [
    "Januari",
    "Februari",
    "Mars",
    "April",
    "Maj",
    "Juni",
    "Juli",
    "Augusti",
    "September",
    "Oktober",
    "November",
    "December",
  ];

  function borderRounding(date: DateValue): string {
    if (date.subtract({ days: 1 }).month != date.month) {
      return "rounded-l-md";
    }
    if (date.add({ days: 1 }).month != date.month) {
      return "rounded-r-md";
    }
    return "";
  }

  function asRgb(hex: string) {
    return {
      r: parseInt(hex.slice(0,2), 16),
      g: parseInt(hex.slice(2, 4), 16),
      b: parseInt(hex.slice(3, 6), 16),
    }
  }

  function getPersonalHighlightColor(date: DateValue, month: Month<DateValue>) {
    if (month.value.month != date.month || month.value.month != date.month) {
      return Promise.reject();
    }
    for (let i = 0; i < ranges.length; i++) {
        if (isBetweenInclusive(date, ranges[i].start, ranges[i].end)) {
            const isEdge = isSameDay(date, ranges[i].start) || isSameDay(date, ranges[i].end);
            return Promise.resolve({
              ...asRgb(ranges[i].color), isEdge
            });
        }
    }
    return Promise.reject();
  }

</script>

<RangeCalendarPrimitive.Root
	bind:ref
	bind:value
	bind:placeholder
  {ranges}
  {locale}
  {fixedWeeks}
	{weekdayFormat}
  {weekStartsOn}
  {numberOfMonths}
	class={cn("p-3", className)}
	{...restProps}
>
	{#snippet children({ months, weekdays })}
		<RangeCalendar.Header>
			<RangeCalendar.PrevButton />
			<RangeCalendar.Heading />
			<RangeCalendar.NextButton />
		</RangeCalendar.Header>
		<RangeCalendar.Months>
			{#each months as month, i (month)}
				<RangeCalendar.Grid>
					<RangeCalendar.GridHead>
            {#if months.length > 1} 
              <div class="flex justify-center items-center p-2">
                {MONTHS[month.value.month-1]}
              </div>
            {/if}
						<RangeCalendar.GridRow class="flex">
							{#each weekdays as weekday (weekday)}
								<RangeCalendar.HeadCell class={(isDesktop.current || (!isDesktop.current && i == 0)) ? "" : "hidden"}>
									{weekday.slice(0, 2)}
								</RangeCalendar.HeadCell>
							{/each}
						</RangeCalendar.GridRow>
					</RangeCalendar.GridHead>
					<RangeCalendar.GridBody>
						{#each month.weeks as weekDates (weekDates)}
							<RangeCalendar.GridRow class="mt-2 w-full">
								{#each weekDates as date (date)}
                  {#await getPersonalHighlightColor(date, month) then color}
									  <RangeCalendar.Cell {date} month={month.value} class={borderRounding(date)}
                      style="background-color: rgba({color.r},{color.g},{color.b},0.5) !important;">
                      {#if color.isEdge}
									  	  <RangeCalendar.Day
                          style="background-color: rgba({color.r},{color.g},{color.b},0.8) !important;"/>
                      {:else} 
									  	  <RangeCalendar.Day/>
                      {/if}
									  </RangeCalendar.Cell>
                  {:catch} 
									  <RangeCalendar.Cell {date} month={month.value} class={borderRounding(date)}>
									  	<RangeCalendar.Day/>
									  </RangeCalendar.Cell>
                  {/await}
								{/each}
							</RangeCalendar.GridRow>
						{/each}
					</RangeCalendar.GridBody>
				</RangeCalendar.Grid>
			{/each}
		</RangeCalendar.Months>
	{/snippet}
</RangeCalendarPrimitive.Root>
