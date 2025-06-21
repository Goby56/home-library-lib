<script lang="ts">
 import * as Popover from "$lib/components/ui/popover/index.js";

 import Input from "./ui/input/input.svelte";
 import Search from "@lucide/svelte/icons/search";

</script>
 
<div class="relative w-full flex max-w-1/2">
  <div class="absolute flex items-center pl-3 w-full h-full pointer-events-none">
    <Search class="items-center size-5 text-muted-foreground"/>
  </div>
  <Input type="search" placeholder="Sök efter föremål" class="pl-10 w-full rounded-3xl"/>
</div>

<!--
<Popover.Root>
  <Popover.Trigger>
  </Popover.Trigger>
  <Popover.Content trapFocus={false}>Place content for the popover here.</Popover.Content>
</Popover.Root>
-->
