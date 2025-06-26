<script lang="ts">
  import * as Dialog from "$lib/components/ui/dialog/index.js";
  import Input from "./ui/input/input.svelte";
  import Button from "./ui/button/button.svelte";

  let { isbnDialogOpen = $bindable() } = $props();
  let isbn = $state("")
</script>

<Dialog.Root bind:open={isbnDialogOpen}>
  <Dialog.Trigger></Dialog.Trigger>
  <Dialog.Content class="sm:max-w-[425px]">
    <Dialog.Header>
      <Dialog.Title>Bok från ISBN</Dialog.Title>
      <Dialog.Description>
        Mata in ISBN som finns på baksidan av din bok
      </Dialog.Description>
    </Dialog.Header>
    <div class="flex">
      <Input type="search" id="isbn" bind:value={isbn} placeholder="ISBN" class="mr-3" />
      <Button href="/add?isbn={isbn}">Sök</Button>
    </div>
  </Dialog.Content>
</Dialog.Root>
