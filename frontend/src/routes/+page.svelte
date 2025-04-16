<script lang="ts">
  import type { PageProps } from "./$types";
  let { data }: PageProps = $props();
</script>

<h1>Home library lib</h1>
{#each data.books as book}
  <p>{book.title}</p>
{/each}

