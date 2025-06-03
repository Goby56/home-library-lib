<script lang="ts">
    import Button from '$lib/components/ui/button/button.svelte';
    import { Badge } from "$lib/components/ui/badge/index.js";
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();

  let book = data.book, book_copies = data.copies, coverImage = data.cover;
  
  console.log(data.copies)
  // let copyAvailableForReservation = data.copies.reduce()
</script>

<div class="flex flex-col">
    <div class="grid md:grid-cols-2 grid-cols-1 gap-3">
      <div class="flex justify-center">
        <img src="{coverImage}" alt="book cover"
        class="rounded-xl h-96">
      </div>
      <div class="flex flex-col gap-3">
        <h1 class="scroll-m-20 text-4xl font-extrabold tracking-tight lg:text-5xl">
        {book.title}
        </h1>
        <div class="flex">
          {#each book.authors as author, i (author)}
            <span>{author}{i < book.authors.length - 1 ? ', ' : ''}</span>
          {/each}
        </div>
        <div class="flex md:flex-row flex-col gap-5 items-center">
          <Button>Reservera</Button>
          <div class="border-l-2 h-3/4 border-muted-foreground hidden md:flex"></div>
          <div class="flex justify-center gap-1">
            {#each ["RVA8", "RKB4", "RX13"] as copy}
              <Badge class="rounded-md" variant="outline">{copy}</Badge>
            {/each}
          </div>
        </div>

        <h2 class="scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight transition-colors first:mt-0">
          Information
        </h2>
        <div class="grid grid-cols-2 gap-y-3 p-3 h-fit w-fit rounded-sm border-muted-foreground border-2">
          <div class="flex flex-row items-center">
            <p><b>Publicerad:</b> {book.publication_year}</p>
          </div>
          <div class="flex flex-row items-center">
            <p><b>Språk:</b> {book.language}</p>
          </div>
          <div class="flex flex-row items-center">
            <p><b>Antal sidor:</b> {book.page_count}</p>
          </div>
          <div class="flex flex-row items-center">
            <p><b>ISBN:</b> {book.isbn}</p>
          </div>
        </div>
        <h2 class="scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight transition-colors first:mt-0">
          Kategori
        </h2>
        {#each book.genres as genre, i (genre)}
          <span>{genre}{i < book.genres.length - 1 ? ', ' : ''}</span>
        {/each}
      </div>
    </div>
</div>


