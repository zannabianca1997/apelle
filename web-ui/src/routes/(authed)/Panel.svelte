<script lang="ts">
	import Button from '$lib/components/forms/Button.svelte';
	import { onMount, type Component, type Snippet } from 'svelte';
	import type { HTMLFormAttributes, SVGAttributes } from 'svelte/elements';

	interface Props extends Pick<HTMLFormAttributes, 'onsubmit'> {
		icon: Component<Pick<SVGAttributes<SVGSVGElement>, 'height' | 'width'>>;
		title: string;
		activePanel: string;
		active?: boolean;
		children: Snippet;
		color: 'red' | 'blue';
	}

	const id = $props.id();
	let {
		icon,
		title,
		activePanel = $bindable(),
		active = false,
		children,
		color,
		onsubmit
	}: Props = $props();

	onMount(() => {
		if (active && !activePanel) {
			activePanel = id;
		}
	});
	$effect(() => {
		active = activePanel === id;
	});
</script>

<div
	class={[
		'flex flex-col gap-2 rounded-md',
		color === 'red' ? 'border-redpill' : 'border-bluepill',
		active ? 'flex-grow-1 border-3 p-2' : 'p-0'
	]}
>
	<Button
		{icon}
		class={['w-full', color === 'red' ? 'bg-redpill' : 'bg-bluepill']}
		onclick={() => {
			activePanel = id;
		}}>{title}</Button
	>
	{#if active}
		<form class="flex-grow-1 flex flex-wrap items-center justify-stretch gap-2" {onsubmit}>
			{@render children()}
		</form>
	{/if}
</div>
