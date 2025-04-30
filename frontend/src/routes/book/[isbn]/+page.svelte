<script lang="ts">
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();

  let book = data.books[0];
</script>

<h4 class="line-clamp-2 scroll-m-20 text-xl font-semibold tracking-tight">{book.title}</h4>
