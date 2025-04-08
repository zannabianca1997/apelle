<script lang="ts">
	import type { QueueQueryDto } from '$lib/apis/apelle';
	import TextInput from '$lib/components/forms/TextInput.svelte';
	import type { PageProps } from './$types';
	import type { QueueUserQueryWithRoleDto } from './+page';
	import { _ } from 'svelte-i18n';
	import Player from '$lib/components/backoffice/Player.svelte';

	const { data }: PageProps = $props();
	const queue: QueueQueryDto = $state(data.queue);
	let isPlayer: boolean = $state(data.isPlayer);
	const user: QueueUserQueryWithRoleDto = $state(data.user);
</script>

<main>
	{#if isPlayer}
		<Player current={queue.current} />
	{/if}
	<h1>{$_('backoffice.partyName')}<code>{queue.code}</code></h1>
	<form>
		<TextInput
			label={$_('backoffice.addSong.label')}
			placeholder={$_('backoffice.addSong.placeholder')}
		/>
		<button>{$_('backoffice.addSong.submit')}</button>
	</form>
</main>

<style lang="scss">
	main {
		width: 80%;

		display: flex;
		flex-direction: column;
		gap: 57px;

		h1 {
			font-weight: 900;
			font-size: 32px;
			line-height: 150%;
			letter-spacing: 1%;

			color: #379b46;

			code {
				color: white;
			}
		}
		form {
			width: 100%;

			display: flex;
			gap: 12px;
			align-items: last baseline;

			--input-flex-grow: 1;

			button {
				width: 100px;
				border: 1px solid white;
				height: 48px;
				border-radius: 4px;
				padding-top: 6px;
				padding-right: 12px;
				padding-bottom: 6px;
				padding-left: 12px;

				font-weight: 900;
				font-size: 16px;
				line-height: 100%;
				letter-spacing: 0%;

				color: white;
				background-color: transparent;

				cursor: pointer;
			}
		}
	}
</style>
