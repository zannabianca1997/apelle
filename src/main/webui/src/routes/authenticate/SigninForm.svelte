<script lang="ts">
	import { _ } from 'svelte-i18n';
	import TextInput from '$lib/components/forms/TextInput.svelte';
	import authService from '$lib/auth.svelte';
	import { error } from '$lib/errors.svelte';

	let { onsuccess = () => {} }: { onsuccess?: () => void } = $props();

	let username: string | null = $state(null);
	let usernameError: string | null = $state(null);

	let password: string | null = $state(null);
	let passwordError: string | null = $state(null);

	async function onsubmit(e: Event) {
		e.preventDefault();

		usernameError = !username ? $_('login.signinForm.errors.usernameRequired') : null;
		passwordError = !password ? $_('login.signinForm.errors.passwordRequired') : null;
		if (!username || !password) {
			return;
		}
		const signinResult = await authService.signin({ username, password });
		if (signinResult) {
			switch (signinResult.error) {
				case 'badCredentials':
					error($_('login.signinForm.errors.badCredentials'));
					break;
			}
			return;
		}
		onsuccess();
	}
</script>

<form {onsubmit}>
	<TextInput label={$_('login.signinForm.username')} bind:value={username} error={usernameError} />
	<TextInput
		label={$_('login.signinForm.password')}
		isPassword
		bind:value={password}
		error={passwordError}
	/>
	<button>{$_('login.signinForm.submit')}</button>
</form>

<style lang="scss">
	@import './form.scss';
</style>
