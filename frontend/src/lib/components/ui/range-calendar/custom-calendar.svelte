<script lang="ts">
	import { RangeCalendar as RangeCalendarPrimitive, type WithoutChildrenOrChild } from "bits-ui";
	import * as RangeCalendar from "./index.js";
	import { cn } from "$lib/utils.js";
    import { MediaQuery } from "svelte/reactivity";

	let {
		ref = $bindable(null),
		value = $bindable(),
		placeholder = $bindable(),
    numberOfMonths = $bindable(2),
		weekdayFormat = "short",
    weekStartsOn = 1,
    locale= "sv",
    isDateUnavailable = undefined,
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
</script>

<RangeCalendarPrimitive.Root
	bind:ref
	bind:value
	bind:placeholder
  {locale}
	{weekdayFormat}
  {weekStartsOn}
  {numberOfMonths}
  isDateDisabled={isDateUnavailable}
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
            <div class="flex justify-center items-center">
              {MONTHS[month.value.month-1]}
            </div>
						<RangeCalendar.GridRow class="flex">
							{#each weekdays as weekday (weekday)}
								<RangeCalendar.HeadCell class={(isDesktop.current || (!isDesktop && i == 0)) ? "" : "hidden"}>
									{weekday.slice(0, 2)}
								</RangeCalendar.HeadCell>
							{/each}
						</RangeCalendar.GridRow>
					</RangeCalendar.GridHead>
					<RangeCalendar.GridBody>
						{#each month.weeks as weekDates (weekDates)}
							<RangeCalendar.GridRow class="mt-2 w-full">
								{#each weekDates as date (date)}
									<RangeCalendar.Cell {date} month={month.value}>
										<RangeCalendar.Day class={date.month == month.value.month ? "" : "hidden"}/>
									</RangeCalendar.Cell>
								{/each}
							</RangeCalendar.GridRow>
						{/each}
					</RangeCalendar.GridBody>
				</RangeCalendar.Grid>
			{/each}
		</RangeCalendar.Months>
	{/snippet}
</RangeCalendarPrimitive.Root>
