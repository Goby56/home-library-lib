<script lang="ts">
  import "./../../app.css"
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import { ModeWatcher } from "mode-watcher";

  import AppSidebar from "$lib/components/AppSidebar.svelte";
  import AppHeader from "$lib/components/AppHeader.svelte";
  import AppContent from "$lib/components/AppContent.svelte";

  import type { LayoutProps } from './$types';

	let { data, children }: LayoutProps = $props();

</script>

<AppHeader user={data.user}/>
<AppContent>
  {@render children?.()}
</AppContent>
