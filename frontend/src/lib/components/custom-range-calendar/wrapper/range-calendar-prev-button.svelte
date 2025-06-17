<script lang="ts">
	import { RangeCalendar as RangeCalendarPrimitive } from "../base";
	import ChevronLeft from "@lucide/svelte/icons/chevron-left";
	import { buttonVariants } from "$lib/components/ui/button/index.js";
	import { cn } from "$lib/utils.js";

	let {
		ref = $bindable(null),
		class: className,
		children,
		...restProps
	}: RangeCalendarPrimitive.PrevButtonProps = $props();
</script>

{#snippet Fallback()}
	<ChevronLeft class="size-4" />
{/snippet}

<RangeCalendarPrimitive.PrevButton
	bind:ref
	class={cn(
		buttonVariants({ variant: "default" }),
		"size-7 p-0",
		className
	)}
	children={children || Fallback}
	{...restProps}
/>
