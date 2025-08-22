<script lang="ts">
  import "./../../app.css"
  import AppHeader from "./AppHeader.svelte";
  import AppContent from "./AppContent.svelte";

  import InfoIcon from "@lucide/svelte/icons/info";
  import UserIcon from "@lucide/svelte/icons/user-round";
  import PencilLineIcon from "@lucide/svelte/icons/pencil-line";
  import HashIcon from "@lucide/svelte/icons/hash";
  import ContainerIcon from "@lucide/svelte/icons/container";
  import LibraryIcon from "@lucide/svelte/icons/library";
  import GalleryHorizontalEnd from "@lucide/svelte/icons/gallery-horizontal-end";

  import type { LayoutProps } from './$types';
  import Button from "$lib/components/ui/button/button.svelte";
  import IsbnDialog from "$lib/components/IsbnDialog.svelte";

	let { data, children }: LayoutProps = $props();

  let isbnDialogOpen = $state(false)
</script>

<AppHeader user={data.user}/>
<AppContent>
  <IsbnDialog bind:isbnDialogOpen/>
  {#snippet leftMargin()}
    <div class="flex flex-col gap-2">
      <p class="text-muted-foreground">Meny</p>
      <div class="flex flex-col justify-start">
        <Button href="/profile" variant="ghost" class="justify-start p-2 h-fit">
          <UserIcon/>
          Min profil
        </Button> 
        <Button href="/" variant="ghost" class="justify-start p-2 h-fit">
          <LibraryIcon/>
          Samling
        </Button> 
        <Button href="/shelves" variant="ghost" class="justify-start p-2 h-fit">
          <ContainerIcon/>
          Bokhyllor
        </Button> 
        <Button href="/help" variant="ghost" class="justify-start p-2 h-fit">
          <InfoIcon/>
          Information
        </Button> 
      </div>
    </div>
    <div class="flex flex-col gap-2">
      <p class="text-muted-foreground">Lägg till bok</p>
      <div class="flex flex-col justify-start">
        <Button onclick={() => isbnDialogOpen = true} variant="ghost" class="justify-start p-2 h-fit">
          <HashIcon/>
          Från ISBN
        </Button> 
        <Button href="/add" variant="ghost" class="justify-start p-2 h-fit">
          <PencilLineIcon/>
          Manuell inmatning
        </Button> 
        <Button href="/add" variant="ghost" class="justify-start p-2 h-fit">
          <GalleryHorizontalEnd/>
          Bulk inskanning
        </Button> 
      </div>
    </div>
  {/snippet}
  {@render children?.()}
</AppContent>
