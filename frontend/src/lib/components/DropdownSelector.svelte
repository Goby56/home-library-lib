<script lang="ts">
  import Check from "@lucide/svelte/icons/check";
  import ChevronsUpDown from "@lucide/svelte/icons/chevrons-up-down";
  import { tick } from "svelte";
  import * as Command from "$lib/components/ui/command/index.js";
  import * as Popover from "$lib/components/ui/popover/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import { cn } from "$lib/utils.js";
   
  let { name, value = $bindable(""), items, translations }: 
    { 
      name: string,
      value: string, 
      items: { 
        value: string, 
        label: string
      }[],
      translations: {
        choose: string,
        search: string,
        notFound: string,
      }
    } = $props()
 
  let open = $state(false);
  let triggerRef = $state<HTMLButtonElement>(null!);
 
  const selectedValue = $derived(
    items.find((item) => item.value === value)?.label
  );
 
  // We want to refocus the trigger button when the user selects
  // an item from the list so users can continue navigating the
  // rest of the form with the keyboard.
  function closeAndFocusTrigger() {
    open = false;
    tick().then(() => {
      triggerRef.focus();
    });
  }
</script>
 
<Popover.Root bind:open>
  <Popover.Trigger bind:ref={triggerRef}>
    {#snippet child({ props })}
      <Button
        variant="outline"
        class="justify-between"
        {...props}
        role="combobox"
        aria-expanded={open}
      >
        {selectedValue || translations.choose }
        <ChevronsUpDown class="opacity-50" />
      </Button>
    {/snippet}
  </Popover.Trigger>
  <input hidden value={value} name={name} />
  <Popover.Content class="w-[200px] p-0">
    <Command.Root>
      <Command.Input placeholder={translations.search} />
      <Command.List>
        <Command.Empty class="p-3">{translations.notFound}</Command.Empty>
        <Command.Group>
          {#each items as item (item.value)}
            <Command.Item
              value={item.value}
              onSelect={() => {
                value = item.value;
                closeAndFocusTrigger();
              }}
            >
              <Check
                class={cn(value !== item.value && "text-transparent")}
              />
              {item.label}
            </Command.Item>
          {/each}
        </Command.Group>
      </Command.List>
    </Command.Root>
  </Popover.Content>
</Popover.Root>
