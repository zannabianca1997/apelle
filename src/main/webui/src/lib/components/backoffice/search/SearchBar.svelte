<script lang="ts">
	import TextInput from '$lib/components/forms/TextInput.svelte';
	import { _ } from 'svelte-i18n';

	let {
		onsubmit: onsubmitInner
	}: {
		onsubmit?: (e: string) => Promise<boolean>;
	} = $props();

	let value: string | null = $state(null);

	async function onsubmit(e: SubmitEvent) {
		e.preventDefault();
		const query = value?.trim();
		if (!query) {
			return;
		}
		const reset = onsubmitInner?.(query);
		if (reset && (await reset)) {
			value = null;
		}
	}
</script>

<form {onsubmit}>
	<TextInput
		bind:value
		label={$_('backoffice.searchSong.label')}
		placeholder={$_('backoffice.searchSong.placeholder')}
	/>
	<button>{$_('backoffice.searchSong.submit')}</button>
</form>

<style lang="scss">
	form {
		width: 100%;

		display: flex;
		gap: 12px;
		align-items: last baseline;

		--input-flex-grow: 1;

		button {
			width: 175px;
			height: 48px;
			top: 26px;
			left: 758px;
			border-radius: 4px;
			padding-top: 6px;
			padding-right: 12px;
			padding-bottom: 6px;
			padding-left: 12px;

			font-weight: 900;
			font-size: 16px;
			line-height: 100%;
			letter-spacing: 0%;

			text-transform: uppercase;
			border: 0;

			color: white;
			background: #911616;

			cursor: pointer;
		}
	}
</style>
