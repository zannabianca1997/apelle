<script lang="ts">
	import TextInput from '$lib/components/forms/TextInput.svelte';
	import { _ } from 'svelte-i18n';

	let {
		onsubmit,
		query = $bindable()
	}: {
		onsubmit: (e: SubmitEvent) => void;
		query?: string | null;
	} = $props();
</script>

<form {onsubmit}>
	<TextInput
		bind:value={query}
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
