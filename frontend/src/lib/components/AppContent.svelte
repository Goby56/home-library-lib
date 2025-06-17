<script lang="ts">
  let { children, leftMargin = null, rightMargin = null } = $props();

</script>

<div class="flex justify-between content-center mt-20 mx-8">
  <div class="flex-col w-0 md:w-1/4">
    {@render leftMargin?.()} 
  </div>
  
  <div class="flex w-full flex-col px-3">
    {@render children?.()}
  </div>
  
  <div class="flex-col w-0 md:w-1/4">
    {@render rightMargin?.()} 
  </div>
</div>

