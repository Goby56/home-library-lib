<script lang="ts">
  import { onMount } from "svelte";
  import type { PageProps } from "./$types";
  let { data }: PageProps = $props();

  import placeHolderImage from "$lib/assets/placeholder_image.webp";
  
  // svelte-ignore non_reactive_update
  let image = placeHolderImage;
  if (data.book != null) {
    image = data.book.imageLinks != null ? data.book.imageLinks.thumbnail : placeHolderImage;
  }

  onMount(() => {
    console.log(data)
  })
</script>

{#if data.book != null}
<p class="text-muted-foreground text-xl pb-5">
Hittade följande bok...
</p>
<div class="flex-col bg-muted/50 rounded-xl p-3">
  <div class="flex">
    <div>
      <img src="{image}" alt="{data.book.title} book cover"
      class="rounded-xl mr-3 h-48">
    </div>
    <div>
      <h3 class="scroll-m-20 text-2xl font-semibold tracking-tight">
      { data.book.title }
      </h3>
      <span>
      { data.book.authors.join(", ") }
      </span>
      <ul class="my-6 ml-6 list-disc [&>li]:mt-2">
        <li>Publicerad: { data.book.publishedDate }</li>
        <li>Språk: { data.book.language }</li>
        <li>Antal sidor: { data.book.pageCount }</li>
      </ul>
    </div>
  </div>
  <div>
  <p class="leading-7 [&:not(:first-child)]:mt-6">
  { data.book.description }
  </p>
  </div>
</div>
{:else}  
<p class="text-muted-foreground text-xl pb-5">
Hittade ingen bok med ISBN { data.content }
</p>
{/if}


