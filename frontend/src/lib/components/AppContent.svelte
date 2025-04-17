<script lang="ts">
  let { children } = $props();

</script>

<div class="flex content-center mt-10 md:mx-36 mx-8 flex-col">
  {@render children?.()}
</div>
