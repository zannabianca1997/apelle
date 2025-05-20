<script lang="ts">
	import type { Component, Snippet } from 'svelte';
	import { _ } from 'svelte-i18n';
	import type { HTMLButtonAttributes } from 'svelte/elements';

	const {
		icon: Icon,
		children,
		...buttonProps
	}: {
		icon?: Component<{ height: number; width: number }>;
		children?: Snippet;
	} & HTMLButtonAttributes = $props();
</script>

<button {...buttonProps}>
	{#if Icon}
		<Icon height={24} width={24} />
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

		border-radius: 4px;

		span {
			text-transform: uppercase;

			text-wrap: nowrap;

			font-weight: 900;
			font-size: 16px;
			line-height: 100%;
			letter-spacing: 0%;
		}

		&:hover {
			background: linear-gradient(#911616 0%, #691010 77.5%);
		}
	}
</style>
