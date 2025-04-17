<script>
  import User from "@lucide/svelte/icons/user";
  import QuickActionMenu from "./QuickActionMenu.svelte";
</script>

<div class="w-full px-4 py-3 flex items-center justify-between">
  <div class="flex items-center gap-3 z-10">
    <h2 class="scroll-m-20 text-3xl font-semibold tracking-tight transition-colors first:mt-0">
      Home library lib
    </h2>
  </div>
  <div class="flex gap-8">
    <div class="hidden md:flex">
      <QuickActionMenu/>
    </div>
    <div class="flex items-center z-10">
      <User/>
    </div>
  </div>
</div>
