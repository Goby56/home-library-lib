<script lang="ts">
  import Button from "$lib/components/ui/button/button.svelte";

  let { selectedCopy = $bindable(), physicalCopies } = $props();

  if (physicalCopies.length > 0) {
    selectedCopy = physicalCopies[0];
  }
  
</script>

<div class="flex justify-center flex-wrap gap-3">
  {#each physicalCopies as copy}
    <Button 
      onclick={() => selectedCopy = (copy.id == selectedCopy?.id ? "" : copy)} 
      variant={selectedCopy?.id == copy.id ? "default" : "secondary"}>
      {copy.shelf.name}</Button> 
  {/each}
</div>
