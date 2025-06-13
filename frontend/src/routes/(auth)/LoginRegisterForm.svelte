<script lang="ts">
  import * as Form from "$lib/components/ui/form/index.js";
  import * as Card from "$lib/components/ui/card/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import { userCredentialsSchema, type FormSchema } from "./user-credentials-schema.js";
  import {
   type SuperValidated,
   type Infer,
   superForm,
  } from "sveltekit-superforms";
  import { zodClient } from "sveltekit-superforms/adapters";
  
  let { data, type }: { 
    data: { form: SuperValidated<Infer<FormSchema>> },
    type: string
    } =$props();
  
  const form = superForm(data.form, {
   validators: zodClient(userCredentialsSchema),
  });
  
  const { form: formData, enhance } = form;

  let isLogin = type == "login";
</script>
 

<Card.Root class="mx-auto w-full max-w-sm">
  <Card.Header>
    {#if isLogin}
      <Card.Title class="text-2xl">Logga in</Card.Title>
      <Card.Description>Ange dina inloggningsuppgifter för att få åtkomst till biblioteket.</Card.Description>
    {:else}
      <Card.Title class="text-2xl">Bli användare</Card.Title>
      <Card.Description>För att få åtkomst till biblioteket måste du skapa en inloggning.</Card.Description>
    {/if}
  </Card.Header>
  <Card.Content>
    <div class="grid gap-4">
      <form method="POST" use:enhance>
        <Form.Field {form} name="username">
          <Form.Control>
            {#snippet children({ props })}
              <Form.Label>Användarnamn</Form.Label>
              <Input {...props} bind:value={$formData.username} />
            {/snippet}
          </Form.Control>
          <Form.FieldErrors />
        </Form.Field>
        <Form.Field {form} name="password">
          <Form.Control>
            {#snippet children({ props })}
              <Form.Label>Lösenord</Form.Label>
              <Input {...props} type="password" bind:value={$formData.password} />
            {/snippet}
          </Form.Control>
          <Form.FieldErrors />
        </Form.Field>
        {#if isLogin}
          <Form.Button class="w-full">Logga in</Form.Button>
        {:else}
          <Form.Button class="w-full">Registrera</Form.Button>
        {/if}
      </form>
    </div>
    <div class="mt-4 text-center text-sm">
      {#if isLogin}
        Har du inga inloggningsuppgifter?
        <a href="/register" class="underline"> Bli användare</a>
      {:else}
        Är du redan en användare?
        <a href="/login" class="underline"> Logga in</a>
      {/if}
    </div>
  </Card.Content>
</Card.Root>

