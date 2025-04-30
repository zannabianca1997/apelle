<script lang="ts">
	import type { SongAddDto } from '$lib/apis/apelle';
	import SearchView from './SearchView.svelte';

	const {
		onSongChosen: onSongChosenInner
	}: {
		onSongChosen?: (s: SongAddDto) => void;
	} = $props();

	let dialog: HTMLDialogElement;

	let searchView: SearchView;

	export async function open(initialQuery: string) {
		dialog.show();
		await searchView.searchFor(initialQuery);
	}

	function onSongChosen(s: SongAddDto) {
		onSongChosenInner?.(s);
		dialog.close();
	}
</script>

<dialog bind:this={dialog} closedby="any">
	<SearchView bind:this={searchView} {onSongChosen} />
</dialog>

<style lang="scss">
	dialog[open] {
		width: 90%;

		display: flex;
		flex-direction: column;
	}
</style>
