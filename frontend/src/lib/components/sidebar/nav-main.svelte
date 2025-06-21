<script lang="ts">
  import type { Component } from "svelte";
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  let {
    items,
  }: {
    items: {
      title: string;
      url: string;
      icon: Component;
    }[];
  } = $props();
</script>
<Sidebar.Group>
  <Sidebar.Menu>
    {#each items as item (item.title)}
      <Sidebar.MenuItem>
        <Sidebar.MenuButton>
          <a href={item.url} class="flex items-center gap-2">
            <item.icon />
            <span>{item.title}</span>
          </a>
        </Sidebar.MenuButton>
      </Sidebar.MenuItem>
    {/each}
  </Sidebar.Menu>
</Sidebar.Group>
