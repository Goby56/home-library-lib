<script lang="ts">
  import type { PageProps } from "./$types";
  let { data }: PageProps = $props();

  import QuickItemAddButton from "$lib/components/QuickItemAddButton.svelte";
</script>

<QuickItemAddButton/>

<h2 class="scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight transition-colors first:mt-0">
  Reserverade föremål
</h2>

<div class="flex flex-1 flex-col gap-4 p-4">
	<div class="grid auto-rows-min gap-4 md:grid-cols-3">
		<div class="bg-muted/50 aspect-video rounded-xl"></div>
		<div class="bg-muted/50 aspect-video rounded-xl"></div>
		<div class="bg-muted/50 aspect-video rounded-xl"></div>
	</div>
</div>

<h2 class="scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight transition-colors first:mt-0">
  Alla föremål
</h2>

<div class="flex flex-1 flex-col gap-4 p-4">
	<div class="grid auto-rows-min gap-4 md:grid-cols-6">
		<div class="bg-muted/50 aspect-video rounded-xl"></div>
		<div class="bg-muted/50 aspect-video rounded-xl"></div>
		<div class="bg-muted/50 aspect-video rounded-xl"></div>
	</div>
</div>

{#each data.books as book}
  <p>{book.title}</p>
{/each}
