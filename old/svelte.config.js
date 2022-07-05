const autoPreprocess = require('svelte-preprocess');
const path = require('path');

module.exports = {
  preprocess: autoPreprocess({ includePaths: [path.join(__dirname, 'node_modules')] }),
};
