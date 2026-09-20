const { chromium } = require('playwright');
const path = require('path');
const os = require('os');
const fs = require('fs');

const CHALLENGE =
  /just a moment|nur einen moment|attention required|checking your browser|verifying you are human|enable javascript and cookies|ddos protection|sicherheitsüberprüfung|überprüfung erfolgreich|warten auf antwort|ray id/i;

const SAME_SITE = { strict: 'Strict', lax: 'Lax', none: 'None', no_restriction: 'None' };

async function loadCookies(context) {
  const cookiesPath = process.env.OMEGAFLOW_COOKIES;
  if (!cookiesPath) return 0;
  let raw;
  try {
    raw = fs.readFileSync(cookiesPath, 'utf8');
  } catch (e) {
    process.stderr.write('playwright_fetch: OMEGAFLOW_COOKIES set but unreadable: ' + cookiesPath + '\n');
    process.exit(2);
  }
  let parsed;
  try {
    parsed = JSON.parse(raw);
  } catch (e) {
    process.stderr.write('playwright_fetch: OMEGAFLOW_COOKIES is not valid JSON: ' + cookiesPath + '\n');
    process.exit(2);
  }
  const list = Array.isArray(parsed) ? parsed : parsed && Array.isArray(parsed.cookies) ? parsed.cookies : null;
  if (!list) {
    process.stderr.write('playwright_fetch: OMEGAFLOW_COOKIES must carry a Cookie-Editor JSON array: ' + cookiesPath + '\n');
    process.exit(2);
  }
  const cookies = [];
  for (const c of list) {
    if (!c || typeof c.name !== 'string' || typeof c.value !== 'string' || typeof c.domain !== 'string') continue;
    const cookie = { name: c.name, value: c.value, domain: c.domain, path: typeof c.path === 'string' ? c.path : '/' };
    if (!c.session && typeof c.expirationDate === 'number') cookie.expires = c.expirationDate;
    if (typeof c.httpOnly === 'boolean') cookie.httpOnly = c.httpOnly;
    if (typeof c.secure === 'boolean') cookie.secure = c.secure;
    const sameSite = typeof c.sameSite === 'string' ? SAME_SITE[c.sameSite.toLowerCase()] : undefined;
    if (sameSite) cookie.sameSite = sameSite;
    cookies.push(cookie);
  }
  if (cookies.length === 0) {
    process.stderr.write('playwright_fetch: OMEGAFLOW_COOKIES carried no usable cookies: ' + cookiesPath + '\n');
    process.exit(2);
  }
  await context.addCookies(cookies);
  return cookies.length;
}

async function main() {
  const input = process.argv[2];
  if (!input) {
    process.stderr.write('playwright_fetch: no input\n');
    process.exit(2);
  }
  const target = input;
  const kind = 'page';

  const proxy = process.env.OMEGAFLOW_PROXY || null;
  const headed = process.env.OMEGAFLOW_HEADED === '1';
  const profile =
    process.env.OMEGAFLOW_PROFILE || path.join(os.homedir(), '.cache/omegaflow/playwright-profile');
  const options = {
    channel: 'chrome',
    viewport: { width: 1280, height: 900 },
    ...(proxy ? { proxy: { server: proxy } } : {}),
  };

  let browser = null;
  let context = null;
  let page = null;
  if (headed) {
    context = await chromium.launchPersistentContext(profile, { ...options, headless: false });
    page = context.pages()[0] || (await context.newPage());
  } else {
    browser = await chromium.launch({ channel: 'chrome', headless: true, ...(proxy ? { proxy: { server: proxy } } : {}) });
    context = await browser.newContext({ viewport: options.viewport });
    page = await context.newPage();
  }

  const cookiesLoaded = await loadCookies(context);
  if (cookiesLoaded > 0) {
    process.stderr.write('playwright_fetch: ' + cookiesLoaded + ' cookies loaded from OMEGAFLOW_COOKIES\n');
  }

  let status = null;
  let challenged = false;
  page.on('response', (r) => {
    if (r.request().resourceType() === 'document' && r.frame() === page.mainFrame()) {
      status = r.status();
    }
  });
  try {
    await page.goto(target, { waitUntil: 'domcontentloaded', timeout: 45000 });
    await page.waitForLoadState('networkidle', { timeout: 15000 }).catch(() => {});
    const deadline = Date.now() + 60000;
    while (Date.now() < deadline) {
      const title = await page.title().catch(() => '');
      const body = await page
        .evaluate(() => (document.body ? document.body.innerText.slice(0, 600) : ''))
        .catch(() => '');
      challenged = CHALLENGE.test(title) || CHALLENGE.test(body);
      if (!challenged && status === 200) break;
      await page.waitForTimeout(1000);
    }
    if (challenged) {
      process.stderr.write(
        'playwright_fetch: the interstitial did not clear (a fresh profile is detected; retry with OMEGAFLOW_HEADED=1 on a display, the profile persists under ~/.cache/omegaflow/playwright-profile)\n',
      );
    }
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

  await context.close();
  if (browser) await browser.close();
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
  process.stdout.write(JSON.stringify({ kind, status, challenge: challenged, ...data }));
}

main().catch((e) => {
  process.stderr.write('playwright_fetch: ' + String(e && e.stack ? e.stack : e) + '\n');
  process.exit(1);
});
