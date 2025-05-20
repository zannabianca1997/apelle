<script lang="ts">
	import { _ } from 'svelte-i18n';
	import type { CurrentSong } from '$lib/models/Queue.svelte';
	import YoutubePlayer from './sources/YoutubePlayer.svelte';
	import Thumbnail from '../Thumbnail.svelte';
	import { Logger } from '$lib/logger';
	import config from '$lib/config';
	import type { Snapshot } from '../../../../routes/$types';

	const logger = new Logger(config.log.player);

	const {
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

	export const snapshot: Snapshot<
		| (NonNullable<typeof sourcePlayer> extends { snapshot: Snapshot<infer T> } ? T : never)
		| undefined
	> = {
		capture: () => sourcePlayer?.snapshot.capture(),
		restore: (value) => value && sourcePlayer?.snapshot.restore(value)
	};

	export { navbar };
</script>

{#snippet navbar()}{@render sourcePlayer?.navbar()}{/snippet}

{#if isPlayer}
	<div class="iframe">
		<SourcePlayer bind:this={sourcePlayer} {current} />
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
