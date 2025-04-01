<script lang="ts">
	import { goto } from '$app/navigation';
	import authService from '$lib/auth.svelte';

	let { collapsed = $bindable() }: { collapsed?: boolean } = $props();

	async function signout() {
		await authService.signout();
		await goto('/authenticate');
	}
</script>

<nav class={{ collapsed }}>
	<button onclick={() => (collapsed = !collapsed)}>Apelle</button>
	{#if !collapsed}
		<button onclick={signout}>Logout</button>
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
