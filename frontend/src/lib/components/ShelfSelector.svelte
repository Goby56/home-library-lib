<script lang="ts">
  import Button from "$lib/components/ui/button/button.svelte";
  import * as Popover from "$lib/components/ui/popover/index.js";
  import LoaderCircleIcon from "@lucide/svelte/icons/loader-circle";
  import { invalidateAll } from "$app/navigation";

  import CheckIcon from "@lucide/svelte/icons/check";
  import ChevronsUpDownIcon from "@lucide/svelte/icons/chevrons-up-down";
  import { tick } from "svelte";
  import * as Command from "$lib/components/ui/command/index.js";
  import { cn } from "$lib/utils.js";
  import type { Snippet } from "svelte";
  import { MediaQuery } from "svelte/reactivity";
  import * as Dialog from "$lib/components/ui/dialog/index.js";
    import { enhance } from "$app/forms";

  let {
    value = $bindable(),
    shelves,
    actionTrigger,
    noShelfSelected,
  }: {
    value: string;
    shelves: string[];
    actionTrigger: Snippet;
    noShelfSelected: Snippet;
  } = $props();

  let shelfPopupOpen = $state(false);
  let triggerRef = $state<HTMLButtonElement>(null!);

  // We want to refocus the trigger button when the user selects
  // an item from the list so users can continue navigating the
  // rest of the form with the keyboard.
  function closeAndFocusTrigger() {
    shelfPopupOpen = false;
    tick().then(() => {
      triggerRef.focus();
    });
  }

  let pendingAction = $state(false);

  const isDesktop = new MediaQuery("(min-width: 768px)");
</script>

<form method="POST" use:enhance={() => {
    pendingAction = true;
    return async ({ update }) => {
      await update();
      pendingAction = false;
    }

  }}>
  {#if isDesktop.current}
    <Popover.Root bind:open={shelfPopupOpen}>
      <Popover.Trigger bind:ref={triggerRef}>
        {#snippet child({ props })}
          <div class="flex">
            <Button
              {...props}
              class={value ? "rounded-r-none" : ""}
              role="combobox"
              variant="secondary"
              aria-expanded={shelfPopupOpen}
            >
              {#if value}
                {value}
                <ChevronsUpDownIcon />
              {:else}
                {@render noShelfSelected()}
              {/if}
            </Button>
            {#if value}
              {#if pendingAction}
                <Button disabled variant="outline" class="rounded-l-none">
                  <LoaderCircleIcon class="animate-spin" />
                </Button>
              {:else}
                {@render actionTrigger()}
              {/if}
            {/if}
          </div>
        {/snippet}
      </Popover.Trigger>
      <Popover.Content class="w-[200px] p-0">
        <Command.Root>
          <Command.Input placeholder="Sök efter bokhyllor..." />
          <Command.List>
            <Command.Empty class="flex flex-col p-1 gap-1">
              <p>Bokhyllan hittades inte</p>
            </Command.Empty>
            <Command.Group class="p-0" value="shelves">
              {#each shelves as shelf}
                <Command.Item
                  value={shelf}
                  onSelect={() => {
                    value = shelf;
                    closeAndFocusTrigger();
                  }}
                >
                  <CheckIcon class={cn(value !== shelf && "text-transparent")} />
                  {shelf}
                </Command.Item>
              {/each}
            </Command.Group>
          </Command.List>
        </Command.Root>
      </Popover.Content>
    </Popover.Root>
  {:else}
    <Dialog.Root bind:open={shelfPopupOpen}>
      <div class="flex">
        <Dialog.Trigger>
          <Button
            class={value ? "rounded-r-none" : ""}
            role="combobox"
            variant="secondary"
            aria-expanded={shelfPopupOpen}
          >
            {#if value}
              {value}
              <ChevronsUpDownIcon />
            {:else}
              {@render noShelfSelected()}
            {/if}
          </Button>
        </Dialog.Trigger>
        {#if value}
          {#if pendingAction}
            <Button disabled variant="outline" class="rounded-l-none">
              <LoaderCircleIcon class="animate-spin" />
            </Button>
          {:else}
            {@render actionTrigger()}
          {/if}
        {/if}
      </div>
      <Dialog.Content class="w-11/12 rounded-md">
        <Command.Root>
          <Command.Input placeholder="Sök efter bokhyllor..." />
          <Command.List>
            <Command.Empty class="flex flex-col p-1 gap-1">
              <p>Bokhyllan hittades inte</p>
            </Command.Empty>
            <Command.Group class="p-0" value="shelves">
              {#each shelves as shelf}
                <Command.Item
                  value={shelf}
                  onSelect={() => {
                    value = shelf;
                    closeAndFocusTrigger();
                  }}
                >
                  <CheckIcon class={cn(value !== shelf && "text-transparent")} />
                  {shelf}
                </Command.Item>
              {/each}
            </Command.Group>
          </Command.List>
        </Command.Root>
      </Dialog.Content>
    </Dialog.Root>
  {/if}
</form>
