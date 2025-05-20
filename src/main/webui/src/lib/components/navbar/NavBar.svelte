<script lang="ts">
	import { _ } from 'svelte-i18n';
	import { navElements, navMenuElements, type NavBarElement } from './stores.svelte';
	import IconMenu from '~icons/mdi/menu';
	import NavBarButton from './elements/NavBarButton.svelte';

	const sortedNav = $derived(
		Object.entries(navElements)
			.map(([id, element]) => ({ id, ...element }))
			.toSorted(orderElements)
	);
	const sortedNavMenu = $derived(
		Object.entries(navMenuElements)
			.map(([id, element]) => ({ id, ...element }))
			.toSorted(orderElements)
	);

	let isMenuHardOpen = $state(false);
	let isMenuSoftOpen = $state(false);

	const isMenuOpen = $derived(isMenuHardOpen || isMenuSoftOpen);

	function orderElements(a: NavBarElement, b: NavBarElement): number {
		// Order negative numbers at the end
		if (a.order * b.order < 0) {
			return a.order < 0 ? 1 : -1;
		}
		return a.order - b.order;
	}
</script>

<nav>
	{#each sortedNav as { id, content } (id)}
		{@render content()}
	{/each}

	<div class="dropdown" onmouseleave={() => (isMenuSoftOpen = false)} role="toolbar" tabindex={0}>
		<NavBarButton
			icon={IconMenu}
			onclick={() => {
				isMenuHardOpen = !isMenuHardOpen;
				isMenuSoftOpen = false;
			}}
			onmouseover={() => (isMenuSoftOpen = true)}
		>
			{$_('navbar.dropdown')}
		</NavBarButton>
		<menu class:open={isMenuOpen}>
			{#each sortedNavMenu as { id, content } (id)}
				{@render content()}
			{/each}
		</menu>
	</div>
</nav>

<style lang="scss">
	nav {
		position: fixed;
		top: 0;
		width: 100%;

		padding-top: 12px;
		padding-right: 55px;
		padding-bottom: 12px;
		padding-left: 55px;

		border-bottom: 1px solid #e182824f;

		background-color: #0a0a0a;

		display: flex;
		justify-content: space-between;

		z-index: 1;

		.dropdown {
			position: relative;

			menu {
				display: none;
				position: absolute;
				top: 100%;
				right: 0;
				background-color: #0a0a0a;
				margin: 0;
				padding: 12px;

				border: 1px solid #e182824f;

				&.open {
					display: block;
				}
			}
		}
	}
</style>
