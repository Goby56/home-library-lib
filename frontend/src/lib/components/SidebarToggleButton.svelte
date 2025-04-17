<script lang="ts">
  import Button from "./ui/button/button.svelte";
  import PanelLeft from "@lucide/svelte/icons/panel-left";
  import { useSidebar } from "$lib/components/ui/sidebar/context.svelte.js";
	const sidebar = useSidebar();
</script>

<Button
	type="button"
	onclick={() => {
		sidebar.toggle();
	}}
	data-sidebar="trigger"
	variant="ghost"
	size="icon"
>
<PanelLeft/>
</Button>
