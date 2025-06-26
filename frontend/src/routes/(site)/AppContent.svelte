<script lang="ts">
  let { children, leftMargin = null, rightMargin = null } = $props();

</script>

<div class="flex justify-between content-center pt-20 md:mx-8">
  <div class="hidden md:flex flex-col gap-3 w-0 md:w-1/4">
    {@render leftMargin?.()} 
  </div>
  
  <div class="flex w-full flex-col px-3">
    {@render children?.()}
  </div>
  
  <div class="hidden md:flex flex-col gap-3 w-0 md:w-1/4">
    {@render rightMargin?.()} 
  </div>
</div>

