<script lang="ts">
	import { RangeCalendar as RangeCalendarPrimitive } from "../base";
	import ChevronRight from "@lucide/svelte/icons/chevron-right";
	import { buttonVariants } from "$lib/components/ui/button/index.js";
	import { cn } from "$lib/utils.js";

	let {
		ref = $bindable(null),
		class: className,
		children,
		...restProps
	}: RangeCalendarPrimitive.NextButtonProps = $props();
</script>

{#snippet Fallback()}
	<ChevronRight class="size-4" />
{/snippet}

<RangeCalendarPrimitive.NextButton
	bind:ref
	class={cn(
		buttonVariants({ variant: "default" }),
		"size-7 p-0",
		className
	)}
	children={children || Fallback}
	{...restProps}
/>
