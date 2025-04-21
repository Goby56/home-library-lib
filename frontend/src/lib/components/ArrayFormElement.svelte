<script lang="ts">
  import Button from "$lib/components/ui/button/button.svelte";
  import X from "@lucide/svelte/icons/x";
  import Plus from "@lucide/svelte/icons/plus";
  import * as Form from "$lib/components/ui/form/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import { tick } from "svelte";
  
  let { array = $bindable(), form, elementName, label } : 
    { 
      array: any[], 
      form: any, 
      elementName: string,
      label: string
    } = $props()
  
  let arrayInternal = $state(array);

  $effect(() => {
    array = arrayInternal;
  })


  function addEmpty() {
    arrayInternal = [...arrayInternal, ""];
    tick().then(() => { // Wait for after dom has updated
      const inputFields = Array.from(document.querySelectorAll<HTMLElement>(`#book-form input[name='${elementName}']`));
      inputFields[inputFields.length - 1].focus() // Focus last author input field
    })
  }

  function removeAtIndex(index: number) {
    if (arrayInternal.length == 1) {
      return
    }
    arrayInternal = [
      ...arrayInternal.slice(0, index),
      ...arrayInternal.slice(index + 1, undefined)
    ];
  }

</script>

<div>
	<Form.Fieldset {form} name={elementName}>
		<Form.Legend class="text-base">{ label }</Form.Legend>
		{#each arrayInternal as _, i}
			<Form.ElementField {form} name="{elementName}[{i}]">
				<Form.Control> 
          {#snippet children({ props })}
            <div class="flex gap-3">
              <Input {...props} bind:value={arrayInternal[i]} />
              {#if i == arrayInternal.length - 1}
                <Button
                	type="button"
                  onclick={() => addEmpty()}
                	variant="ghost"
                	size="icon"
                ><Plus/></Button>
              {/if}
              <Button
              	type="button"
                onclick={() => removeAtIndex(i)}
              	variant="ghost"
              	size="icon"
              ><X/></Button>
            </div>
          {/snippet}
				</Form.Control>
				<Form.FieldErrors />
			</Form.ElementField>
		{/each}
	</Form.Fieldset>
</div>
