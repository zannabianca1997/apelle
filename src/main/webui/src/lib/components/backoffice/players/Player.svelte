<script lang="ts">
	import { _ } from 'svelte-i18n';
	import type { CurrentSong } from '$lib/models/Queue.svelte';
	import YoutubePlayer from './sources/YoutubePlayer.svelte';
	import Thumbnail from '../Thumbnail.svelte';
	import { Logger } from '$lib/logger';
	import config from '$lib/config';
	import type { Snapshot } from '@sveltejs/kit';
	import NavBarSection from '$lib/components/navbar/NavBarSection.svelte';
	import NavBarToggle from '$lib/components/navbar/elements/NavBarToggle.svelte';
	import NavBarSlider from '$lib/components/navbar/elements/NavBarSlider.svelte';
	import IconVolume from '~icons/mdi/volume';

	const logger = new Logger(config.log.player);

	let {
		current = $bindable(),
		isPlayer = $bindable(false)
	}: { current: CurrentSong; isPlayer: boolean } = $props();

	const SourcePlayer = $derived.by(() => {
		switch (current.kind) {
			case 'Youtube':
				return YoutubePlayer;
			default:
				const unknownKind: never = current.kind;
				logger.error('Unknown song kind: %o', unknownKind);
				throw new Error(`Unknown song kind: ${unknownKind}`);
		}
	});
	let sourcePlayer: YoutubePlayer | undefined = $state();

	let volume: number = $state(50);

	export const snapshot: Snapshot<{
		sourcePlayer?: NonNullable<typeof sourcePlayer> extends { snapshot: Snapshot<infer T> }
			? T
			: never;
		volume: number;
	}> = {
		capture: () => ({ sourcePlayer: sourcePlayer?.snapshot.capture(), volume }),
		restore: (value) => {
			value.sourcePlayer && sourcePlayer?.snapshot.restore(value.sourcePlayer);
			volume = value.volume;
		}
	};
</script>

<NavBarSection menu order={2}>
	<NavBarToggle icons bind:value={isPlayer}>
		{$_('navbar.playFromHere')}
	</NavBarToggle>
	{#if isPlayer}
		<NavBarSlider icon={IconVolume} bind:value={volume} min={0} max={100}>
			{$_('navbar.volume')}
		</NavBarSlider>
	{/if}
</NavBarSection>

{#if isPlayer}
	<div class="iframe">
		<SourcePlayer bind:this={sourcePlayer} {current} bind:volume />
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
	<h3>
		{$_('backoffice.song.progress', {
			values: {
				position: current.position.format($_('backoffice.song.durationFormat')),
				duration: current.duration.format($_('backoffice.song.durationFormat'))
			}
		})}
	</h3>
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
