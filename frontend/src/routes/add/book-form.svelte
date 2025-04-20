<script lang="ts">
  import Button from "$lib/components/ui/button/button.svelte";
  import X from "@lucide/svelte/icons/x";
  import * as Form from "$lib/components/ui/form/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import { tick } from "svelte";
  import { bookFormSchema, type FormSchema } from "./book-form-schema";
  import {
    type SuperValidated,
    type Infer,
    superForm,
  } from "sveltekit-superforms";
  import { zodClient } from "sveltekit-superforms/adapters";
 
  let { data }: { data: { form: SuperValidated<Infer<FormSchema>> } } =
    $props();
 
  const form = superForm(data.form, {
    validators: zodClient(bookFormSchema),
  });
 
  const { form: formData, enhance } = form;

  function addAuthor() {
    $formData.authors = [...$formData.authors, ""];
    tick().then(() => { // Wait for after dom has updated
      const authorInputs = Array.from(document.querySelectorAll<HTMLElement>("#book-form input[name='authors']"));
      authorInputs[authorInputs.length - 1].focus() // Focus last author input field
    })
  }

  function removeAuthor(index: number) {
    if ($formData.authors.length == 1) {
      return
    }
    $formData.authors = [
      ...$formData.authors.slice(0, index),
      ...$formData.authors.slice(index + 1, undefined)
    ];
  }

</script>
 
<form method="POST" use:enhance id="book-form">
  <Form.Field {form} name="title">
    <Form.Control>
      {#snippet children({ props })}
        <Form.Label>Titel</Form.Label>
        <Input {...props} bind:value={$formData.title} />
      {/snippet}
    </Form.Control>
    <Form.FieldErrors />
  </Form.Field>

  <div>
  	<Form.Fieldset {form} name="authors">
  		<Form.Legend>Författare</Form.Legend>
  		{#each $formData.authors as _, i}
  			<Form.ElementField {form} name="authors[{i}]">
  				<Form.Control> 
            {#snippet children({ props })}
              <div class="flex">
                <Input {...props} bind:value={$formData.authors[i]} />
                <Button
                	type="button"
                  onclick={() => removeAuthor(i)}
                	variant="ghost"
                	size="icon"
                  class="mx-2"
                ><X/></Button>
              </div>
            {/snippet}
  				</Form.Control>
  				<Form.FieldErrors />
  			</Form.ElementField>
  		{/each}
  	</Form.Fieldset>
  	<Button type="button" variant="outline" size="sm" class="mt-2" onclick={addAuthor}>
  		Lägg till fler författare
  	</Button>
  </div>

  <Form.Button>Lägg till bok</Form.Button>
</form>
