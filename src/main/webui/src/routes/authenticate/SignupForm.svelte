<script lang="ts">
	import './form.scss';

	import { _ } from 'svelte-i18n';
	import TextInput from '$lib/components/forms/TextInput.svelte';
	import { error } from '$lib/errors.svelte';
	import authService from '$lib/auth.svelte';

	let { onsuccess = () => {} }: { onsuccess?: () => void } = $props();

	let username: string | null = $state(null);
	let usernameError: string | null = $state(null);

	let password: string | null = $state(null);
	let passwordError: string | null = $state(null);

	let passwordCheck: string | null = $state(null);
	let passwordCheckError: string | null = $state(null);

	async function onsubmit(e: Event) {
		e.preventDefault();
		usernameError = !username ? $_('login.signupForm.errors.usernameRequired') : null;
		passwordError = !password ? $_('login.signupForm.errors.passwordRequired') : null;
		passwordCheckError =
			password != passwordCheck ? $_('login.signupForm.errors.passwordDoesNotMatch') : null;
		if (!username || !password || password != passwordCheck) {
			return;
		}
		const signupResult = await authService.signup({ username, password });
		if (signupResult) {
			switch (signupResult.error) {
				case 'userExists':
					error('login.signupForm.errors.userExists');
					break;
			}
			return;
		}
		onsuccess();
	}
</script>

<form {onsubmit}>
	<input type="submit" value={$_('login.signupForm.submit')} />
	<TextInput label={$_('login.signupForm.username')} bind:value={username} error={usernameError} />
	<TextInput
		label={$_('login.signupForm.password')}
		isPassword
		bind:value={password}
		error={passwordError}
	/>
	<TextInput
		label={$_('login.signupForm.passwordCheck')}
		isPassword
		bind:value={passwordCheck}
		error={passwordCheckError}
	/>
</form>
