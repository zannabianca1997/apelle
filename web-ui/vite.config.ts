import tailwindcss from '@tailwindcss/vite';
import { sveltekit } from '@sveltejs/kit/vite';
import Icons from 'unplugin-icons/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [tailwindcss(), sveltekit(), Icons({ compiler: 'svelte' })],
	server: {
		allowedHosts: ['localhost', 'front_service'],
		port: 3000,
		hmr: {
			// Force the use of the 3000 port, that is different from the one
			// used by the server (reverse proxied by nginx)
			port: 3000
		}
	}
});
