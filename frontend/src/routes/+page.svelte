<script lang="ts">
  import type { PageProps } from "./$types";
  import { Button } from "$lib/components/ui/button/index.js";

  let { data }: PageProps = $props();
</script>

<h1>Home library lib</h1>
<span class="iconify mingcute--menu-line"></span>
{#each data.books as book}
  <p>{book.title}</p>
{/each}
 
<Button>Click me</Button>

