pub const PAGE: &str = r##"
<!DOCTYPE html>
<html lang="de">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>archive_search — Suchmaske</title>
<style>
  :root{
    --bg:#0e1116;
    --panel:#151a21;
    --panel2:#1b222b;
    --line:#2a333f;
    --fg:#e6edf3;
    --dim:#8b98a5;
    --accent:#4da3ff;
    --accent2:#1f6feb;
    --ok:#3fb950;
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
    max-width:960px;
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
  #q:focus{border-color:var(--accent2);box-shadow:0 0 0 2px rgba(77,163,255,.18);}
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
  button:hover{border-color:#3a4653;}
  button.primary{
    background:var(--accent2);
    border-color:var(--accent2);
    color:#fff;
    font-weight:600;
  }
  button.primary:hover{background:#2a7ff2;}
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
    gap:7px;
    overflow-x:auto;
    padding:4px 2px 10px;
    scrollbar-width:thin;
  }
  .chips::-webkit-scrollbar{height:8px;}
  .chips::-webkit-scrollbar-thumb{background:#2a333f;border-radius:4px;}
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
  .chip:hover{color:var(--fg);border-color:#3a4653;}
  .chip.active{
    background:var(--accent2);
    border-color:var(--accent2);
    color:#fff;
    font-weight:600;
  }
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
  #status.pending{color:#d29922;}
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
    display:flex;
    align-items:baseline;
    gap:10px;
    padding:9px 12px;
    border-bottom:1px solid #202832;
    cursor:pointer;
    font-size:13.5px;
    word-break:break-word;
  }
  li.row:last-child{border-bottom:none;}
  li.row:hover{background:var(--panel2);}
  .loc{color:var(--accent);font-weight:600;flex:0 0 auto;}
  .txt{color:var(--fg);flex:1 1 auto;min-width:0;}
  .hint{
    flex:0 0 auto;
    color:var(--ok);
    font-size:12px;
    opacity:.9;
    min-width:52px;
    text-align:right;
  }
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

  var FALLBACK = ["local","leads","index","git","verdict","arxiv","ads","ntrs","wayback","crossref","wiki","github","crates","librs","openalex","pubmed","europepmc","semanticscholar","clinicaltrials","openfda","pubchem","uniprot","pdb","chembl","ensembl","entrez","ena","doaj","unpaywall","reactome","interpro","alphafold"];
  var STORE_KEY = "archive_search_mode";

  var formEl = document.getElementById("form");
  var inputEl = document.getElementById("q");
  var liveEl = document.getElementById("live");
  var chipsEl = document.getElementById("chips");
  var statusEl = document.getElementById("status");
  var resultsEl = document.getElementById("results");
  var indexEl = document.getElementById("index");

  var modes = [];
  var activeMode = "local";
  var lastLines = [];
  var ctrl = null;
  var timer = null;

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

  function renderResults(lines){
    lastLines = lines.slice();
    resultsEl.innerHTML = "";
    var frag = document.createDocumentFragment();
    for (var i = 0; i < lines.length; i++){
      frag.appendChild(makeRow(lines[i]));
    }
    resultsEl.appendChild(frag);
  }

  function makeRow(line){
    var li = document.createElement("li");
    li.className = "row";

    var m = /^(\S.*?):(\d+):(.*)$/.exec(line);
    if (m){
      var loc = document.createElement("span");
      loc.className = "loc";
      loc.textContent = m[1] + ":" + m[2];
      li.appendChild(loc);

      var txt = document.createElement("span");
      txt.className = "txt";
      txt.textContent = m[3];
      li.appendChild(txt);
    } else {
      var txt2 = document.createElement("span");
      txt2.className = "txt";
      txt2.textContent = line;
      li.appendChild(txt2);
    }

    var hint = document.createElement("span");
    hint.className = "hint";
    li.appendChild(hint);

    li.addEventListener("click", function(){
      copyText(line).then(function(){
        hint.textContent = "kopiert";
        if (hint._t){ clearTimeout(hint._t); }
        hint._t = setTimeout(function(){ hint.textContent = ""; }, 900);
      }).catch(function(){});
    });

    return li;
  }

  function run(){
    var q = inputEl.value.trim();
    if (ctrl){ ctrl.abort(); }
    if (!q){
      renderResults([]);
      setStatus("bereit — " + activeMode);
      return;
    }
    ctrl = new AbortController();
    var signal = ctrl.signal;
    var t0 = performance.now();
    setStatus("liest…", "busy");

    var url = "/api/run?mode=" + encodeURIComponent(activeMode) + "&q=" + encodeURIComponent(q);
    fetch(url, {signal: signal}).then(function(r){
      if (!r.ok){ throw new Error("http"); }
      return r.json();
    }).then(function(data){
      if (!Array.isArray(data)){ throw new Error("shape"); }
      var ms = Math.round(performance.now() - t0);
      renderResults(data);
      setStatus(activeMode + " — " + data.length + " Treffer — " + ms + " ms");
    }).catch(function(err){
      if (err && err.name === "AbortError"){ return; }
      renderResults([]);
      setStatus("pending", "pending");
    });
  }

  function scheduleLive(){
    if (timer){ clearTimeout(timer); }
    timer = setTimeout(run, 150);
  }

  function saveMode(){
    try { localStorage.setItem(STORE_KEY, activeMode); } catch (err) {}
  }

  function buildChips(){
    chipsEl.innerHTML = "";
    var entries = [{label:"alle", mode:"all"}];
    for (var i = 0; i < modes.length; i++){
      entries.push({label: modes[i], mode: modes[i]});
    }
    var known = entries.some(function(e){ return e.mode === activeMode; });
    if (!known){ activeMode = "local"; }

    entries.forEach(function(entry){
      var b = document.createElement("button");
      b.type = "button";
      b.className = "chip" + (entry.mode === activeMode ? " active" : "");
      b.textContent = entry.label;
      b.addEventListener("click", function(){
        activeMode = entry.mode;
        saveMode();
        var all = chipsEl.querySelectorAll(".chip");
        for (var k = 0; k < all.length; k++){
          all[k].classList.toggle("active", all[k] === b);
        }
        if (inputEl.value.trim()){ run(); }
      });
      chipsEl.appendChild(b);
    });
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
      buildChips();
    }).catch(function(){
      modes = FALLBACK.slice();
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
    if (activeMode === "local" && liveEl.checked){ scheduleLive(); }
  });

  liveEl.addEventListener("change", function(){
    if (activeMode === "local" && liveEl.checked && inputEl.value.trim()){ scheduleLive(); }
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
    if (ctrl){ ctrl.abort(); }
    if (timer){ clearTimeout(timer); }
    inputEl.value = "";
    renderResults([]);
    setStatus("bereit — " + activeMode);
    inputEl.focus();
  });

  var saved = null;
  try { saved = localStorage.getItem(STORE_KEY); } catch (err) {}
  if (saved){ activeMode = saved; }

  loadModes();
  loadStatus();
})();
</script>
</body>
</html>
"##;
