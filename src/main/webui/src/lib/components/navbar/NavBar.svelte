<script lang="ts">
	import { goto } from '$app/navigation';
	import authService from '$lib/auth.svelte';
	import IconMusicNote from '~icons/mdi/music-note';
	import IconLogout from '~icons/mdi/logout-variant';
	import { _ } from 'svelte-i18n';

	let { collapsed = $bindable() }: { collapsed?: boolean } = $props();

	async function signout() {
		await authService.signout();
		await goto('/authenticate');
	}
</script>

<nav class={{ collapsed }}>
	<button onclick={() => (collapsed = !collapsed)}>
		<IconMusicNote height="24px" width="24px" /><span>{$_('navbar.title')}</span>
	</button>
	{#if !collapsed}
		<button onclick={signout}>
			<IconLogout height="24px" width="24px" /><span>{$_('navbar.logout')}</span>
		</button>
	{/if}
</nav>

<style lang="scss">
	nav {
		position: absolute;

		top: 12px;
		left: 55px;

		display: flex;
		gap: 10px;

		border-radius: 4px;

		background: linear-gradient(180deg, #911616 0%, #691010 77.5%);

		button {
			height: 36px;

			padding-top: 6px;
			padding-right: 12px;
			padding-bottom: 6px;
			padding-left: 12px;

			border: 0px;
			background: transparent;
			color: white;

			display: flex;
			align-items: center;
			gap: 5px;

			span {
				text-transform: uppercase;

				font-weight: 900;
				font-size: 16px;
				line-height: 100%;
				letter-spacing: 0%;
			}

			&:hover {
				background: linear-gradient(0deg, #911616 0%, #691010 77.5%);
			}

			&:first-child {
				border-top-left-radius: 4px;
				border-bottom-left-radius: 4px;
			}

			&:last-child {
				border-top-right-radius: 4px;
				border-bottom-right-radius: 4px;
			}
		}
	}
</style>
