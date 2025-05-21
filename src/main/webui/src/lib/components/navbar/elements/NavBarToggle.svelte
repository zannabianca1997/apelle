<script lang="ts">
	import type { Component, Snippet } from 'svelte';
	import IconOnDefault from '~icons/mdi/checkbox-marked-circle-outline';
	import IconOffDefault from '~icons/mdi/checkbox-blank-circle-outline';
	import { _ } from 'svelte-i18n';

	type IconComponent = Component<{ height: number; width: number }>;

	let {
		value = $bindable(false),
		icons,
		children
	}: {
		value?: boolean;
		children?: Snippet<[{ value: boolean }]>;
		icons?:
			| boolean
			| {
					on?: IconComponent;
					off?: IconComponent;
			  };
	} = $props();

	const { on: IconOn = IconOnDefault, off: IconOff = IconOffDefault } =
		typeof icons === 'boolean' || !icons ? {} : icons;

	const Icon = $derived(value ? IconOn : IconOff);

	function onclick() {
		value = !value;
	}
</script>

<button {onclick}>
	{#if icons}
		<Icon height={24} width={24} />
	{/if}
	{#if children}
		<span>{@render children({ value })}</span>
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
