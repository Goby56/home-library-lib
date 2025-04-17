<script lang="ts">
  import "../app.css";
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import { ModeWatcher } from "mode-watcher";

  import AppSidebar from "$lib/components/AppSidebar.svelte";
  import AppHeader from "$lib/components/AppHeader.svelte";
  import AppContent from "$lib/components/AppContent.svelte";

  let sideBarOpen = $state(false);

  function onSideBarChange(open: boolean) {
    sideBarOpen = open;
  }

  let { children } = $props();
</script>

<ModeWatcher />
<Sidebar.Provider onOpenChange={onSideBarChange}>
	<AppSidebar />
	<Sidebar.Inset>
		<header class="flex w-full sticky h-16 items-center gap-2 bg-secondary border-b px-4">
			<Sidebar.Trigger class="-ml-1" />
      <AppHeader/>
		</header>
    <AppContent>
      {@render children()}
    </AppContent>
	</Sidebar.Inset>
</Sidebar.Provider>
 
