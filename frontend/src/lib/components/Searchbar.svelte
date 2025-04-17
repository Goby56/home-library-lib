<script lang="ts">
 import * as Popover from "$lib/components/ui/popover/index.js";

 import Input from "./ui/input/input.svelte";
 import Search from "@lucide/svelte/icons/search";

</script>
 
<div class="relative w-full flex mx-8 max-w-1/2">
  <div class="absolute flex items-center pl-3 w-full h-full">
    <Search class="items-center size-5 text-neutral-400"/>
  </div>
  <Input type="search" placeholder="Search for items" class="pl-10 w-full"/>
</div>

<!--
<Popover.Root>
  <Popover.Trigger>
  </Popover.Trigger>
  <Popover.Content trapFocus={false}>Place content for the popover here.</Popover.Content>
</Popover.Root>
-->
