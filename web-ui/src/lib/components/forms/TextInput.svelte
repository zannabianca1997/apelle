<script lang="ts">
	import type { HTMLInputAttributes } from 'svelte/elements';

	interface CapturedProps {
		label: string;

		password?: boolean;
		error?: string | null;
	}

	type Props = CapturedProps &
		Omit<HTMLInputAttributes, keyof CapturedProps | 'type' | 'id' | 'class'>;

	const id = $props.id();
	let {
		label,

		password = false,
		error = null,

		value = $bindable(),

		...inputProps
	}: Props = $props();
</script>

<div class="flex flex-col gap-1.5">
	<label for="input-{id}" class="w-full text-base leading-[150%] font-light tracking-[1%]"
		>{label}</label
	>
	<input
		id="input-{id}"
		type={password ? 'password' : 'text'}
		class="w-full rounded-md border border-[#122a42] p-3 text-base leading-[150%] font-light tracking-[1%] text-black placeholder-[#122a4282]"
		bind:value
		{...inputProps}
	/>
	<div
		class={[
			'mt-auto h-[20px] w-full',
			!!error &&
				'rounded-sm border border-red-500 text-center text-xs leading-[150%] font-light tracking-[1%] text-red-500'
		]}
	>
		{error}
	</div>
</div>
