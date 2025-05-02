<script lang="ts">
	import { getApiV1Search, type PageSearchedSongQueryDto, type SongAddDto } from '$lib/apis/apelle';
	import type { Snapshot } from '@sveltejs/kit';
	import SearchBar from './SearchBar.svelte';
	import { _ } from 'svelte-i18n';
	import Thumbnail from '../Thumbnail.svelte';

	const {
		onSongChosen: onSongChosenInner,
		onDismiss: onDismissInner
	}: {
		onSongChosen?: (s: SongAddDto) => void;
		onDismiss?: () => void;
	} = $props();

	let query = $state('');
	let songs = $state.raw<PageSearchedSongQueryDto | null>(null);
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
		query = q;
		songs = (await getApiV1Search({ q })).data;
		searching = false;
		return true;
	}

	async function goto(page?: string | null) {
		if (!page) {
			return;
		}

		searching = true;
		songs = (await getApiV1Search({ q: query, page })).data;
		searching = false;
	}

	function onSongChosen(s: SongAddDto) {
		query = '';
		songs = null;
		searching = false;
		onSongChosenInner?.(s);
	}

	export const snapshot: Snapshot<{
		query: string;
		songs: PageSearchedSongQueryDto;
	} | null> = {
		capture: () => songs && { query, songs },
		restore: (v) => {
			if (v) {
				query = v.query;
				songs = v.songs;
			}
		}
	};

	const onDismiss =
		onDismissInner &&
		(() => {
			query = '';
			songs = null;
			searching = false;
			onDismissInner();
		});
</script>

<section>
	<SearchBar onsubmit={searchFor} />
</section>
<section class="songList">
	{#if songs}
		<table>
			<tbody>
				{#each songs.items as song}
					<tr>
						<td class="thumb">
							{#if song.thumbnails}
								<Thumbnail thumbnails={song.thumbnails} />
							{/if}
						</td>
						<td>
							{song.name}
						</td>
						<td>
							<button onclick={() => onSongChosen?.(song.enqueue_data)}>
								{$_('backoffice.search.add')}
							</button>
						</td>
					</tr>
				{/each}
			</tbody>
		</table>
	{/if}
</section>
<nav>
	{#if onDismiss}
		<button onclick={onDismiss} class="dismiss">
			{$_('backoffice.search.dismiss')}
		</button>
	{/if}
	{#if songs}
		{#if songs.page_info.prev}
			<button onclick={() => goto(songs?.page_info.prev)}>
				{$_('backoffice.search.prev')}
			</button>
		{/if}
		<span>
			{$_('backoffice.search.pageOnTotal', {
				values: {
					number: songs.page_info.number,
					total:
						songs.page_info.total_items != null
							? Math.ceil(songs.page_info.total_items / songs.page_info.items)
							: $_('backoffice.search.unknowPageNumber')
				}
			})}
		</span>
		{#if songs.page_info.next}
			<button onclick={() => goto(songs?.page_info.next)}>
				{$_('backoffice.search.next')}
			</button>
		{/if}
	{/if}
</nav>

<style lang="scss">
	.songList {
		table {
			width: 100%;
		}

		.thumb {
			width: 176px;
			height: 99px;

			background-color: transparent;

			padding: 0;
		}
	}

	nav {
		display: flex;
		flex-direction: row;

		gap: 10px;

		.dismiss {
			margin-right: auto;
		}
	}
</style>
