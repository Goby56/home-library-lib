<script lang="ts">
	import { RangeCalendar as RangeCalendarPrimitive } from "../base";
	import { cn } from "$lib/utils.js";

	let {
		ref = $bindable(null),
		class: className,
		...restProps
	}: RangeCalendarPrimitive.HeadingProps = $props();
</script>

<RangeCalendarPrimitive.Heading
	bind:ref
	class={cn(
    "text-sm font-medium", 
    "md:w-56 flex justify-center",
    className)}
	{...restProps}
/>
