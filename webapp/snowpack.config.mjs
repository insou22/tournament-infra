/** @type {import("snowpack").SnowpackUserConfig } */
export default {
    mount: {
        public: {url: '/', static: true},
        "src/client": {url: '/dist/client'},
        "src/shared": {url: '/dist/shared'},
    },
    plugins: [
        '@snowpack/plugin-react-refresh',
        '@snowpack/plugin-dotenv',
        'snowpack-plugin-markdown',
        [
            '@snowpack/plugin-typescript',
            {
                /* Yarn PnP workaround: see https://www.npmjs.com/package/@snowpack/plugin-typescript */
                ...(process.versions.pnp ? {tsc: 'yarn pnpify tsc'} : {})
            },
        ],
    ],
    routes: [
        /* Enable an SPA Fallback in development: */
        {"match": "routes", "src": ".*", "dest": "/index.html"},
    ],
    optimize: {
        /* Example: Bundle your final build: */
        bundle: false,
    },
    packageOptions: {
        /* ... */
    },
    devOptions: {
        /* ... */
    },
    buildOptions: {
        /* ... */
    },
    alias: {
        "@client": './src/client',
        "@server": './src/server',
        "@shared": './src/shared',
    }
}
