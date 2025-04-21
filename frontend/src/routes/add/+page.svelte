<script lang="ts">
  import type { PageData } from "./$types.js";
  import BookForm from "./book-form.svelte";
  let { data }: { data: PageData } = $props();
</script>

<h2
  class="scroll-m-20 pb-2 text-3xl font-semibold tracking-tight transition-colors first:mt-0"
>
Lägg till en bok
</h2>
 
<BookForm {data} />
