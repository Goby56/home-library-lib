<script lang="ts">
  import Calendar from "@lucide/svelte/icons/calendar";
  import House from "@lucide/svelte/icons/house";
  import Inbox from "@lucide/svelte/icons/inbox";
  import Search from "@lucide/svelte/icons/search";
  import Settings from "@lucide/svelte/icons/settings";
  import Sailboat from "@lucide/svelte/icons/sailboat";
  import ChartColumnStacked from "@lucide/svelte/icons/chart-column-stacked";
  import Library from "@lucide/svelte/icons/library";
  import Film from "@lucide/svelte/icons/film";
  import User from "@lucide/svelte/icons/user";
  import BookMarked from "@lucide/svelte/icons/film";
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";

  import DarkModeToggle from "./DarkModeToggle.svelte";
  import SidebarToggleButton from "./SidebarToggleButton.svelte";"$lib/components/SidebarToggleButton.svelte"
  import Icon from "./ui/typography/icon.svelte";
  import Plus from "@lucide/svelte/icons/plus";
  import ChevronLeft from "@lucide/svelte/icons/chevrons-left";

  import { useSidebar } from "$lib/components/ui/sidebar/context.svelte.js";
	const sidebar = useSidebar();

  function proximitySidebarTrigger(event: MouseEvent) {
    if (!sidebar.openMobile && event.clientX <= 10 && event.clientY >= 70) {
      sidebar.toggle()
    }
  }
 
  // Menu items.
  const categories = [
    {
      title: "Böcker",
      url: "/books",
      icon: Library,
    },
    {
      title: "Filmer",
      url: "/movies",
      icon: Film,
    },
    {
      title: "Båtar",
      url: "/boats",
      icon: Sailboat,
    },
  ];
  const menuItems = [
    {
      title: "Hem",
      url: "/",
      icon: House,
    },
    {
      title: "Lägg tilll föremål",
      url: "/add",
      icon: Plus,
    },
    {
      title: "Hyllor",
      url: "/shelves",
      icon: ChartColumnStacked,
    },
    {
      title: "Kalender",
      url: "/calendar",
      icon: Calendar,
    },
    {
      title: "Mina sidor",
      url: "#",
      icon: User,
    },
    {
      title: "Inställningar",
      url: "#",
      icon: Settings,
    },
  ]
</script>

<!--
<svelte:window onpointermove={proximitySidebarTrigger}/>
-->

<!--
<div class="fixed left-0 bottom-1/2 bg-primary">
  <ChevronLeft/>
</div>
-->
 
<Sidebar.Root variant="sidebar" >
  <Sidebar.Header class="bg-popover">
    <SidebarToggleButton/>
  </Sidebar.Header>
  <Sidebar.Content class="bg-popover">
    <Sidebar.Group>
      <Sidebar.GroupLabel>Kategorier</Sidebar.GroupLabel>
      <Sidebar.GroupContent>
        <Sidebar.Menu>
          {#each categories as item (item.title)}
            <Sidebar.MenuItem>
              <Sidebar.MenuButton>
                {#snippet child({ props })}
                  <a onclick={() => sidebar.toggle()} href={item.url} {...props}>
                    <item.icon />
                    <span>{item.title}</span>
                  </a>
                {/snippet}
              </Sidebar.MenuButton>
            </Sidebar.MenuItem>
          {/each}
        </Sidebar.Menu>
      </Sidebar.GroupContent>
    </Sidebar.Group>
    <Sidebar.Group>
      <Sidebar.GroupLabel>Meny</Sidebar.GroupLabel>
      <Sidebar.GroupContent>
        <Sidebar.Menu>
          {#each menuItems as item (item.title)}
            <Sidebar.MenuItem>
              <Sidebar.MenuButton>
                {#snippet child({ props })}
                  <a onclick={() => sidebar.toggle()} href={item.url} {...props}>
                    <item.icon />
                    <span>{item.title}</span>
                  </a>
                {/snippet}
              </Sidebar.MenuButton>
            </Sidebar.MenuItem>
          {/each}
        </Sidebar.Menu>
      </Sidebar.GroupContent>
    </Sidebar.Group>
  </Sidebar.Content>
  <Sidebar.Footer class="bg-popover">
    <DarkModeToggle/>
  </Sidebar.Footer>
</Sidebar.Root>
