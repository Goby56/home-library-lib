<script lang="ts">
  import { onMount } from "svelte";
  import Input from "./ui/input/input.svelte";
  import SearchIcon from "@lucide/svelte/icons/search";
  import { afterNavigate, goto } from "$app/navigation";
    import { MediaQuery } from "svelte/reactivity";
    import Button from "./ui/button/button.svelte";
  
  let { value = $bindable(), searchSuggestionsOpen = $bindable() } = $props();

  const isDesktop = new MediaQuery("(min-width: 768px)");

  let searchSuggestions: any[] = $state([])

  async function getSuggestions() {
    const response = await fetch("/api/get-search-suggestions?search=" + value, { method: "GET" });
    if (response.ok) {
      searchSuggestions = await response.json();
    }
  }

  async function onKeyPress(e: KeyboardEvent) {
    if (e.key === "Enter") {
      goto("/?search=" + value)
    }
  }

  $effect(() => {
    if (searchSuggestionsOpen) {
      getSuggestions();
    }
  })

  let container: HTMLDivElement;
  
  function handleClickOutside(event: MouseEvent) {
    if (!container.contains(event.target as Node)) {
      searchSuggestionsOpen = false;
    }
  }

  onMount(() => {
    document.addEventListener("mousedown", handleClickOutside);
    return () => document.removeEventListener("mousedown", handleClickOutside);
  });

  $effect(() => {
    if (searchSuggestionsOpen && !isDesktop.current) {
      document.body.classList.add("overflow-hidden");
    } else {
      document.body.classList.remove("overflow-hidden");
    }
  })

  afterNavigate(() => {
    searchSuggestionsOpen = false;
  })

</script>

{#snippet inputFieldSnippet()}
  <Input 
    oninput={getSuggestions}
    onkeypress={onKeyPress}
    onfocus={() => searchSuggestionsOpen = true} 
    type="search" 
    bind:value 
    placeholder="Sök efter böcker" 
    class="pl-10 w-full rounded-3xl focus-visible:ring-0 focus-visible:ring-offset-0 focus:border-primary"/>
{/snippet}

{#snippet searchSuggestionsSnippet()}
  <div class="flex flex-col absolute top-full w-full bg-secondary rounded-md mt-1 p-2">
    {#each searchSuggestions as suggestion}
      <a href="/book/{suggestion.isbn ?? suggestion.uuid}" class="flex gap-1 hover:bg-muted/50 rounded-sm p-1">
        <p><b>{suggestion.title}</b> {suggestion.authors}</p>
        <div class="flex gap-1">
          {#each suggestion.genres.split("\n") as genre }
            <span class="bg-muted size-fit relative rounded p-1 text-sm font-semibold">{genre}</span>
          {/each}
        </div>
      </a>
    {:else}
      <div class="hover:bg-muted rounded-sm p-1">
        {#if value}
          Hittade inga böcker 
        {:else}
          Sökfältet är tomt
        {/if}
      </div>
    {/each} 
  </div>
{/snippet}

<div bind:this={container} class="relative w-full flex">
  <div class="absolute flex items-center pl-3 w-full h-full pointer-events-none">
    <SearchIcon class="items-center size-5 text-muted-foreground"/>
  </div>
  {@render inputFieldSnippet()}
  {#if searchSuggestionsOpen}
    {#if !isDesktop.current}
      <Button variant="ghost" onclick={() => searchSuggestionsOpen = false} class="ml-1">Tillbaka</Button>  
    {/if}
    {@render searchSuggestionsSnippet()}
  {/if}
</div>
