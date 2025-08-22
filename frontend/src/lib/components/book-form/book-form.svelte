<script lang="ts">
  import Button from "$lib/components/ui/button/button.svelte";
  import { Separator } from "$lib/components/ui/separator/index.js";
  import SuperDebug from "sveltekit-superforms";
  import * as Form from "$lib/components/ui/form/index.js";
  import ImagePlusIcon from "@lucide/svelte/icons/image-plus";
  import LoaderCircleIcon from "@lucide/svelte/icons/loader-circle";
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
  import { Textarea } from "$lib/components/ui/textarea/index.js";
  import imageCompression from "browser-image-compression";
  import { MediaQuery } from "svelte/reactivity";
  import { mode } from "mode-watcher";
 
  let { data, edit }: { data: { form: SuperValidated<Infer<FormSchema>>, coverURL: string }, 
    edit: boolean & any } = $props();
  
  const isDesktop = new MediaQuery("(min-width: 768px)");
 
  const form = superForm(data.form, {
    validators: zodClient(bookFormSchema),
  });
 
  const { form: formData, enhance, errors } = form;

  const coverImageFile = fileProxy(form, 'cover')
  let coverImageURL = $state(data.coverURL || placeHolderImage);
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
          {#if !edit}
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
          {/if}
        </div>
      {/snippet}
    </Form.Control>
    <Form.Description>
      En unik sträng siffror som identifierar din boks utgåva. Du kan oftast hitta den nedanför en streckkod på baksidan.
		</Form.Description>
    <Form.FieldErrors />
  </Form.Field> 

  <Separator/>

  {#snippet titleFieldSnippet()}
    <Form.Field {form} name="title">
      <Form.Control>
        {#snippet children({ props })}
          <Form.Label class="text-base">Titel</Form.Label>
          <Input {...props} placeholder="Bokens titel" bind:value={$formData.title} />
        {/snippet}
      </Form.Control>
    </Form.Field>
  {/snippet}

  {#snippet authorsFieldSnippet()}
    <Form.Field {form} name="authors">
      <Form.Control>
        {#snippet children({ props })}
          <Form.Label class="text-base">Författare</Form.Label>
          <Textarea {...props} placeholder={`George Orwell\nTolkien\n...`} bind:value={$formData.authors} />
        {/snippet}
      </Form.Control>
    </Form.Field>
  {/snippet}

  {#snippet genresFieldSnippet()}
    <Form.Field {form} name="genres">
      <Form.Control>
        {#snippet children({ props })}
          <Form.Label class="text-base">Genre</Form.Label>
          <Textarea {...props} placeholder={`Sci-fi\nRomance\n...`} bind:value={$formData.genres} />
        {/snippet}
      </Form.Control>
    </Form.Field>
  {/snippet}

  {#snippet pubYearFieldSnippet()}
    <Form.Field {form} name="publication_year">
      <Form.Control>
        {#snippet children({ props })}
          <Form.Label class="text-base">Publicerad</Form.Label>
          <Input {...props} placeholder="Årtal" type="number" bind:value={$formData.publication_year} />
        {/snippet}
      </Form.Control>
    </Form.Field>
  {/snippet}

  {#snippet pageCountFieldSnippet()}
    <Form.Field {form} name="page_count">
      <Form.Control>
        {#snippet children({ props })}
          <Form.Label class="text-base">Antal sidor</Form.Label>
          <Input {...props} type="number" bind:value={$formData.page_count} />
        {/snippet}
      </Form.Control>
    </Form.Field>
  {/snippet}

  {#snippet languageFieldSnippet()}
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
  {/snippet}

  <div class="flex flex-col gap-3">
    <div class="flex gap-3 justify-start">
      <div class="h-60 md:h-80 aspect-[2/3] relative group">
        <img src="{coverImageURL}" alt="book cover"
        class="rounded-xl object-contain w-full h-full">
        <input class="absolute top-0 w-full h-full opacity-0" type="file" name="cover" bind:files={$coverImageFile} oninput={onCoverImageChange} multiple={false} accept="image/webp, image/png, image/jpeg"/>
        <ImagePlusIcon class="bg-background/75 rounded-md p-1 group-hover:scale-110 pointer-events-none size-12 absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2"/>
      </div>
      
      <div class="flex flex-col gap-3 flex-1">
        {#if isDesktop.current}
          {@render titleFieldSnippet()}
          <div class="flex gap-3">
            {@render authorsFieldSnippet()}
            {@render genresFieldSnippet()}
          </div>
          <p class="text-muted-foreground">Ange ytterligare författare och genrer på nya rader</p>
          <div class="flex gap-3">
            {@render pubYearFieldSnippet()}
            {@render pageCountFieldSnippet()}
            {@render languageFieldSnippet()}
          </div>
        {:else}
          {@render pubYearFieldSnippet()}
          {@render pageCountFieldSnippet()}
          {@render languageFieldSnippet()}
        {/if}
      </div>
    </div>

    {#if !isDesktop.current}
      {@render titleFieldSnippet()}
      <div class="flex gap-3">
        {@render authorsFieldSnippet()}
        {@render genresFieldSnippet()}
      </div>
      <p class="text-muted-foreground">Ange ytterligare författare och genrer på nya rader</p>
    {/if}

  </div>
  
  <div class="flex justify-center">
  {#if pendingCompression}
    <Form.Button disabled>
      Komprimerar omslag
      <LoaderCircleIcon class="animate-spin"/>
    </Form.Button>
  {:else if edit}
    <Form.Button>Uppdatera bok</Form.Button>
  {:else}
    <Form.Button>Lägg till bok</Form.Button>
  {/if}
  </div>
</form>

<SuperDebug data={$formData} />
