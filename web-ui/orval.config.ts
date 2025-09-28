import { Config, defineConfig } from 'orval';

const config: Config = {
    apelle: {
        input: { target: '../openapi.yml' },
        output: './src/lib/apis/apelle.ts',
        hooks: {
            afterAllFilesWrite: 'prettier --write'
        }
    }
};

export default defineConfig(config);
