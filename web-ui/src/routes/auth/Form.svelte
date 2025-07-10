<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { HTMLFormAttributes } from 'svelte/elements';

	interface Props extends Omit<HTMLFormAttributes, 'class'> {
		children?: Snippet;
		color: 'red' | 'blue';
	}

	const { children, color, ...formAttributes }: Props = $props();
</script>

<form
	{...formAttributes}
	class={[
		'm-0.5 flex flex-col items-center justify-start gap-1 rounded-md border-2 p-1.5 transition-all duration-200 ease-in-out hover:m-0 hover:border-3 hover:p-[7px]',
		color == 'red' ? 'border-redpill' : 'border-bluepill'
	]}
>
	{@render children?.()}
</form>
