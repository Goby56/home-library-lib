<script lang="ts">
  import Button from "$lib/components/ui/button/button.svelte";

  let { selectedCopyID = $bindable(), physicalCopies }: { selectedCopyID: any, physicalCopies: any[] } = $props();
  
</script>

<div class="flex justify-center flex-wrap gap-3">
  {#each physicalCopies as copy}
    <Button 
      onclick={() => selectedCopyID = selectedCopyID == copy.id ? undefined : copy.id} 
      variant={selectedCopyID == copy.id ? "default" : "secondary"}>
      {copy.shelf.name}</Button> 
  {/each}
</div>
