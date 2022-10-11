module.exports = {
    tracker: {
        input: './public/dist/openapi.json',
        output: {
            mode: 'split',
            target: 'web/api.ts',
            schemas: 'web/model',
            client: 'react-query',
        }
    },
};