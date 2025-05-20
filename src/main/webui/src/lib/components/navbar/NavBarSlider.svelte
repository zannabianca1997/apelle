<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { HTMLInputAttributes } from 'svelte/elements';
	import { _ } from 'svelte-i18n';

	let {
		value = $bindable(0),
		children,
		...inputAttributes
	}: {
		value?: number;
		children?: Snippet<[{ value: number }]>;
	} & Omit<HTMLInputAttributes, 'type' | 'value'> = $props();
</script>

<div>
	<input type="range" bind:value {...inputAttributes} />
	{#if children}
		<span>{@render children({ value })}</span>
	{/if}
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

		span {
			text-transform: uppercase;

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
