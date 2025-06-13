<script lang="ts">
  import { Button } from "$lib/components/ui/button/index.js";
  import * as Card from "$lib/components/ui/card/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import { Label } from "$lib/components/ui/label/index.js";
  const id = $props.id();
</script>

<Card.Root class="mx-auto w-full max-w-sm">
  <Card.Header>
    <Card.Title class="text-2xl">Logga in</Card.Title>
    <Card.Description>Ange dina inloggningsuppgifter för att få åtkomst till biblioteket.</Card.Description>
  </Card.Header>
  <Card.Content>
    <div class="grid gap-4">
      <div class="grid gap-2">
        <Label for="username-{id}">Användarnamn</Label>
        <Input id="username-{id}" type="username" required />
      </div>
      <div class="grid gap-2">
        <div class="flex items-center">
          <Label for="password-{id}">Lösenord</Label>
          <a href="/help#Glömt lösenord" class="ml-auto inline-block text-sm underline">
            Glömt lösenord?
          </a>
        </div>
        <Input id="password-{id}" type="password" required />
      </div>
      <Button type="submit" class="w-full">Logga in</Button>
    </div>
    <div class="mt-4 text-center text-sm">
      Har du inga inloggningsuppgifter?
      <a href="/register" class="underline"> Bli användare</a>
    </div>
  </Card.Content>
</Card.Root>
