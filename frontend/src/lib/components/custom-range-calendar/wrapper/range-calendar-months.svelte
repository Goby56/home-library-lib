<script lang="ts">
	import type { WithElementRef } from "bits-ui";
	import type { HTMLAttributes } from "svelte/elements";
	import { cn } from "$lib/utils.js";

	let {
		ref = $bindable(null),
		class: className,
		children,
		...restProps
	}: WithElementRef<HTMLAttributes<HTMLDivElement>> = $props();
</script>

<div
	bind:this={ref}
	class={cn(
    "flex flex-col space-y-4 sm:flex-row sm:space-x-4 sm:space-y-0", 
    "md:border-x-4 md:border-b-4 rounded-b-md md:px-2 md:pb-2", 
    className)}
	{...restProps}
>
	{@render children?.()}
</div>
