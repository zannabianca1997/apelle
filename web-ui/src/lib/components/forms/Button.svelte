<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { ClassValue, HTMLButtonAttributes } from 'svelte/elements';

	interface Props extends Omit<HTMLButtonAttributes, 'class'> {
		children?: Snippet;
		class?: ClassValue | null | undefined;
	}

	const { children, class: clazz, ...buttonAttributes }: Props = $props();
</script>

<button {...buttonAttributes} class={['rounded-md p-3', clazz]}>
	{@render children?.()}
</button>
