<script lang="ts">
	import { box, mergeProps } from "svelte-toolbelt";
	import type { CalendarHeadingProps } from "../types.js";
	import { useCalendarHeading } from "../calendar.svelte.js";
	import { useId } from "bits-ui";

	let {
		children,
		child,
		ref = $bindable(null),
		id = useId(),
		...restProps
	}: CalendarHeadingProps = $props();

	const headingState = useCalendarHeading({
		id: box.with(() => id),
		ref: box.with(
			() => ref,
			(v) => (ref = v)
		),
	});

	const mergedProps = $derived(mergeProps(restProps, headingState.props));
</script>

{#if child}
	{@render child({ props: mergedProps, headingValue: headingState.headingValue })}
{:else}
	<div {...mergedProps}>
		{#if children}
			{@render children?.({ headingValue: headingState.headingValue })}
		{:else}
			{headingState.headingValue}
		{/if}
	</div>
{/if}
