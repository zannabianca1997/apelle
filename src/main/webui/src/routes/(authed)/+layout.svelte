<script lang="ts">
	import { _ } from 'svelte-i18n';
	import NavBar from '$lib/components/navbar/NavBar.svelte';
	import NavbarButton from '$lib/components/navbar/NavBarButton.svelte';
	import authService from '$lib/auth.svelte';
	import IconMusicNote from '~icons/mdi/music-note';
	import IconLogout from '~icons/mdi/logout-variant';
	import { goto } from '$app/navigation';
	import type { Snippet } from 'svelte';
	import { PageNavBar } from '$lib/components/navbar/stores';

	let { children }: { children: Snippet } = $props();

	async function home() {
		await goto('/');
	}
	async function logout() {
		await authService.signout();
		await goto('/authenticate');
	}
</script>

<NavBar>
	<NavbarButton icon={IconMusicNote} onclick={home}>{$_('navbar.title')}</NavbarButton>
	{@render $PageNavBar?.()}
	<NavbarButton icon={IconLogout} onclick={logout}>{$_('navbar.logout')}</NavbarButton>
</NavBar>
{@render children()}
