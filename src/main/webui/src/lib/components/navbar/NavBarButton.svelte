<script lang="ts">
	import type { Component, Snippet } from 'svelte';
	import { _ } from 'svelte-i18n';

	const {
		icon: Icon,
		onclick,
		children
	}: {
		icon?: Component<{ height: string; width: string }>;
		onclick?: () => void;
		children?: Snippet;
	} = $props();
</script>

<button {onclick}>
	{#if Icon}
		<Icon height="24px" width="24px" />
	{/if}
	{#if children}
		<span>{@render children()}</span>
	{/if}
</button>

<style lang="scss">
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
</style>
