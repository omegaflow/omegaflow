const { chromium } = require('playwright');

async function main() {
  const input = process.argv[2];
  if (!input) {
    process.stderr.write('playwright_fetch: no input\n');
    process.exit(2);
  }
  const target = input;
  const kind = 'page';

  const browser = await chromium.launch({ channel: 'chrome', headless: true });
  const page = await browser.newPage({ viewport: { width: 1280, height: 900 } });
  let status = null;
  try {
    const resp = await page.goto(target, { waitUntil: 'domcontentloaded', timeout: 45000 });
    status = resp ? resp.status() : null;
    await page.waitForLoadState('networkidle', { timeout: 15000 }).catch(() => {});
  } catch (e) {
    process.stderr.write('playwright_fetch: ' + String(e && e.message ? e.message : e) + '\n');
  }

  const data = await page.evaluate(() => {
    const norm = (s) => (s || '').replace(/\s+/g, ' ').trim();
    const title = norm(document.title);
    const metaDesc = document.querySelector('meta[name="description"]');
    const description = norm(metaDesc ? metaDesc.content : '');
    const headings = [];
    for (const h of Array.from(document.querySelectorAll('h1,h2,h3'))) {
      const t = norm(h.textContent);
      if (t.length > 2) headings.push(h.tagName.toLowerCase() + ': ' + t);
      if (headings.length >= 25) break;
    }
    const links = [];
    const seen = new Set();
    for (const a of Array.from(document.querySelectorAll('a[href]'))) {
      const href = a.href;
      if (!href || href.startsWith('javascript:') || seen.has(href)) continue;
      seen.add(href);
      links.push({ text: norm(a.textContent).slice(0, 140), href: href.slice(0, 400) });
      if (links.length >= 80) break;
    }
    const results = [];
    for (const r of Array.from(document.querySelectorAll('li.b_algo'))) {
      const a = r.querySelector('h2 a');
      const sn = r.querySelector('.b_caption p') || r.querySelector('p');
      if (a && a.href) {
        results.push({
          title: norm(a.textContent),
          href: a.href.slice(0, 400),
          snippet: norm(sn ? sn.textContent : '').slice(0, 240),
        });
      }
      if (results.length >= 15) break;
    }
    const text = norm(document.body ? document.body.innerText : '').slice(0, 5000);
    return { title, description, headings, links, results, text, url: location.href };
  });

  await browser.close();
  const decodeBing = (href) => {
    try {
      const u = new URL(href).searchParams.get('u');
      if (u && u.startsWith('a1')) {
        const b64 = u.slice(2).replace(/-/g, '+').replace(/_/g, '/');
        return Buffer.from(b64, 'base64').toString('utf8');
      }
    } catch (e) {
      return href;
    }
    return href;
  };
  if (Array.isArray(data.results)) {
    data.results = data.results.map((r) => ({ ...r, href: decodeBing(r.href) }));
  }
  process.stdout.write(JSON.stringify({ kind, status, ...data }));
}

main().catch((e) => {
  process.stderr.write('playwright_fetch: ' + String(e && e.stack ? e.stack : e) + '\n');
  process.exit(1);
});
