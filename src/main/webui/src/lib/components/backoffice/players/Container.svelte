<script lang="ts">
	import type { CurrentSongQueryDto } from '$lib/apis/apelle';
	import { _ } from 'svelte-i18n';
	import Player from './Player.svelte';
	import type { QueueUserQueryWithRoleDto } from '$lib/models/QueueUserQueryWithRoleDto';
	import IconPlay from '~icons/mdi/play';
	import IconPause from '~icons/mdi/pause';
	import IconNext from '~icons/mdi/skip-next';

	const { current, user }: { current?: CurrentSongQueryDto; user: QueueUserQueryWithRoleDto } =
		$props();
</script>

<section>
	{#if current}
		<Player {current} />
	{:else}
		<h1>{$_('backoffice.currentSong.nothingPlaying')}</h1>
	{/if}
	<div class="playControls">
		{#if !current || current.stopped}
			<button><IconPlay height={75} width={75} /></button>
		{:else}
			<button><IconPause height={75} width={75} /></button>
		{/if}
		<button><IconNext height={75} width={75} /></button>
	</div>
</section>

<style lang="scss">
	section {
		display: flex;
		flex-direction: row;
		justify-content: space-evenly;
		align-items: center;
		gap: 24px;

		width: 100%;
		height: 161px;
		border-radius: 6px;
		padding: 12px;

		background: linear-gradient(90deg, rgba(55, 155, 70, 0.75) 0%, rgba(36, 101, 46, 0.75) 81.67%);

		h1 {
			margin: 0;
			flex-grow: 1;

			text-align: center;
		}

		.playControls {
			display: flex;
			gap: 55px;
			flex-grow: 0;

			button {
				background: transparent;
				border: 0;
				color: white;

				&:hover {
					background: radial-gradient(closest-side, #00000088, #00000000);
				}
			}
		}
	}
</style>
