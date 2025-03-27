import { browser } from '$app/environment'
import '$lib/i18n' // Import to initialize. Important :)
import { locale, waitLocale } from 'svelte-i18n'
import type { LayoutLoad } from './$types'

/**
 * Sets the locale to the user's preferred language, as determined by the
 * browser, and waits for the locale to be ready.
 *
 * This is a layout load function, which means it will be called whenever the
 * user navigates between pages. It is used to set the locale when the user
 * first visits the site, and to wait for the locale to be ready before
 * rendering the page.
 *
 * If the user is not using a browser (i.e. the site is being prerendered by a
 * server), the locale is not set.
 */
export const load: LayoutLoad = async () => {
    if (browser) {
        locale.set(window.navigator.language)
    }
    await waitLocale();
}

// Create a static site
export const ssr = false