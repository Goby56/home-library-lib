<script lang="ts">
  import Button from '$lib/components/ui/button/button.svelte';
  import PlusIcon from "@lucide/svelte/icons/plus";
  import axios from "axios";

  import { languageCodes, getLabelFromLanguageCode } from "$lib/utils";
  import PhysicalBookManagerButton from "$lib/components/PhysicalBookManagerButton.svelte";
  import ShelfSelector from '$lib/components/ShelfSelector.svelte';

	let { data } = $props();

  let shelves = $derived(data.shelves);

  let selectedShelf = $state("");

  async function addToShelf(shelf: string) {
      let physical_copy = {
          isbn: data.book.isbn, name: shelf
      }
      return axios.post("http://192.168.1.223:8080/add_physical_book", physical_copy);
  }
</script>


<div class="flex flex-col gap-3">
  <h1 class="scroll-m-20 text-4xl font-extrabold tracking-tight lg:text-5xl">
  {data.book.title}
  </h1>
  <div class="flex">
    {#each data.book.authors as author, i (author)}
      <span>{author}{i < data.book.authors.length - 1 ? ', ' : ''}</span>
    {/each}
  </div>
  <div class="flex flex-col gap-1">
    {#if data.copies.length == 0}
      <p class="text-muted-foreground text-sm">Denna bok tillhör ännu inte någon bokhylla</p>
    {:else}
      <p class="text-muted-foreground text-sm">Finns i följande bokhyllor:</p>
    {/if}
    <div class="flex gap-3 items-center">
      {#each data.copies as physical_copy}
        <PhysicalBookManagerButton book={data.book} physicalCopy={physical_copy} shelves={shelves}/>
      {/each}

      <ShelfSelector bind:value={selectedShelf} action={addToShelf} shelves={data.shelves}>
        {#snippet actionTrigger(performAction)}
          <Button onclick={performAction} class="rounded-l-none" size="icon"><PlusIcon/></Button>
        {/snippet}
        {#snippet noShelfSelected()}
          {#if data.copies.length == 0}
            Placera i bokylla
          {:else} 
            <PlusIcon/>
          {/if}
        {/snippet}
      </ShelfSelector>
    </div>
  </div>

  <h2 class="scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight transition-colors first:mt-0">
    Information
  </h2>
  <div class="grid grid-cols-2 gap-y-3 p-3 h-fit w-fit rounded-sm border-muted-foreground border-2">
    <div class="flex flex-row items-center">
      <p><b>Publicerad:</b> {data.book.publication_year}</p>
    </div>
    <div class="flex flex-row items-center">
      <p><b>Språk:</b> {getLabelFromLanguageCode(data.book.language)}</p>
    </div>
    <div class="flex flex-row items-center">
      <p><b>Antal sidor:</b> {data.book.page_count}</p>
    </div>
    <div class="flex flex-row items-center">
      <p><b>ISBN:</b> {data.book.isbn}</p>
    </div>
  </div>
  <h2 class="scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight transition-colors first:mt-0">
    Kategori
  </h2>
  {#each data.book.genres as genre, i (genre)}
    <span>{genre}{i < data.book.genres.length - 1 ? ', ' : ''}</span>
  {/each}
</div>
