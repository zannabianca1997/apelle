<script lang="ts">
	import type { Snapshot } from './$types';

	import morpheus from '$lib/assets/morpheus.png';
	import TextInput from '$lib/components/forms/TextInput.svelte';

	import IconCrown from '~icons/mdi/crown';
	import IconUserCircleOutline from '~icons/mdi/user-circle-outline';

	import ExpandingButton from '$lib/components/frontoffice/ExpandingButton.svelte';
	import { _ } from 'svelte-i18n';
	import {
		postApiV1Queues as postQueues,
		getApiV1QueuesIQueueId as getQueueById,
		getApiV1QueuesCQueueCode as getQueueByCode,
		type Uuid
	} from '$lib/apis/apelle';
	import { goto } from '$app/navigation';
	import { isUuid } from '$lib/matchers';
	import { AxiosError } from 'axios';
	import { error } from '$lib/errors.svelte';

	let expanded: string = $state('join');

	let queueCode: string | null = $state(null);
	let queueCodeError: string | null = $state(null);

	export const snapshot: Snapshot<{ expanded: string; queueCode: string | null }> = {
		capture: () => ({
			expanded,
			queueCode
		}),
		restore: (value) => {
			expanded = value.expanded;
			queueCode = value.queueCode;
		}
	};

	async function host(partyKind: 'anything') {
		// create the queue
		const {
			data: { id }
		} = await postQueues();
		// navigate to its page
		goto(`/queues/${id}?player=true`);
	}

	async function join(queueCode: string | null) {
		if (!queueCode) {
			queueCodeError = $_('frontoffice.choices.join.errors.queueCodeRequired');
			return;
		}

		let id: Uuid | null = null;
		if (isUuid(queueCode)) {
			try {
				const {
					data: { id: returnedId }
				} = await getQueueById(queueCode);
				id = returnedId;
			} catch (e: unknown) {
				if (!(e instanceof AxiosError) || e.status != 404) {
					throw e;
				}
				// continue assuming it is a code that matches a uuid
			}
		}
		if (!id) {
			try {
				const {
					data: { id: returnedId }
				} = await getQueueByCode(queueCode);
				id = returnedId;
			} catch (e: unknown) {
				if (!(e instanceof AxiosError) || e.status != 404) {
					throw e;
				}
				// This code does not exist
				queueCodeError = $_('frontoffice.choices.join.errors.queueNotFound', {
					values: { queueCode }
				});
				return;
			}
		}

		// navigate to the queue page
		goto(`/queues/${id}`);
	}
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
			{#snippet icon({ size })}<IconCrown height={size} width={size} />{/snippet}
			<form class="host">
				<button
					onclick={(e) => {
						e.preventDefault();
						host('anything');
					}}>{$_('frontoffice.choices.host.anything')}</button
				>
			</form>
		</ExpandingButton>
		<ExpandingButton
			id="join"
			text={$_('frontoffice.choices.join.text')}
			--theme-color="var(--blue-pill)"
			bind:expanded
		>
			{#snippet icon({ size })}<IconUserCircleOutline height={size} width={size} />{/snippet}
			<form class="join">
				<TextInput
					bind:value={queueCode}
					error={queueCodeError}
					label={$_('frontoffice.choices.join.id.label')}
					placeholder={$_('frontoffice.choices.join.id.placeholder')}
				/>
				<button
					onclick={(e) => {
						e.preventDefault();
						join(queueCode);
					}}>{$_('frontoffice.choices.join.submit')}</button
				>
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

			form {
				width: 100%;
				margin-top: auto;
				margin-bottom: auto;

				&.host {
					display: flex;
					justify-content: stretch;

					gap: 12px;

					button {
						flex-grow: 1;
					}
				}

				&.join {
					display: flex;
					flex-direction: row;
					gap: 12px;
					align-items: last baseline;

					--input-flex-grow: 1;

					button {
						width: 100px;
					}
				}

				button {
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
