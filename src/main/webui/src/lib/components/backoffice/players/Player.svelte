<script lang="ts">
	import { _ } from 'svelte-i18n';
	import type { CurrentSong } from '$lib/models/Queue.svelte';
	import type { ThumbnailQueryDto } from '$lib/apis/apelle';

	const {
		current = $bindable(),
		isPlayer = $bindable(false)
	}: { current: CurrentSong; isPlayer: boolean } = $props();

	let thumbHeight: number = $state(176);
	let thumbWidth: number = $state(99);

	let choosedThumb = $derived.by(() => {
		const thumbScore = (thumb: ThumbnailQueryDto) => {
			return -((thumb.height - thumbHeight) ** 2 + (thumb.width - thumbWidth) ** 2);
		};

		return current.thumbnails?.reduce((t1, t2) => (thumbScore(t1) > thumbScore(t2) ? t1 : t2)).url;
	});
</script>

{#if isPlayer}
	<div class="iframe"></div>
{:else}
	<div class="thumb" bind:offsetHeight={thumbHeight} bind:offsetWidth={thumbWidth}>
		{#if choosedThumb}
			<img alt="" src={choosedThumb} />
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
		width: 244px;
		height: 137px;
	}

	.thumb {
		background-color: transparent;

		padding: 0;
		img {
			width: 100%;
			height: 100%;

			display: block;
		}
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
