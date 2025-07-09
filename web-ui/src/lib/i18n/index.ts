import { browser } from '$app/environment';
import { Logger } from '$lib/logger';
import { init, register } from 'svelte-i18n';

const logger = new Logger('lib.i18n');

logger.info('Initializing i18n');

const defaultLocale = 'en';

register('en', () => {
	logger.debug('Loading english language');
	return import('./en.json');
});
register('it', () => {
	logger.debug('Loading italian language');
	return import('./it.json');
});

init({
	fallbackLocale: defaultLocale,
	initialLocale: window.navigator.language
});
