<script lang="ts">
	import { RangeCalendar as RangeCalendarPrimitive } from "../base";
	import { cn } from "$lib/utils.js";

	let {
		ref = $bindable(null),
		class: className,
		...restProps
	}: RangeCalendarPrimitive.HeaderProps = $props();
</script>

<RangeCalendarPrimitive.Header
	bind:ref
	class={cn(
    "relative flex w-full items-center md:justify-center justify-between",
    "gap-2 py-1 bg-accent rounded-sm md:rounded-b-none"
    , className)}
	{...restProps}
/>
