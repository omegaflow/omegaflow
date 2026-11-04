const PARSEC_M = 3.085677581e16;
const SIMBAD = "https://simbad.cds.unistra.fr/simbad/sim-id?output.format=JSON&Ident=";

let sourcesPromise = null;
let results = [];
let active = -1;
let seq = 0;
let timer = null;

const style = document.createElement("style");
style.textContent = `
#palette { position: fixed; inset: 0; z-index: 20; display: flex; align-items: flex-start; justify-content: center; padding-top: 16vh; background: rgba(0, 0, 0, 0.6); }
#palette[hidden] { display: none; }
#palette .card { width: min(40rem, 92vw); background: #0a0908; border: 1px solid #2a2622; }
#palette input { width: 100%; background: none; border: none; border-bottom: 1px solid #2a2622; color: #f0e8d8; font: inherit; font-size: 1rem; letter-spacing: 0.04em; padding: 0.9rem 1rem; outline: none; }
#palette ul { list-style: none; max-height: 42vh; overflow-y: auto; }
#palette li { display: flex; justify-content: space-between; gap: 1rem; padding: 0.55rem 1rem; font-size: 0.85rem; border-bottom: 1px solid #14110e; cursor: pointer; }
#palette li[aria-selected="true"] { background: #17130d; }
#palette .body { color: #c4a45a; }
#palette .status { min-height: 1rem; padding: 0.55rem 1rem; font-size: 0.78rem; color: #9c9284; border-top: 1px solid #2a2622; }
`;
document.head.append(style);

const overlay = document.createElement("div");
overlay.id = "palette";
overlay.hidden = true;
overlay.innerHTML = `<div class="card"><input type="text" autocomplete="off" spellcheck="false" placeholder="search source or object"><ul></ul><div class="status"></div></div>`;
document.body.append(overlay);

const input = overlay.querySelector("input");
const list = overlay.querySelector("ul");
const statusEl = overlay.querySelector(".status");

function sources() {
  if (sourcesPromise === null) {
    sourcesPromise = fetch("/sources")
      .then((r) => (r.ok ? r.json() : []))
      .then((v) => (Array.isArray(v) ? v : []))
      .catch(() => []);
  }
  return sourcesPromise;
}

function matchScore(hay, q) {
  const h = hay.toLowerCase();
  const i = h.indexOf(q);
  if (i >= 0) {
    return 1000 - i;
  }
  let k = 0;
  for (let j = 0; j < h.length && k < q.length; j++) {
    if (h[j] === q[k]) {
      k += 1;
    }
  }
  return k === q.length ? 1 : null;
}

function localMatches(srcs, q) {
  if (q === "") {
    return srcs.map((s) => ({ source: s, score: 0 }));
  }
  const out = [];
  for (const s of srcs) {
    const name = matchScore(s.name, q);
    const body = s.body ? matchScore(s.body, q) : null;
    if (name === null && body === null) {
      continue;
    }
    out.push({ source: s, score: Math.max(name ?? -1, body ?? -1) });
  }
  return out.sort((a, b) => b.score - a.score);
}

function setStatus(text) {
  statusEl.textContent = text;
}

function select(index) {
  active = index;
  for (const li of list.children) {
    li.setAttribute("aria-selected", String(Number(li.dataset.index) === index));
  }
}

function render(rows) {
  results = rows;
  list.replaceChildren();
  for (let i = 0; i < rows.length; i++) {
    const row = rows[i];
    const li = document.createElement("li");
    li.dataset.index = String(i);
    const label = document.createElement("span");
    const right = document.createElement("span");
    right.className = "body";
    if (row.source) {
      label.textContent = row.source.name;
      right.textContent = row.source.body ? row.source.body : "no body";
    } else {
      label.textContent = row.object.main_id;
      right.textContent = "ra " + row.object.ra.toFixed(4) + " dec " + row.object.dec.toFixed(4);
    }
    li.append(label, right);
    list.append(li);
  }
  select(rows.length > 0 ? 0 : -1);
}

function open() {
  overlay.hidden = false;
  input.value = "";
  input.focus();
  refresh("");
}

function close() {
  overlay.hidden = true;
  list.replaceChildren();
  results = [];
  active = -1;
  setStatus("");
}

async function simbad(ident) {
  let r;
  try {
    r = await fetch(SIMBAD + encodeURIComponent(ident));
  } catch {
    return null;
  }
  if (!r.ok) {
    return null;
  }
  let json;
  try {
    json = await r.json();
  } catch {
    return null;
  }
  const data = Array.isArray(json.data) ? json.data[0] : json.data;
  if (!data) {
    return null;
  }
  const ra = typeof data.ra === "number" ? data.ra : NaN;
  const dec = typeof data.dec === "number" ? data.dec : NaN;
  if (!Number.isFinite(ra) || !Number.isFinite(dec)) {
    return null;
  }
  const plx = typeof data.plx_value === "number" && Number.isFinite(data.plx_value) && data.plx_value > 0 ? data.plx_value : null;
  return { main_id: data.main_id || data.id || ident, ra, dec, plx };
}

async function refresh(q) {
  const mine = ++seq;
  const query = q.trim().toLowerCase();
  const srcs = await sources();
  if (mine !== seq) {
    return;
  }
  const matches = localMatches(srcs, query);
  if (matches.length > 0 || query === "") {
    render(matches.map((m) => ({ source: m.source })));
    setStatus(query === "" ? "" : matches.length + " local source" + (matches.length === 1 ? "" : "s"));
    return;
  }
  render([]);
  setStatus("no local source — querying SIMBAD…");
  const object = await simbad(query);
  if (mine !== seq) {
    return;
  }
  if (object === null) {
    setStatus("SIMBAD unreachable — /sources stays the substrate");
    return;
  }
  render([{ object }]);
  setStatus("SIMBAD " + object.main_id + (object.plx === null ? " — no parallax" : " — parallax " + object.plx + " mas"));
}

function choose() {
  const row = results[active];
  if (!row) {
    return;
  }
  if (row.source) {
    if (row.source.body) {
      window.omegaflow.jumpBody(row.source.body);
      setStatus("presence at " + row.source.body);
    } else {
      setStatus(row.source.name + " has no body — no jump");
    }
    return;
  }
  const o = row.object;
  if (o.plx === null) {
    setStatus(o.main_id + " ra " + o.ra.toFixed(4) + " dec " + o.dec.toFixed(4) + " — no parallax, no jump");
    return;
  }
  const ra = (o.ra * Math.PI) / 180;
  const dec = (o.dec * Math.PI) / 180;
  const d = (1000 / o.plx) * PARSEC_M;
  const x = Math.cos(dec) * Math.cos(ra) * d;
  const y = Math.cos(dec) * Math.sin(ra) * d;
  const z = Math.sin(dec) * d;
  window.omegaflow.jumpTo(x, y, z);
  setStatus("presence at " + o.main_id);
}

input.addEventListener("input", () => {
  clearTimeout(timer);
  const q = input.value;
  timer = setTimeout(() => refresh(q), 120);
});

overlay.addEventListener("keydown", (e) => {
  e.stopPropagation();
  if (e.key === "Escape") {
    e.preventDefault();
    close();
    return;
  }
  if (e.key === "ArrowDown") {
    e.preventDefault();
    if (results.length > 0) {
      select((active + 1) % results.length);
    }
    return;
  }
  if (e.key === "ArrowUp") {
    e.preventDefault();
    if (results.length > 0) {
      select((active - 1 + results.length) % results.length);
    }
    return;
  }
  if (e.key === "Enter") {
    e.preventDefault();
    choose();
  }
});

overlay.addEventListener("keyup", (e) => e.stopPropagation());

list.addEventListener("click", (e) => {
  const li = e.target.closest("li");
  if (!li) {
    return;
  }
  select(Number(li.dataset.index));
  choose();
});

overlay.addEventListener("mousedown", (e) => {
  if (e.target === overlay) {
    close();
  }
});

window.addEventListener("keydown", (e) => {
  if (!overlay.hidden) {
    return;
  }
  const k = e.key.toLowerCase();
  if ((e.metaKey || e.ctrlKey) && k === "k") {
    e.preventDefault();
    open();
    return;
  }
  if (e.ctrlKey && e.shiftKey && k === "p") {
    e.preventDefault();
    open();
  }
});
