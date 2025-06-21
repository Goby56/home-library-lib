<script lang="ts">
  import type { Component, ComponentProps } from "svelte";
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import NavMain from "./nav-main.svelte";
  import NavUser from "./nav-user.svelte";
  import AppLogo from "$lib/assets/rose_icon.svelte";
  import Icon from "../ui/typography/icon.svelte";
  
  let { menuItems, ref = $bindable(null), ...restProps }: { menuItems: { title: string, url: string, icon: Component<Icon> }[]} 
    & ComponentProps<typeof Sidebar.Root> = $props();
</script>
<Sidebar.Root variant="floating" class="mt-12" {...restProps}>
  <Sidebar.Header>
    <Sidebar.Menu>
      <Sidebar.MenuItem>
        <Sidebar.MenuButton size="lg">
          {#snippet child({ props })}
            <a href="/" {...props}>
              <div
                class="text-primary flex aspect-square size-8 items-center justify-center rounded-lg"
              >
              <AppLogo/>
              </div>
              <div class="grid flex-1 text-left text-lg leading-tight">
                <span class="truncate font-medium">Rosenport</span>
                <span class="truncate text-xs">Hemma bibliotek</span>
              </div>
            </a>
          {/snippet}
        </Sidebar.MenuButton>
      </Sidebar.MenuItem>
    </Sidebar.Menu>
  </Sidebar.Header>
  <Sidebar.Content>
    <NavMain items={menuItems} />
  </Sidebar.Content>
  <Sidebar.Footer>
  <!--
    <NavUser/>
  -->
  </Sidebar.Footer>
</Sidebar.Root>
