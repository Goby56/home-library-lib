<script lang="ts">
  import type { PageData } from "./$types.js";
  import BookForm from "./book-form.svelte";
  let { data }: { data: PageData } = $props();
</script>
 
<BookForm {data} />
