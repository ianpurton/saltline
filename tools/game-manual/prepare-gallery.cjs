// Shared preparation for the manual and GLB export. All extracted output is ignored.
const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');
const {execFileSync} = require('node:child_process');
const {repo, output, reference} = require('./paths.cjs');
function fingerprint(files) {
  const hash = crypto.createHash('sha256');
  for (const file of files) hash.update(file).update(fs.readFileSync(path.join(repo, file)));
  return hash.digest('hex');
}
const inputs = ['references/index.html', 'tools/game-manual/rip-assets.cjs', 'tools/game-manual/registry.js', 'tools/game-manual/gallery.js', 'tools/game-manual/paths.cjs', 'tools/game-manual/prepare-gallery.cjs'];
function prepare() {
  const key = fingerprint(inputs), gallery = path.join(output, 'gallery'), stamp = path.join(gallery, 'cache-key');
  const provenance = path.join(gallery, 'provenance.json');
  let complete = false;
  if (fs.existsSync(provenance)) {
    const files = Object.keys(JSON.parse(fs.readFileSync(provenance)).files);
    complete = [...files, 'shared/reference.js', 'registry.js', 'gallery.js', 'index.html'].every(f => fs.existsSync(path.join(gallery, f)));
  }
  if (!complete || !fs.existsSync(stamp) || fs.readFileSync(stamp, 'utf8') !== key) {
    execFileSync(process.execPath, [path.join(__dirname, 'rip-assets.cjs')], {stdio:'inherit'});
    for (const file of ['registry.js', 'gallery.js']) fs.copyFileSync(path.join(__dirname, file), path.join(gallery, file));
    fs.writeFileSync(stamp, key);
  }
  return gallery;
}
module.exports = prepare;
module.exports.thumbnailKey = () => fingerprint([...inputs, 'tools/game-manual/render-thumbnails.cjs', 'package-lock.json']);
if (require.main === module) { prepare(); console.log('Prepared target/game-manual/gallery'); }
