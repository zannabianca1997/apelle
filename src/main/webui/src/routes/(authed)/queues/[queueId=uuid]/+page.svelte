<script lang="ts">
	import type { PageData, PageProps } from './$types';

	let { data }: PageProps = $props();
</script>

<span>Page for queue: {data.queue.id}</span>
