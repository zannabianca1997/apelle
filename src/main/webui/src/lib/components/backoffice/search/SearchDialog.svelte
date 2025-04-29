<script lang="ts">
	import type { SongAddDto } from '$lib/apis/apelle';
	import SearchView from './SearchView.svelte';

	const {
		onSongChosen: onSongChosenInner,
		initialQuery
	}: {
		onSongChosen?: (s: SongAddDto) => void;
		initialQuery?: string;
	} = $props();

	let dialog: HTMLDialogElement;

	function onSongChosen(s: SongAddDto) {
		onSongChosenInner?.(s);
		dialog.close();
	}
</script>

<dialog bind:this={dialog}>
	<SearchView {onSongChosen} {initialQuery} />
</dialog>

<style lang="scss">
	dialog {
		width: 90%;

		display: flex;
		flex-direction: column;
	}
</style>
