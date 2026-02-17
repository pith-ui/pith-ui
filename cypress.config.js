const { defineConfig } = require('cypress');

module.exports = defineConfig({
    viewportWidth: 1024,
    viewportHeight: 768,
    fixturesFolder: false,
    defaultCommandTimeout: 20_000,
    e2e: {
        testIsolation: false,
        setupNodeEvents(_on, _config) {},
        baseUrl: 'http://localhost:8080',
    },
});
