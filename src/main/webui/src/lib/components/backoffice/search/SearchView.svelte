<script lang="ts">
	import { getApiV1Search, type PageSearchedSongQueryDto, type SongAddDto } from '$lib/apis/apelle';
	import type { Snapshot } from '@sveltejs/kit';
	import SearchBar from './SearchBar.svelte';
	import { _ } from 'svelte-i18n';
	import Thumbnail from '../Thumbnail.svelte';
	import IconAdd from '~icons/mdi/plus';
	import IconExit from '~icons/mdi/close';
	import IconNextPage from '~icons/mdi/chevron-right';
	import IconPreviousPage from '~icons/mdi/chevron-left';

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
	<SearchBar onsubmit={searchFor} placeholder={query} />
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
						<td class="songCard">
							{song.name}
						</td>
						<td class="addBtn">
							<button onclick={() => onSongChosen?.(song.enqueue_data)}>
								{$_('backoffice.search.add')}
								<IconAdd height={24} width={24} />
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
		<button
			onclick={onDismiss}
			class="dismiss iconOnly"
			aria-label={$_('backoffice.search.dismiss')}
		>
			<IconExit height={24} width={24} />
		</button>
	{/if}
	{#if songs}
		{#if songs.page_info.prev}
			<button
				onclick={() => goto(songs?.page_info.prev)}
				aria-label={$_('backoffice.search.prev')}
				class="iconOnly"
			>
				<IconPreviousPage height={24} width={24} />
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
			<button
				onclick={() => goto(songs?.page_info.next)}
				aria-label={$_('backoffice.search.next')}
				class="iconOnly"
			>
				<IconNextPage height={24} width={24} />
			</button>
		{/if}
	{/if}
</nav>

<style lang="scss">
	.songList {
		table {
			table-layout: fixed;
			width: 100%;
		}

		.thumb {
			width: 176px;
			height: 99px;

			background-color: transparent;

			padding: 0;
		}

		.songCard {
			padding-left: 15px;
			overflow: hidden;
			white-space: nowrap;
			text-overflow: ellipsis;
		}

		.addBtn {
			width: 175px;

			button {
				width: 100%;
				height: 48px;
				border-radius: 4px;
				gap: 10px;
				padding-top: 6px;
				padding-right: 12px;
				padding-bottom: 6px;
				padding-left: 12px;

				border: 0;

				background: #3a3a3a;
				color: white;

				display: flex;
				justify-content: center;
				align-items: center;

				cursor: pointer;
			}
		}
	}

	nav {
		display: flex;
		flex-direction: row;
		align-items: center;

		gap: 10px;

		.dismiss {
			margin-right: auto;
		}
	}

	button.iconOnly {
		border: 0;
		background: transparent;
		cursor: pointer;

		color: white;

		display: flex;
		justify-content: center;
		align-items: center;

		padding: 0;
	}
</style>
