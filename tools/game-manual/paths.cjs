const path = require('node:path');
const repo = path.resolve(__dirname, '../..');
module.exports = {repo, output: path.join(repo, 'target/game-manual'), reference: path.join(repo, 'references/index.html')};
