<script lang="ts">
  import Button, { buttonVariants } from "$lib/components/ui/button/button.svelte";
  import * as Drawer from "$lib/components/ui/drawer/index.js";
  import Input from "$lib/components/ui/input/input.svelte";
  import HashIcon from "@lucide/svelte/icons/hash";
  import PencilLineIcon from "@lucide/svelte/icons/pencil-line";
  import ScanBarcodeIcon from "@lucide/svelte/icons/scan-barcode";

  let isbn = $state("")
</script>

<Drawer.Root>
  <Drawer.Trigger class="flex justify-center">
    <div class="md:hidden flex bg-background justify-center w-full pb-3 px-5 pt-2 rounded-t-md fixed bottom-0">
      <Button class="w-full text-lg font-semibold">
        Lägg till bok
      </Button>
    </div>
  </Drawer.Trigger>
  <Drawer.Content>
    <Drawer.Header>
      <Drawer.Title>Lägg till bok</Drawer.Title>
    </Drawer.Header>
    <div class="flex flex-col gap-3 justify-center items-start h-full px-5">
      <div>
        <p class="font-semibold">Från ISBN</p>
        <div class="flex gap-3">
          <Input type="search" id="isbn" bind:value={isbn} maxlength={13} placeholder="ex. 9780141036144"/>
          <Button href="/add?isbn={isbn}">Sök</Button>
        </div>
      </div>
      <div class="flex gap-3 flex-wrap">
        <Button href="/api/scan?redirect=true">
          <ScanBarcodeIcon/>
          Scanna in en bok
        </Button>
        <Button href="/add">
          <PencilLineIcon/>
          Manuell inmatning
        </Button>
      </div>
      <Drawer.Footer class="w-full bottom-0">
        <Drawer.Close class={buttonVariants({ variant: "outline" })}>
          Tillbaka
        </Drawer.Close>
      </Drawer.Footer>
    </div>
  </Drawer.Content>
</Drawer.Root>
