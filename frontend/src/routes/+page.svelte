<script lang="ts">
  import type { PageProps } from "./$types";
  let { data }: PageProps = $props() ;

  import QuickItemAddButton from "$lib/components/QuickItemAddButton.svelte";
  import placeHolderImage from "$lib/assets/placeholder_image.webp";
  import { getCoverImage } from "./../lib/utils";
  let books = data.books as any[];
  let bookCovers = $state(Object.fromEntries(books.map(book => [book.isbn, placeHolderImage])));
  books.forEach(async (book) => {
    bookCovers[book.isbn] = await getCoverImage(book.isbn);
  });
</script>

<QuickItemAddButton/>

<h2 class="scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight transition-colors first:mt-0">
  Alla föremål
</h2>

<div class="grid auto-rows-min gap-4 grid-cols-2 md:grid-cols-3 xl:grid-cols-5 p-4">
  {#each data.books as book} 
    <div class="flex hover:shadow-md flex-col bg-muted/50 p-3 gap-3 rounded-md">
      <a class="flex justify-center" href="/book/{book.isbn}" >
        <img src="{bookCovers[book.isbn]}" alt="{book.title} book cover"
        class="rounded-md h-48">
      </a>
      <div class="flex flex-col">
        <a href="/book/{book.isbn}"><h4 class="line-clamp-2 scroll-m-20 text-xl font-semibold tracking-tight">
        {book.title}</h4></a>
        <div class="flex justify-start">
          {#each book.authors as author}
            <p>{author}</p>
          {/each}
        </div>
      </div>
 </div>
  {/each}
</div>


