pub const PAGE: &str = r##"
<!DOCTYPE html>
<html lang="de">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>archive_search — Suchmaske</title>
<style>
  :root{
    --bg:#0e1520;
    --panel:#141d2c;
    --panel2:#1b2536;
    --line:#384a6b;
    --fg:#eaeef4;
    --dim:#8a94aa;
    --accent:#6f8fd0;
    --accent2:#b86838;
    --ok:#6aa88c;
    --warn:#e0a06e;
  }
  *{box-sizing:border-box;}
  html,body{margin:0;padding:0;}
  body{
    background:var(--bg);
    color:var(--fg);
    font-family:system-ui,-apple-system,"Segoe UI",Roboto,sans-serif;
    font-size:15px;
    line-height:1.45;
    min-height:100vh;
  }
  .wrap{
    max-width:980px;
    margin:0 auto;
    padding:18px 14px 40px;
  }
  header.top{
    display:flex;
    align-items:baseline;
    justify-content:space-between;
    gap:12px;
    flex-wrap:wrap;
    margin-bottom:14px;
  }
  h1{
    font-size:20px;
    margin:0;
    font-weight:600;
    letter-spacing:.2px;
  }
  h1 .sub{color:var(--dim);font-weight:400;font-size:14px;margin-left:8px;}
  #index{
    color:var(--dim);
    font-size:13px;
    white-space:nowrap;
  }
  form.search{
    display:flex;
    gap:8px;
    flex-wrap:wrap;
    align-items:center;
    margin-bottom:12px;
  }
  #q{
    flex:1 1 260px;
    min-width:180px;
    background:var(--panel);
    border:1px solid var(--line);
    color:var(--fg);
    border-radius:9px;
    padding:11px 13px;
    font-size:16px;
    outline:none;
  }
  #q:focus{border-color:var(--accent2);box-shadow:0 0 0 2px rgba(184,104,56,.22);}
  .limit{
    display:inline-flex;
    align-items:center;
    gap:6px;
    color:var(--dim);
    font-size:13.5px;
    white-space:nowrap;
  }
  #limit{
    width:72px;
    background:var(--panel);
    border:1px solid var(--line);
    color:var(--fg);
    border-radius:9px;
    padding:9px 8px;
    font-size:14px;
    outline:none;
  }
  #limit:focus{border-color:var(--accent2);}
  button{
    font-family:inherit;
    font-size:14px;
    color:var(--fg);
    background:var(--panel2);
    border:1px solid var(--line);
    border-radius:9px;
    padding:10px 14px;
    cursor:pointer;
  }
  button:hover{border-color:#4d6085;}
  button.primary{
    background:var(--accent2);
    border-color:var(--accent2);
    color:#fff;
    font-weight:600;
  }
  button.primary:hover{background:#c9773f;}
  label.live{
    display:inline-flex;
    align-items:center;
    gap:6px;
    color:var(--dim);
    font-size:14px;
    cursor:pointer;
    user-select:none;
    white-space:nowrap;
  }
  label.live input{accent-color:var(--accent2);}
  .chips{
    display:flex;
    flex-wrap:wrap;
    gap:7px;
    padding:4px 2px 10px;
  }
  .chip{
    flex:0 0 auto;
    border:1px solid var(--line);
    background:var(--panel);
    color:var(--dim);
    border-radius:999px;
    padding:6px 13px;
    font-size:13.5px;
    cursor:pointer;
    white-space:nowrap;
  }
  .chip:hover{color:var(--fg);border-color:#4d6085;}
  .chip.active{
    background:var(--accent2);
    border-color:var(--accent2);
    color:#fff;
    font-weight:600;
  }
  .chip.all{border-color:#4d6085;color:var(--fg);}
  .bar{
    display:flex;
    align-items:center;
    justify-content:space-between;
    gap:12px;
    flex-wrap:wrap;
    margin:6px 0 10px;
  }
  #status{color:var(--dim);font-size:13.5px;min-height:19px;}
  #status.busy{color:var(--accent);}
  #status.pending{color:var(--warn);}
  .actions{display:flex;gap:8px;}
  .actions button{font-size:13px;padding:7px 11px;}
  ul.results{
    list-style:none;
    margin:0;
    padding:0;
    border:1px solid var(--line);
    border-radius:10px;
    background:var(--panel);
    overflow:hidden;
  }
  ul.results:empty{display:none;}
  li.row{
    padding:10px 12px;
    border-bottom:1px solid #222e42;
    font-size:13.5px;
    word-break:break-word;
  }
  li.row:last-child{border-bottom:none;}
  li.row.web{cursor:default;}
  li.row.web:hover{background:var(--panel2);}
  li.row.local{cursor:pointer;}
  li.row.local:hover{background:var(--panel2);}
  .line1{
    display:flex;
    align-items:baseline;
    gap:10px;
  }
  a.link{
    color:var(--accent);
    text-decoration:none;
    font-weight:600;
  }
  a.link:hover{text-decoration:underline;}
  a.link.sub{font-weight:400;color:var(--accent);opacity:.85;}
  .hint{
    margin-left:auto;
    color:var(--ok);
    font-size:12px;
    opacity:.9;
    min-width:52px;
    text-align:right;
  }
  .fields{margin-top:5px;display:block;}
  .field{
    display:block;
    color:var(--fg);
    font-size:13px;
    padding-left:2px;
    line-height:1.5;
  }
  .field .fk{color:var(--dim);}
  .loc{color:var(--accent);font-weight:600;margin-right:8px;}
  .txt{color:var(--fg);}
  .plain{color:var(--dim);}
  li.row.section{background:var(--panel2);cursor:default;}
  .sect{color:var(--accent);font-weight:600;letter-spacing:.3px;}
  @media (max-width:560px){
    .wrap{padding:14px 10px 30px;}
    h1{font-size:18px;}
    #q{font-size:16px;}
  }
</style>
</head>
<body>
<div class="wrap">
  <header class="top">
    <h1>archive_search<span class="sub">Suchmaske</span></h1>
    <div id="index">Index: …</div>
  </header>

  <form class="search" id="form">
    <input id="q" type="text" autofocus autocomplete="off" spellcheck="false" placeholder="Suchbegriff…">
    <label class="limit">Limit <input id="limit" type="number" min="1" max="200" step="1" value="10"></label>
    <button class="primary" type="submit">Suchen</button>
    <label class="live"><input id="live" type="checkbox" checked> live</label>
  </form>

  <div class="chips" id="chips"></div>

  <div class="bar">
    <div id="status">bereit</div>
    <div class="actions">
      <button type="button" id="copyAll">Alles kopieren</button>
      <button type="button" id="clear">Leeren</button>
    </div>
  </div>

  <ul class="results" id="results"></ul>
</div>

<script>
(function(){
  "use strict";

  var FALLBACK = ["local","leads","index","git","verdict","arxiv","ads","ntrs","wayback","crossref","wiki","github","crates","librs","brave","mwmbl","datacite","zenodo","isc","openalex","pubmed","europepmc","semanticscholar","clinicaltrials","openfda","pubchem","uniprot","pdb","chembl","ensembl","entrez","ena","doaj","unpaywall","reactome","interpro","alphafold","supermag","heasarc"];
  var STORE_KEY = "archive_search_modes";
  var LIMIT_KEY = "archive_search_limit";

  var formEl = document.getElementById("form");
  var inputEl = document.getElementById("q");
  var limitEl = document.getElementById("limit");
  var liveEl = document.getElementById("live");
  var chipsEl = document.getElementById("chips");
  var statusEl = document.getElementById("status");
  var resultsEl = document.getElementById("results");
  var indexEl = document.getElementById("index");

  var modes = [];
  var selected = ["local"];
  var lastLines = [];
  var ctrls = [];
  var timer = null;
  var runId = 0;

  function copyText(text){
    if (navigator.clipboard && navigator.clipboard.writeText){
      return navigator.clipboard.writeText(text);
    }
    return new Promise(function(resolve, reject){
      var ta = document.createElement("textarea");
      ta.value = text;
      ta.style.position = "fixed";
      ta.style.top = "-1000px";
      ta.style.opacity = "0";
      document.body.appendChild(ta);
      ta.focus();
      ta.select();
      try {
        var ok = document.execCommand("copy");
        document.body.removeChild(ta);
        if (ok) { resolve(); } else { reject(new Error("copy")); }
      } catch (err) {
        document.body.removeChild(ta);
        reject(err);
      }
    });
  }

  function setStatus(text, cls){
    statusEl.textContent = text;
    statusEl.className = cls || "";
  }

  function abortAll(){
    for (var i = 0; i < ctrls.length; i++){
      try { ctrls[i].abort(); } catch (err) {}
    }
    ctrls = [];
  }

  function flash(hint){
    if (!hint){ return; }
    hint.textContent = "kopiert";
    if (hint._t){ clearTimeout(hint._t); }
    hint._t = setTimeout(function(){ hint.textContent = ""; }, 900);
  }

  function attachCopy(li, text, hint){
    li.addEventListener("click", function(ev){
      var t = ev.target;
      if (t && t.closest && t.closest("a")){ return; }
      copyText(text).then(function(){ flash(hint); }).catch(function(){});
    });
  }

  function makeHint(){
    var hint = document.createElement("span");
    hint.className = "hint";
    return hint;
  }

  function makeLink(href, label, sub){
    var a = document.createElement("a");
    a.className = "link" + (sub ? " sub" : "");
    a.href = href;
    a.target = "_blank";
    a.rel = "noopener noreferrer";
    a.textContent = label;
    return a;
  }

  function makeField(part){
    var sp = document.createElement("span");
    sp.className = "field";
    var kv = /^([A-Za-z_]+):\s*([\s\S]*)$/.exec(part);
    if (kv){
      var k = document.createElement("span");
      k.className = "fk";
      k.textContent = kv[1] + ": ";
      sp.appendChild(k);
      var v = kv[2];
      if (/^https?:\/\//.test(v)){
        sp.appendChild(makeLink(v, v, true));
      } else {
        sp.appendChild(document.createTextNode(v));
      }
    } else {
      sp.textContent = part;
    }
    return sp;
  }

  function makeRow(line){
    var secM = /^===\s+(.*?)\s+===$/.exec(line);
    if (secM){
      var liSec = document.createElement("li");
      liSec.className = "row section";
      var sect = document.createElement("span");
      sect.className = "sect";
      sect.textContent = secM[1];
      liSec.appendChild(sect);
      return liSec;
    }

    var urlM = /^url\s+(\S+)(?:\s+([\s\S]*))?$/.exec(line);
    if (urlM){
      var liWeb = document.createElement("li");
      liWeb.className = "row web";
      var head = document.createElement("div");
      head.className = "line1";
      head.appendChild(makeLink(urlM[1], urlM[1], false));
      var hintWeb = makeHint();
      head.appendChild(hintWeb);
      liWeb.appendChild(head);
      var rest = urlM[2] || "";
      if (rest){
        var fields = document.createElement("div");
        fields.className = "fields";
        var parts = rest.split("\t");
        for (var i = 0; i < parts.length; i++){
          if (!parts[i]){ continue; }
          fields.appendChild(makeField(parts[i]));
        }
        liWeb.appendChild(fields);
      }
      attachCopy(liWeb, line, hintWeb);
      return liWeb;
    }

    var localM = /^(\S.*?):(\d+):([\s\S]*)$/.exec(line);
    if (localM){
      var liLocal = document.createElement("li");
      liLocal.className = "row local";
      var loc = document.createElement("span");
      loc.className = "loc";
      loc.textContent = localM[1] + ":" + localM[2];
      liLocal.appendChild(loc);
      var txt = document.createElement("span");
      txt.className = "txt";
      txt.textContent = localM[3];
      liLocal.appendChild(txt);
      var hintLocal = makeHint();
      liLocal.appendChild(hintLocal);
      attachCopy(liLocal, line, hintLocal);
      return liLocal;
    }

    var liPlain = document.createElement("li");
    liPlain.className = "row";
    var plain = document.createElement("span");
    plain.className = "txt plain";
    plain.textContent = line;
    liPlain.appendChild(plain);
    var hintPlain = makeHint();
    liPlain.appendChild(hintPlain);
    attachCopy(liPlain, line, hintPlain);
    return liPlain;
  }

  function appendRows(container, lines){
    for (var i = 0; i < lines.length; i++){
      container.appendChild(makeRow(lines[i]));
    }
  }

  function currentLimit(){
    var n = parseInt(limitEl.value, 10);
    if (isNaN(n) || n < 1){ n = 1; }
    if (n > 200){ n = 200; }
    return n;
  }

  function run(){
    var q = inputEl.value.trim();
    abortAll();
    var myId = ++runId;
    if (!q){
      lastLines = [];
      resultsEl.innerHTML = "";
      setStatus("bereit");
      return;
    }
    var targets = selected.length ? selected.slice() : ["local"];
    var limit = currentLimit();
    var t0 = performance.now();
    setStatus("liest… (" + targets.length + ")", "busy");

    var jobs = targets.map(function(mode){
      var ctrl = new AbortController();
      ctrls.push(ctrl);
      var url = "/api/run?mode=" + encodeURIComponent(mode) + "&q=" + encodeURIComponent(q) + "&limit=" + limit;
      return fetch(url, {signal: ctrl.signal})
        .then(function(r){ if (!r.ok){ throw new Error("http"); } return r.json(); })
        .then(function(data){ return {mode: mode, lines: Array.isArray(data) ? data : []}; })
        .catch(function(err){ return {mode: mode, lines: [], aborted: !!(err && err.name === "AbortError")}; });
    });

    Promise.all(jobs).then(function(results){
      if (myId !== runId){ return; }
      var all = [];
      var total = 0;
      for (var i = 0; i < results.length; i++){
        if (results[i].aborted){ return; }
      }
      for (var j = 0; j < results.length; j++){
        var r = results[j];
        all.push("=== " + r.mode + " (" + r.lines.length + ") ===");
        all = all.concat(r.lines);
        total += r.lines.length;
      }
      lastLines = all;
      resultsEl.innerHTML = "";
      var frag = document.createDocumentFragment();
      appendRows(frag, all);
      resultsEl.appendChild(frag);
      var ms = Math.round(performance.now() - t0);
      setStatus(targets.length + " Quellen — " + total + " Treffer — " + ms + " ms");
    });
  }

  function scheduleLive(){
    if (timer){ clearTimeout(timer); }
    timer = setTimeout(run, 150);
  }

  function isLiveLocal(){
    return selected.length === 1 && selected[0] === "local" && liveEl.checked;
  }

  function saveState(){
    try {
      localStorage.setItem(STORE_KEY, JSON.stringify(selected));
      localStorage.setItem(LIMIT_KEY, String(currentLimit()));
    } catch (err) {}
  }

  function setSelected(next){
    selected = next;
    var all = chipsEl.querySelectorAll(".chip");
    for (var i = 0; i < all.length; i++){
      var m = all[i].getAttribute("data-mode");
      all[i].classList.toggle("active", m === "__all__" ? false : selected.indexOf(m) >= 0);
    }
    saveState();
  }

  function toggleMode(mode){
    var idx = selected.indexOf(mode);
    if (idx >= 0){ selected.splice(idx, 1); } else { selected.push(mode); }
    setSelected(selected);
    if (inputEl.value.trim()){ run(); }
  }

  function buildChips(){
    chipsEl.innerHTML = "";
    var allBtn = document.createElement("button");
    allBtn.type = "button";
    allBtn.className = "chip all";
    allBtn.setAttribute("data-mode", "__all__");
    allBtn.textContent = "alle";
    allBtn.addEventListener("click", function(){
      selected = modes.slice();
      setSelected(selected);
      if (inputEl.value.trim()){ run(); }
    });
    chipsEl.appendChild(allBtn);

    for (var i = 0; i < modes.length; i++){
      (function(mode){
        var b = document.createElement("button");
        b.type = "button";
        b.className = "chip" + (selected.indexOf(mode) >= 0 ? " active" : "");
        b.setAttribute("data-mode", mode);
        b.textContent = mode;
        b.addEventListener("click", function(){ toggleMode(mode); });
        chipsEl.appendChild(b);
      })(modes[i]);
    }
  }

  function loadModes(){
    fetch("/api/modes").then(function(r){
      if (!r.ok){ throw new Error("http"); }
      return r.json();
    }).then(function(data){
      if (Array.isArray(data)){
        modes = data.filter(function(x){ return typeof x === "string"; });
      }
      if (!modes.length){ modes = FALLBACK.slice(); }
      selected = selected.filter(function(m){ return modes.indexOf(m) >= 0; });
      if (!selected.length){ selected = ["local"]; }
      buildChips();
    }).catch(function(){
      modes = FALLBACK.slice();
      selected = selected.filter(function(m){ return modes.indexOf(m) >= 0; });
      if (!selected.length){ selected = ["local"]; }
      buildChips();
    });
  }

  function loadStatus(){
    fetch("/api/status").then(function(r){
      if (!r.ok){ throw new Error("http"); }
      return r.json();
    }).then(function(data){
      if (data && typeof data.entries === "number"){
        indexEl.textContent = "Index: " + data.entries + " Eintr\u00e4ge";
      } else {
        indexEl.textContent = "Index: pending";
      }
    }).catch(function(){
      indexEl.textContent = "Index: pending";
    });
  }

  formEl.addEventListener("submit", function(ev){
    ev.preventDefault();
    run();
  });

  inputEl.addEventListener("input", function(){
    if (isLiveLocal()){ scheduleLive(); }
  });

  liveEl.addEventListener("change", function(){
    if (isLiveLocal() && inputEl.value.trim()){ scheduleLive(); }
  });

  limitEl.addEventListener("change", function(){
    saveState();
    if (inputEl.value.trim()){ run(); }
  });

  document.getElementById("copyAll").addEventListener("click", function(){
    var text = lastLines.join("\n");
    copyText(text).then(function(){
      var b = document.getElementById("copyAll");
      var old = b.textContent;
      b.textContent = "kopiert";
      setTimeout(function(){ b.textContent = old; }, 900);
    }).catch(function(){});
  });

  document.getElementById("clear").addEventListener("click", function(){
    abortAll();
    if (timer){ clearTimeout(timer); }
    inputEl.value = "";
    lastLines = [];
    resultsEl.innerHTML = "";
    setStatus("bereit");
    inputEl.focus();
  });

  var savedModes = null;
  var savedLimit = null;
  try {
    savedModes = localStorage.getItem(STORE_KEY);
    savedLimit = localStorage.getItem(LIMIT_KEY);
  } catch (err) {}
  if (savedModes){
    try {
      var parsed = JSON.parse(savedModes);
      if (Array.isArray(parsed) && parsed.length){ selected = parsed.filter(function(x){ return typeof x === "string"; }); }
    } catch (err) {}
  }
  if (savedLimit){
    var ln = parseInt(savedLimit, 10);
    if (!isNaN(ln) && ln >= 1 && ln <= 200){ limitEl.value = String(ln); }
  }

  loadModes();
  loadStatus();
})();
</script>
</body>
</html>
"##;
