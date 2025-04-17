<script lang="ts">
  import "../app.css";
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import { ModeWatcher } from "mode-watcher";

  import AppSidebar from "$lib/components/AppSidebar.svelte";
  import AppHeader from "$lib/components/AppHeader.svelte";
  import AppContent from "$lib/components/AppContent.svelte";

  let { children } = $props();
</script>

<ModeWatcher />
<Sidebar.Provider>
	<AppSidebar />
	<Sidebar.Inset>
    <AppHeader/>
    <AppContent>
      {@render children()}
    </AppContent>
	</Sidebar.Inset>
</Sidebar.Provider>
 
