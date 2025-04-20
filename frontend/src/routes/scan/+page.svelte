<script lang="ts">
  import type { PageProps } from "./$types";
  let { data }: PageProps = $props();
  import { onMount } from "svelte";

  import Button from "$lib/components/ui/button/button.svelte";
  import Pen from "@lucide/svelte/icons/pen";
  import * as Tooltip from "$lib/components/ui/tooltip/index.js";

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
<div class="flex-col w-fit bg-muted/50 rounded-xl p-3">
  <div class="flex">
    <div>
      <img src="{image}" alt="{data.book.title} book cover"
      class="rounded-xl h-48">
    </div>
    <div class="mx-3">
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
    <div class="flex flex-col justify-between">
      <div class="flex justify-end">
        <div class="flex justify-center items-center">
          <Tooltip.Provider>
            <Tooltip.Root>
              <Tooltip.Trigger>
                <Button type="button" variant="ghost" size="icon"><Pen/></Button>
              </Tooltip.Trigger>
              <Tooltip.Content>
                <p>Ändra beskrivning</p>
              </Tooltip.Content>
            </Tooltip.Root>
          </Tooltip.Provider>
        </div>
      </div>
      <Button>Lägg till bok</Button>
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


