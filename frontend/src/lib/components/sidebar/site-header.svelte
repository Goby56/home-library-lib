<script lang="ts">
  import SidebarIcon from "@lucide/svelte/icons/sidebar";
  import SearchForm from "./search-form.svelte";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Separator } from "$lib/components/ui/separator/index.js";
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  const sidebar = Sidebar.useSidebar();
</script>
<header class="bg-background sticky top-0 z-50 flex w-full items-center border-b">
  <div class="h-12 flex w-full items-center gap-2 px-4">
    <Button class="size-10" variant="ghost" size="icon" onclick={sidebar.toggle}>
      <SidebarIcon />
    </Button>
    <Separator orientation="vertical" class="mr-2 h-4" />
    <SearchForm class="w-full sm:ml-auto sm:w-auto" />
  </div>
</header>
