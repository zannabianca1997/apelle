<script lang="ts">
	import type { QueueQueue } from '$lib/apis/apelle';
	import QueuedSongCard from './QueuedSongCard.svelte';

	let {
		songs
	}: {
		songs: QueueQueue;
	} = $props();
</script>

<ol>
	{#each Object.entries(songs) as [id, song] (id)}
		<li>
			<QueuedSongCard {song} />
		</li>
	{/each}
</ol>
