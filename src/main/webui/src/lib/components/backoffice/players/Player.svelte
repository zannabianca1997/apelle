<script lang="ts">
	import { _ } from 'svelte-i18n';
	import type { CurrentSong } from '$lib/models/Queue.svelte';
	import type { ThumbnailQueryDto } from '$lib/apis/apelle';
	import YoutubePlayer from './sources/YoutubePlayer.svelte';
	import Thumbnail from '../Thumbnail.svelte';

	const {
		current = $bindable(),
		isPlayer = $bindable(false)
	}: { current: CurrentSong; isPlayer: boolean } = $props();
</script>

{#if isPlayer}
	<div class="iframe">
		{#if current.kind == 'Youtube'}
			<YoutubePlayer {current} />
		{:else}
			<!--No other kinds are possible-->
		{/if}
	</div>
{:else}
	<div class="thumb">
		{#if current.thumbnails}
			<Thumbnail thumbnails={current.thumbnails} />
		{/if}
	</div>
{/if}
<div class="card">
	<h2>{current.name}</h2>
	<h3>{current.position.format($_('backoffice.song.durationFormat'))}</h3>
</div>

<style lang="scss">
	.iframe,
	.thumb {
		flex-shrink: 0;
		width: 357px;
		height: 200px;

		padding: 0;
	}

	.thumb {
		background-color: transparent;
	}

	.card {
		flex-grow: 1;
		min-width: 0;

		h2 {
			overflow: hidden;
			white-space: nowrap;
			text-overflow: ellipsis;
		}
	}
</style>
