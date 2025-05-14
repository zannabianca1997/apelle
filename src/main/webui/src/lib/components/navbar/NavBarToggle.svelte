<script lang="ts">
	import type { Component, Snippet } from 'svelte';
	import IconOnDefault from '~icons/mdi/checkbox-marked-circle-outline';
	import IconOffDefault from '~icons/mdi/checkbox-blank-circle-outline';
	import { _ } from 'svelte-i18n';

	let {
		value = $bindable(false),
		icons,
		children
	}: {
		value?: boolean;

		onclick?: () => void;
		children?: Snippet<[{ value: boolean }]>;

		icons?:
			| true
			| {
					on?: Component<{ height: number; width: number }>;
					off?: Component<{ height: number; width: number }>;
			  };
	} = $props();

	const IconOn = icons === true ? IconOnDefault : (icons?.on ?? IconOnDefault);
	const IconOff = icons === true ? IconOffDefault : (icons?.off ?? IconOffDefault);

	const iconProps = {
		height: 24,
		width: 24
	};

	function onclick() {
		value = !value;
	}
</script>

<button {onclick}>
	{#if icons}
		{#if value}
			<IconOn {...iconProps} />
		{:else}
			<IconOff {...iconProps} />
		{/if}
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
