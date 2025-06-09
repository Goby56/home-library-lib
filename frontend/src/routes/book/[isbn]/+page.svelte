<script lang="ts">
	import type { PageProps } from './$types';
  import { MediaQuery } from "svelte/reactivity";
  import BookInfoDesktop from './BookInfoDesktop.svelte';
    import BookInfoMobile from './BookInfoMobile.svelte';
 
	let { data }: PageProps = $props();

  let coverImage = data.cover;
  const isDesktop = new MediaQuery("(min-width: 768px)");
</script>

<div class="flex flex-col">
    <div class="grid md:grid-cols-2 grid-cols-1 gap-3">
      <div class="flex justify-center">
        <img src="{coverImage}" alt="book cover"
        class="rounded-xl h-96">
      </div>
      {#if isDesktop.current}
        <BookInfoDesktop data={data}/>
      {:else}
        <BookInfoMobile data={data}/>
      {/if}
    </div>
</div>


