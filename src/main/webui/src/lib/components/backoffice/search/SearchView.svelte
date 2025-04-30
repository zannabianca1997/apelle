<script lang="ts">
	import { getApiV1Search, type PageSearchedSongQueryDto, type SongAddDto } from '$lib/apis/apelle';
	import SearchBar from './SearchBar.svelte';
	import { _ } from 'svelte-i18n';

	const {
		onSongChosen
	}: {
		onSongChosen?: (s: SongAddDto) => void;
	} = $props();

	let songs = $state<PageSearchedSongQueryDto | null>(null);
	let searching = $state(false);

	/**
	 * Search for a given song.
	 *
	 * The promise will be resolved when the search is completed.
	 * The return value will be true if the search is successfull
	 *
	 * @param q The song to search for
	 */
	export async function searchFor(q: string): Promise<boolean> {
		searching = true;
		songs = (await getApiV1Search({ q })).data;
		searching = false;
		return true;
	}
</script>

<section>
	<SearchBar onsubmit={searchFor} />
</section>
<section>
	{#if songs}
		<table>
			<tbody>
				{#each songs.items as song}
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
	{:else}
		<p>{$_('')}</p>
	{/if}
</section>

<style lang="scss">
</style>
