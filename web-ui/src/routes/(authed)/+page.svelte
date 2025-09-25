<script lang="ts">
	import { _ } from 'svelte-i18n';
	import morpheus from '$lib/assets/morpheus.png';
	import Panel from './Panel.svelte';
	import IconCrown from '~icons/mdi/crown';
	import IconUserCircleOutline from '~icons/mdi/user-circle-outline';
	import Button from '$lib/components/forms/Button.svelte';
	import SearchBar from '$lib/components/forms/SearchBar.svelte';
	import type { Snapshot } from '@sveltejs/kit';
	import { queuesFind, queuesCreate } from '$lib/apis/apelle';
	import { AxiosError } from 'axios';
	import { Logger } from '$lib/logger';
	import { goto } from '$app/navigation';
	import type { EventHandler, MouseEventHandler } from 'svelte/elements';
	import normalizeCode from '$lib/utils/normalizeCode';

	const logger = new Logger('routes.authed');

	let activePanel = $state('');

	let code: string | null = $state(null);
	let joinError: string | null = $state(null);

	export const snapshot: Snapshot<{ activePanel: string; code: string | null }> = {
		capture: () => ({
			activePanel,
			code
		}),
		restore: (value) => {
			activePanel = value.activePanel;
			code = value.code;
		}
	};

	const host: EventHandler<SubmitEvent, HTMLFormElement> = async (e) => {
		e.preventDefault();

		const createResponse = await queuesCreate({});
		const { id } = createResponse.data;

		logger.debug('Redirecting to queue', id);

		await goto(`/queues/${id}`);
	};

	const join: EventHandler<SubmitEvent, HTMLFormElement> = async (event) => {
		event.preventDefault();

		code = normalizeCode(code);
		if (!code) {
			joinError = $_('landing.choices.join.errors.queueCodeRequired');
			return;
		}
		joinError = null;

		let searchResponse;
		try {
			searchResponse = await queuesFind({ code });
		} catch (error) {
			if (error instanceof AxiosError && error.status === 404) {
				joinError = $_('landing.choices.join.errors.queueNotFound', {
					values: { queueCode: code }
				});
				return;
			}
			throw error;
		}

		const id = searchResponse.data;

		logger.debug('Redirecting to queue', id);

		await goto(`/queues/${id}`);
	};

	const comingSoon: MouseEventHandler<HTMLButtonElement> = (e) => {
		e.preventDefault();
		const comingSoonMessages: string[] = $_('comingSoon') as unknown as string[];
		e.currentTarget.textContent =
			comingSoonMessages[Math.floor(Math.random() * comingSoonMessages.length)];
	};
</script>

<header class="w-full">
	<h1 class="my-10 text-center text-5xl leading-[1.5] font-extrabold tracking-[0.01em]">
		{$_('landing.title')}
	</h1>
	<h2
		class="my-10 text-center text-3xl leading-[1.5] font-extrabold tracking-[0.01em] text-[#e18282]"
	>
		{$_('landing.subtitle')}
	</h2>
</header>
<main class="flex h-64 w-full items-stretch gap-10">
	<img src={morpheus} alt={$_('landing.images.morpheus')} class="h-64 rounded-md max-md:hidden" />
	<div class="flex w-[100%] flex-col gap-3">
		<Panel
			icon={IconCrown}
			title={$_('landing.choices.host.text')}
			bind:activePanel
			color="red"
			onsubmit={host}
		>
			<Button class="flex-grow-1">{$_('landing.choices.host.public')}</Button>
			<Button class="flex-grow-1" onclick={comingSoon}>{$_('landing.choices.host.private')}</Button>
			<Button class="flex-grow-1" onclick={comingSoon}>{$_('landing.choices.host.custom')}</Button>
		</Panel>
		<Panel
			icon={IconUserCircleOutline}
			title={$_('landing.choices.join.text')}
			bind:activePanel
			color="blue"
			onsubmit={join}
			active
		>
			<SearchBar
				label={$_('landing.choices.join.id.label')}
				placeholder={$_('landing.choices.join.id.placeholder')}
				submitTxt={$_('landing.choices.join.submit')}
				bind:value={code}
				error={joinError}
				oninput={() => (code = normalizeCode(code))}
			/>
		</Panel>
	</div>
</main>
