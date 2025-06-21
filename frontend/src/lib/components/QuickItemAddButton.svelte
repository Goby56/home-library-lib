<script lang="ts">
  import Plus from "@lucide/svelte/icons/plus";
  import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index.js";
  import * as Dialog from "$lib/components/ui/dialog/index.js";
  import Input from "./ui/input/input.svelte";
  import Button from "./ui/button/button.svelte";

  import ScanBarcode from "@lucide/svelte/icons/scan-barcode";
  import PencilLine from "@lucide/svelte/icons/pencil-line";
  import Hash from "@lucide/svelte/icons/hash";

  let isbnDialogOpen = $state(false)
  let isbn = $state("")
</script>

<div class="fixed right-10 bottom-10 m-5 bg-primary hover:bg-primary/90 rounded-3xl">
  <DropdownMenu.Root>
    <DropdownMenu.Trigger class="flex p-3">
      <Plus class="text-background"/>
    </DropdownMenu.Trigger>
    <DropdownMenu.Content class="w-56 mx-10">
      <DropdownMenu.Group>
        <DropdownMenu.GroupHeading>Lägg till föremål</DropdownMenu.GroupHeading>
        <DropdownMenu.Separator />
        <DropdownMenu.Item>
          <ScanBarcode/>
          <a href="/api/scan?redirect=true">Scanna in en bok</a>
        </DropdownMenu.Item>
        <DropdownMenu.Item onclick={() => (isbnDialogOpen = true)}> 
            <Hash/>
            <p>Bok från ISBN</p>
        </DropdownMenu.Item>
        <DropdownMenu.Item>
          <PencilLine/>
          <a href="/add">Manuell inmatning</a>        
        </DropdownMenu.Item>
      </DropdownMenu.Group>
    </DropdownMenu.Content>
  </DropdownMenu.Root>
</div>

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
