<script lang="ts">
	import './form.scss';

	import { _ } from 'svelte-i18n';
	import TextInput from '$lib/components/forms/TextInput.svelte';
	import { signup } from '$lib/authenticate/signup';

	let username: string | null = null;
	let usernameError: string | null = null;

	let password: string | null = null;
	let passwordError: string | null = null;

	let passwordCheck: string | null = null;
	let passwordCheckError: string | null = null;

	let signupError: string | null = null;

	async function onSubmit() {
		usernameError = !username ? $_('login.signupForm.errors.usernameRequired') : null;
		passwordError = !password ? $_('login.signupForm.errors.passwordRequired') : null;
		passwordCheckError =
			password != passwordCheck ? $_('login.signupForm.errors.passwordDoesNotMatch') : null;
		if (!!usernameError || !!passwordError || !!passwordCheckError) {
			signupError = null;
			return;
		}
		const signupResult = await signup(username!, password!);
		signupError = signupResult?.error || null;
	}
</script>

<form on:submit|preventDefault={onSubmit}>
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
	{#if signupError}
		<em class="error">{signupError}</em>
	{/if}
</form>

<style lang="scss">
	form {
		--form-color: var(--red-pill);
	}
</style>
