<script lang="ts">
  import Button from "$lib/components/ui/button/button.svelte";
  import { Separator } from "$lib/components/ui/separator/index.js";
  import SuperDebug from "sveltekit-superforms";
  import * as Form from "$lib/components/ui/form/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import { bookFormSchema, type FormSchema } from "./book-form-schema";
  import { languageCodes } from "$lib/utils";
  import {
    type SuperValidated,
    type Infer,
    superForm, fileProxy
  } from "sveltekit-superforms";
  import { zodClient } from "sveltekit-superforms/adapters";
  import * as Tooltip from "$lib/components/ui/tooltip/index.js";
  import DropdownSelector from "$lib/components/DropdownSelector.svelte";
  import ArrayFormElement from "$lib/components/ArrayFormElement.svelte";
  import placeHolderImage from "$lib/assets/placeholder_image.webp";
    import imageCompression from "browser-image-compression";
 
  let { data }: { data: { form: SuperValidated<Infer<FormSchema>> } } =
    $props();
 
  const form = superForm(data.form, {
    validators: zodClient(bookFormSchema),
  });
 
  const { form: formData, enhance, errors } = form;

  const coverImageFile = fileProxy(form, 'cover')
  let coverImageURL = $state(placeHolderImage);
  let pendingCompression = $state(false);

  function onCoverImageChange(event: Event) {
    const files = (event.target as HTMLInputElement).files;
    if (files && files.length > 0) {
      const originalImage = files[0];
      coverImageURL = URL.createObjectURL(originalImage)
      pendingCompression = true;
      imageCompression(originalImage, {
          maxSizeMB: 1,
          maxWidthOrHeight: 1920,
          useWebWorker: true,
      }).then(compressed => {
          coverImageFile.set(compressed)
          coverImageURL = URL.createObjectURL(compressed)
          pendingCompression = false;
      })
    }
  }
</script>
 
<form method="POST" use:enhance id="book-form" enctype="multipart/form-data" class="flex flex-col gap-3 p-3">
  <Form.Field {form} name="isbn">
    <Form.Control>
      {#snippet children({ props })}
        <div class="flex gap-3">
          <Input {...props} placeholder="ISBN" bind:value={$formData.isbn} />
          <Tooltip.Provider>
            <Tooltip.Root>
              <Tooltip.Trigger>
                <Button href="/add?isbn={$formData.isbn}">Sök efter bok</Button>
              </Tooltip.Trigger>
              <Tooltip.Content>
                <p>Fyll i bokens information automatiskt</p>
              </Tooltip.Content>
            </Tooltip.Root>
          </Tooltip.Provider>
        </div>
      {/snippet}
    </Form.Control>
    <Form.Description>
      En unik sträng siffror som identifierar din boks utgåva. Du kan oftast hitta den nedanför en streckkod på baksidan.
		</Form.Description>
    <Form.FieldErrors />
  </Form.Field> 

  <Separator/>
  
  <div class="flex justify-start flex-col gap-3 md:flex-row w-full">
    <div class="flex flex-col flex-grow gap-3">
      <div class="flex gap-5 w-full">
        <Form.Field {form} name="title">
          <Form.Control>
            {#snippet children({ props })}
              <Form.Label class="text-base">Titel</Form.Label>
              <Input {...props} placeholder="Bokens titel" bind:value={$formData.title} />
            {/snippet}
          </Form.Control>
        </Form.Field>

        <Form.Field {form} name="publication_year">
          <Form.Control>
            {#snippet children({ props })}
              <Form.Label class="text-base">Publicerad</Form.Label>
              <Input {...props} type="number" bind:value={$formData.publication_year} />
            {/snippet}
          </Form.Control>
        </Form.Field>
      </div>

      <ArrayFormElement bind:array={$formData.authors} form={form} elementName="authors" label="Författare"/>

      <div class="flex gap-5 w-full">
        <Form.Field {form} name="page_count">
          <Form.Control>
            {#snippet children({ props })}
              <Form.Label class="text-base">Antal sidor</Form.Label>
              <Input {...props} type="number" bind:value={$formData.page_count} />
            {/snippet}
          </Form.Control>
        </Form.Field>

        <Form.Field {form} name="language">
          <Form.Control>
            {#snippet children({ props })}
              <Form.Label class="text-base">Språk</Form.Label>
              <div>
                <DropdownSelector 
                  bind:value={$formData.language} 
                  items={languageCodes}
                  name="language"
                  translations={{
                    choose: "Välj ett språk...",
                    search: "Sök efter ett språk",
                    notFound: "Kunde inte hitta språket"
                  }}/>
              </div>
            {/snippet}
          </Form.Control>
          <Form.Description>
	      	</Form.Description>
        </Form.Field>
      </div>

      <ArrayFormElement bind:array={$formData.genres} form={form} elementName="genres" label="Genre"/>
    </div>
    <div class="flex flex-col items-center gap-3">
      <div>
        <img src="{coverImageURL}" alt="book cover"
        class="rounded-xl h-80">
      </div>
      <Button variant="outline" class="relative">
        Välj bokomslag
        <input class="absolute w-full opacity-0" type="file" name="cover" bind:files={$coverImageFile} oninput={onCoverImageChange} multiple={false} accept="image/webp, image/png, image/jpeg"/>
      </Button>
    </div>
  </div>

  <div class="flex md:justify-end justify-center">
  {#if pendingCompression}
    <Form.Button disabled>Lägg till bok</Form.Button>
  {:else}
    <Form.Button>Lägg till bok</Form.Button>
  {/if}
  </div>
</form>

<SuperDebug data={$formData} />
