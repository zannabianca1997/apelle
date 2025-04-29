<script lang="ts">
	import { getApiV1Search, type PageSearchedSongQueryDto, type SongAddDto } from '$lib/apis/apelle';
	import SearchBar from './SearchBar.svelte';

	const {
		onSongChosen,
		initialQuery
	}: {
		onSongChosen?: (s: SongAddDto) => void;
		initialQuery?: string;
	} = $props();

	let songs = $state<PageSearchedSongQueryDto | null>(null);
	let searching = $state(false);

	async function onsubmit(q: string): Promise<boolean> {
		searching = true;
		songs = (await getApiV1Search({ q })).data;
		searching = false;
		return true;
	}

	$effect(() => {
		if (initialQuery) {
			onsubmit(initialQuery);
		}
	});
</script>

<section>
	<SearchBar {onsubmit} />
</section>
<section>
	<table>
		<tbody>
			{#each songs?.items || [] as song}
				<tr>
					<td>
						{song.name}
					</td>
					<td>
						<button onclick={() => onSongChosen?.(song.enqueue_data)}>Add</button>
					</td>
				</tr>
			{/each}
		</tbody>
	</table>
</section>

<style lang="scss">
</style>
