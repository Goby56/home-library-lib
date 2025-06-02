<script lang="ts">
  import Button from "./ui/button/button.svelte";
  
  let { image = $bindable(), label, imageUrl = $bindable() } : 
    { 
      image: File, 
      label: string, 
      imageUrl: string
    } = $props()
   
  function onInputChange(event: Event) {
    const files = (event.target as HTMLInputElement).files;
    if (files && files.length > 0) {
      image = files[0];
      imageUrl = URL.createObjectURL(files[0])
    }
  }

</script>

<Button variant="outline" class="relative">
  {label}
  <input class="absolute w-full opacity-0" type="file" onchange={onInputChange} multiple={false} accept="image/webp, image/png, image/jpeg" required/>
</Button>

