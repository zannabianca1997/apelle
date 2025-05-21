<script lang="ts">
	import type { Component, Snippet } from 'svelte';
	import type { HTMLInputAttributes } from 'svelte/elements';
	import { _ } from 'svelte-i18n';

	let {
		icon: Icon,
		value = $bindable(0),
		children,
		...inputAttributes
	}: {
		icon?: Component<{ height: number; width: number }>;
		value?: number;
		children?: Snippet<[{ value: number }]>;
	} & Omit<HTMLInputAttributes, 'type' | 'value'> = $props();

	const id = $props.id();
</script>

<div>
	{#if Icon}
		<Icon height={24} width={24} />
	{/if}
	{#if children}
		<label for="input-{id}">{@render children({ value })}</label>
	{/if}
	<input type="range" bind:value {...inputAttributes} id="input-{id}" />
</div>

<style lang="scss">
	div {
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

		label {
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
