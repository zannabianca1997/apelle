<script lang="ts">
	import type { Snapshot } from './$types';

	import morpheus from '$lib/assets/morpheus.png';
	import TextInput from '$lib/components/forms/TextInput.svelte';

	import ExpandingButton from '$lib/components/frontoffice/ExpandingButton.svelte';
	import { _ } from 'svelte-i18n';

	let expanded: string | null = $state(null);
	export const snapshot: Snapshot<{ expanded: string | null }> = {
		capture: () => {
			return {
				expanded
			};
		},
		restore: (value) => (expanded = value.expanded)
	};
</script>

<header>
	<h1>{$_('frontoffice.title')}</h1>
	<h2>{$_('frontoffice.subtitle')}</h2>
</header>
<main>
	<img src={morpheus} alt={$_('frontoffice.images.morpheus')} />
	<div class="panels">
		<ExpandingButton
			id="host"
			text={$_('frontoffice.choices.host.text')}
			--theme-color="var(--red-pill)"
			bind:expanded
		>
			Host choices
		</ExpandingButton>
		<ExpandingButton
			id="join"
			text={$_('frontoffice.choices.join.text')}
			--theme-color="var(--blue-pill)"
			bind:expanded
		>
			<form class="join">
				<TextInput
					label={$_('frontoffice.choices.join.id.label')}
					placeholder={$_('frontoffice.choices.join.id.placeholder')}
				/>
				<input type="submit" value={$_('frontoffice.choices.join.submit')} />
			</form>
		</ExpandingButton>
	</div>
</main>

<style lang="scss">
	header {
		width: 100%;

		h1 {
			font-weight: 900;
			font-size: 48px;
			line-height: 150%;
			letter-spacing: 1%;
			text-align: center;

			margin: 27px;
		}

		h2 {
			font-weight: 900;
			font-size: 32px;
			line-height: 150%;
			letter-spacing: 1%;
			text-align: center;

			margin: 27px;

			color: #e18282;
		}
	}

	main {
		width: 100%;
		display: flex;
		gap: 10px;

		img {
			width: 312px;
			height: 233px;
			border-radius: 6px;
		}
		.panels {
			flex-grow: 1;

			display: flex;
			flex-direction: column;
			gap: 10px;

			form.join {
				width: 100%;
				display: flex;
				gap: 12px;
				align-items: last baseline;

				input[type='submit'] {
					width: 100px;

					border: 1px solid white;
					height: 48px;
					border-radius: 4px;
					padding-top: 6px;
					padding-right: 12px;
					padding-bottom: 6px;
					padding-left: 12px;

					font-weight: 900;
					font-size: 16px;
					line-height: 100%;
					letter-spacing: 0%;

					color: white;
					background-color: transparent;

					cursor: pointer;
				}
			}
		}
	}
</style>
