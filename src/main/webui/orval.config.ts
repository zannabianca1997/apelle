import { defineConfig } from 'orval';

export default defineConfig({
    'apelle': {
        input: './openapi/apelle.yaml',
        output: './src/lib/clients/apelle.ts',
        hooks: {
            afterAllFilesWrite: 'prettier --write',
        },
    },
});