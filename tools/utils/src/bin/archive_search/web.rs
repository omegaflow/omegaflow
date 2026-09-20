pub const PAGE: &str = r##"<!doctype html>
<html lang="de">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>archive_search</title>
<style>
:root { color-scheme: dark; }
* { box-sizing: border-box; }
body { margin: 0; font: 14px/1.4 system-ui, sans-serif; background: #14161a; color: #e6e6e6; }
header { display: flex; gap: 10px; align-items: center; flex-wrap: wrap; padding: 12px 16px; background: #1b1e24; border-bottom: 1px solid #2a2f38; position: sticky; top: 0; }
#q { flex: 1 1 320px; padding: 10px 12px; font-size: 16px; background: #0f1115; color: #e6e6e6; border: 1px solid #333a45; border-radius: 6px; }
button { background: #0f1115; color: #e6e6e6; border: 1px solid #333a45; border-radius: 6px; padding: 8px 12px; cursor: pointer; }
#bar { display: flex; gap: 10px; align-items: center; flex-wrap: wrap; padding: 8px 16px; border-bottom: 1px solid #2a2f38; }
#bar label { display: flex; gap: 4px; align-items: center; color: #aab; font-size: 12px; }
#status { padding: 8px 16px; color: #8b94a3; font-size: 12px; }
pre { margin: 0; padding: 12px 16px; white-space: pre-wrap; word-break: break-word; }
</style>
</head>
<body>
<header>
<input id="q" placeholder="Suchbegriff…" autocomplete="off">
<label><input type="checkbox" id="live" checked> live (lokal, 120 ms)</label>
<button id="submit">Suchen</button>
</header>
<div id="bar"></div>
<div id="status">bereit</div>
<pre id="out"></pre>
<script>
const $ = (id) => document.getElementById(id);
const MODES = ['local','leads','mft','index','git','verdict','arxiv','ads','ntrs','wayback','crossref','wiki','github','crates','librs','openalex','pubmed','europepmc','psychporta','awmf','cochrane','cod','biomodels','core','materialsproject','semanticscholar','clinicaltrials','openfda','pubchem','uniprot','pdb','chembl','ensembl','entrez','ena','doaj','go','unpaywall','reactome','interpro','alphafold'];
let active = 'local';
let timer = null;
const bar = $('bar');
for (const m of MODES) {
  const l = document.createElement('label');
  const c = document.createElement('input');
  c.type = 'checkbox';
  c.checked = (m === active);
  c.addEventListener('change', () => {
    if (!c.checked) { c.checked = true; return; }
    active = m;
    for (const o of bar.querySelectorAll('input')) { if (o !== c) o.checked = false; }
    run();
  });
  l.append(c, document.createTextNode(m));
  bar.append(l);
}
$('q').addEventListener('input', () => {
  clearTimeout(timer);
  if (active === 'local' && $('live').checked) { timer = setTimeout(run, 120); }
});
$('submit').addEventListener('click', run);
async function run() {
  $('status').textContent = 'liest…';
  try {
    const r = await fetch('/api/run?mode=' + encodeURIComponent(active) + '&q=' + encodeURIComponent($('q').value));
    const lines = await r.json();
    $('out').textContent = lines.join('\n');
    $('status').textContent = active + ' — ' + lines.length + ' Zeilen';
  } catch (e) { $('status').textContent = 'pending'; }
}
run();
</script>
</body>
</html>
"##;
