<script lang="ts">
  import type { PageProps } from "./$types.js";
  import BookForm from "$lib/components/book-form/book-form.svelte";
    import { goto, invalidateAll } from "$app/navigation";
    import Button from "$lib/components/ui/button/button.svelte";
  let { data, form }: PageProps = $props();
  
  let pendingDeletion = $state(false);
  async function deleteBook(event: any) {
    event.stopPropagation();
    pendingDeletion = true;
    let response = await fetch("/api/book-operations/delete-book?uuid=" + data.uuid, { method: "POST" });
    console.log(await response.text());
    pendingDeletion = false;
    goto("/")
  }

</script>

<h2
  class="scroll-m-20 pb-2 text-3xl font-semibold tracking-tight transition-colors first:mt-0"
>
Redigera bok
</h2>

<BookForm {data} edit={true} />

<div>
  <Button variant="destructive" onclick={deleteBook}>Ta bort bok</Button>
</div>
