<script lang="ts">
 import * as Popover from "$lib/components/ui/popover/index.js";

 import Input from "./ui/input/input.svelte";

</script>
 
<Popover.Root>
  <Popover.Trigger>
    <div class="absolute left-1/2 transform -translate-x-1/2 hidden md:flex justify-center w-full">
      <Input type="search" placeholder="Search for items" class="w-1/2 h-12"/>
    </div>
  </Popover.Trigger>
  <Popover.Content trapFocus={false}>Place content for the popover here.</Popover.Content>
</Popover.Root>
