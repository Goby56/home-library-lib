<script lang="ts">
  import AppLogo from "$lib/assets/rose_icon.svelte";
  import DarkModeToggle from "$lib/components/DarkModeToggle.svelte";
  import type { PageData } from "./$types.js";
  import LoginRegisterForm from "../LoginRegisterForm.svelte";

  let { data }: { data: PageData } = $props();
</script>
<div class="flex flex-col gap-10 h-screen w-full items-center justify-center px-4">
  <a class="flex justify-center items-center gap-3 text-3xl font-semibold" href="/">
    <AppLogo/>
    Rosenport
  </a>
  <LoginRegisterForm {data} type="login"/>
  <DarkModeToggle/>
</div>

