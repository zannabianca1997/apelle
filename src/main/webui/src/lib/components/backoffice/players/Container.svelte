<script lang="ts">
	import type { CurrentSongQueryDto } from '$lib/apis/apelle';
	import { _ } from 'svelte-i18n';
	import Player from './Player.svelte';

	const { current }: { current?: CurrentSongQueryDto } = $props();
</script>

<section>
	{#if current}
		<Player {current} />
	{:else}
		<h1>{$_('backoffice.currentSong.nothingPlaying')}</h1>
	{/if}
</section>

<style lang="scss">
	section {
		display: flex;
		flex-direction: row;
		justify-content: center;
		align-items: center;
		gap: 24px;

		width: 100%;
		height: 161px;
		border-radius: 6px;
		padding: 12px;

		background: linear-gradient(90deg, rgba(55, 155, 70, 0.75) 0%, rgba(36, 101, 46, 0.75) 81.67%);

		h1 {
			margin: 0;
		}
	}
</style>
