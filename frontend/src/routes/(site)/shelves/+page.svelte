<script lang="ts">
	import type { PageProps } from './$types';
  import { invalidateAll } from '$app/navigation';
  import axios from "axios";
 
	let { data }: PageProps = $props();
</script>

