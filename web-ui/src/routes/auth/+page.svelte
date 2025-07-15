<script lang="ts">
	import { _ } from 'svelte-i18n';
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import TextInput from '$lib/components/forms/TextInput.svelte';
	import { error } from '$lib/errors.svelte';
	import Form from './Form.svelte';
	import { Logger } from '$lib/logger';
	import type { AxiosBasicCredentials } from 'axios';
	import authService from '$lib/auth.svelte';
	import type { UserCreateDto } from '$lib/apis/apelle';
	import type { Snapshot } from './$types';

	const logger = new Logger('routes.auth');

	async function onsuccess() {
		const originalUrl = page.url.searchParams.get('original') ?? '/';

		logger.debug('Redirecting to original url', originalUrl);
		await goto(originalUrl);
	}

	let signinData: AxiosBasicCredentials = $state({ username: '', password: '' });

	async function signin(e: SubmitEvent) {
		e.preventDefault();

		logger.debug('Signing in');

		if (!signinData.username) {
			error({ _tag: 'usernameRequired', msg: $_('login.signinForm.errors.usernameRequired') });
			return;
		}

		if (!signinData.password) {
			error({ _tag: 'passwordRequired', msg: $_('login.signinForm.errors.passwordRequired') });
			return;
		}

		let result = await authService.signin(signinData);

		if (result.isFailure()) {
			error({
				...result.error,
				msg: $_(`login.signinForm.errors.${result.error._tag}`, { values: result.error })
			});
			return;
		}

		await onsuccess();
	}

	let signupData: UserCreateDto & { checkPassword: string } = $state({
		name: '',
		password: '',
		checkPassword: ''
	});
	let checkPasswordError: string | null = $derived(
		(signupData.password != signupData.checkPassword &&
			$_('login.signupForm.errors.passwordDoesNotMatch')) ||
			null
	);

	async function signup(e: SubmitEvent) {
		e.preventDefault();

		logger.debug('Signing up');

		if (!signupData.name) {
			error({ _tag: 'nameRequired', msg: $_('login.signupForm.errors.usernameRequired') });
			return;
		}

		if (!signupData.password) {
			error({ _tag: 'passwordRequired', msg: $_('login.signupForm.errors.passwordRequired') });
			return;
		}

		if (signupData.password != signupData.checkPassword) {
			error({
				_tag: 'passwordDoesNotMatch',
				msg: $_('login.signupForm.errors.passwordDoesNotMatch')
			});
			return;
		}

		let result = await authService.signup(signupData);

		if (result.isFailure()) {
			error({
				...result.error,
				msg: $_(`login.signupForm.errors.${result.error._tag}`, { values: result.error })
			});
			return;
		}

		await onsuccess();
	}

	export const snapshot: Snapshot<{
		signinData: AxiosBasicCredentials;
		signupData: UserCreateDto & { checkPassword: string };
	}> = {
		capture: () => ({
			signinData,
			signupData
		}),
		restore: (value) => {
			signinData = value.signinData;
			signupData = value.signupData;
		}
	};
</script>

<main class="flex flex-col gap-9 sm:flex-row">
	<Form color="red" onsubmit={signin}>
		<TextInput
			label={$_('login.signinForm.username')}
			bind:value={signinData.username}
			error={(!signinData.username && $_('login.signinForm.errors.usernameRequired')) || null}
		/>
		<TextInput
			label={$_('login.signinForm.password')}
			password
			bind:value={signinData.password}
			error={(!signinData.password && $_('login.signinForm.errors.passwordRequired')) || null}
		/>
		{#snippet submit()}{$_('login.signinForm.submit')}{/snippet}
	</Form>
	<Form color="blue" onsubmit={signup}>
		<TextInput
			label={$_('login.signupForm.username')}
			bind:value={signupData.name}
			error={(!signupData.name && $_('login.signupForm.errors.usernameRequired')) || null}
		/>
		<TextInput
			label={$_('login.signupForm.password')}
			password
			bind:value={signupData.password}
			error={(!signupData.password && $_('login.signupForm.errors.passwordRequired')) || null}
		/>
		<TextInput
			label={$_('login.signupForm.passwordCheck')}
			password
			bind:value={signupData.checkPassword}
			error={checkPasswordError}
		/>
		{#snippet submit()}{$_('login.signupForm.submit')}{/snippet}
	</Form>
</main>
