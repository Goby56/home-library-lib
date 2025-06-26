<script lang="ts">
  import Plus from "@lucide/svelte/icons/plus";
  import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index.js";
  import * as Dialog from "$lib/components/ui/dialog/index.js";
  import Input from "./ui/input/input.svelte";
  import Button from "./ui/button/button.svelte";

  import ScanBarcode from "@lucide/svelte/icons/scan-barcode";
  import PencilLine from "@lucide/svelte/icons/pencil-line";
  import Hash from "@lucide/svelte/icons/hash";
  import IsbnDialog from "./IsbnDialog.svelte";

  let isbnDialogOpen = $state(false)
</script>

<IsbnDialog bind:isbnDialogOpen/>

<div class="fixed md:hidden z-50 right-10 bottom-10 m-5 bg-primary rounded-3xl">
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
