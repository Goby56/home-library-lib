<script lang="ts">
  import * as Command from "$lib/components/ui/command/index.js";
  import Button from "./ui/button/button.svelte";
  import { onMount } from "svelte";
 
  let open = $state(false);
 
  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "k" && (e.metaKey || e.ctrlKey)) {
      e.preventDefault();
      open = !open;
    }
  }
</script>
 
<svelte:document onkeydown={handleKeydown} />

<Button onclick={() => open = !open } variant="outline">
  Quick action CTRL-K
</Button>

 
<Command.Dialog bind:open>
  <Command.Input placeholder="Type a command or search..." />
  <Command.List>
    <Command.Empty>No results found.</Command.Empty>
    <Command.Group heading="Suggestions">
      <Command.Item>Calendar</Command.Item>
      <Command.Item>Search Emoji</Command.Item>
      <Command.Item>Calculator</Command.Item>
    </Command.Group>
  </Command.List>
</Command.Dialog>
