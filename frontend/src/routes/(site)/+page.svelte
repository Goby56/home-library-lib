<script lang="ts">
  import type { PageProps } from "./$types";
  let { data }: PageProps = $props() ;

  import QuickItemAddButton from "$lib/components/QuickItemAddButton.svelte";
  import placeHolderImage from "$lib/assets/placeholder_image.webp";
  import AddBookDrawer from "./AddBookDrawer.svelte";

  async function getBookCover(uuid: string) {
		return await fetch('/api/get-book-cover?uuid=' + uuid, {
			method: 'GET',
		});
	}

</script>

<h2 class="scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight transition-colors first:mt-0">
  Alla böcker
</h2>

<div class="grid auto-rows-min gap-4 grid-cols-2 md:grid-cols-3 xl:grid-cols-5 py-2 md:p-4">
  {#each data.books as book} 
    <a href="/book/{book.isbn ?? book.uuid}" class="flex hover:scale-105  flex-col bg-muted/50 p-3 gap-3 rounded-md">
      <div class="flex justify-center" >
        {#await getBookCover(book.uuid)}
          <img src={placeHolderImage} alt="{book.title} book cover" class="rounded-md h-48">
        {:then coverImage}
          {#if coverImage.status == 200}
            <img src={coverImage.url} alt="{book.title} book cover" class="rounded-md h-48">
          {:else}
            <img src={placeHolderImage} alt="{book.title} book cover" class="rounded-md h-48">
          {/if}
        {:catch}
          <img src={placeHolderImage} alt="{book.title} book cover" class="rounded-md h-48">
        {/await}
      </div>
      <div class="flex flex-col">
        <h4 class="line-clamp-2 scroll-m-20 text-xl font-semibold tracking-tight">
          {book.title}
        </h4>
        <div class="flex gap-1">
          <span class="line-clamp-1">
            {#each book.authors as author, i (author)}
              {author}{i < book.authors.length - 1 ? ', ' : ''}
            {/each}
          </span>
        </div>
      </div>
    </a>
  {/each}
</div>

<AddBookDrawer/>
