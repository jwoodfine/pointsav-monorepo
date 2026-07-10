// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

// bim.js — partial-page navigation, SSE hot-reload, SchemaState
// Hand-written; no HTMX dependency.

// ── Navigation ─────────────────────────────────────────────────────────────

function getMain() {
  return document.getElementById('bim-main-content');
}

async function navigate(path) {
  try {
    const res = await fetch('/fragment' + path, {
      headers: { 'X-Fragment': '1' },
    });
    if (!res.ok) {
      // Not every route has a /fragment/* counterpart (e.g. home, /about,
      // /disclaimers, /search — only tokens/tokens-detail/research do).
      // A missing fragment used to silently do nothing on click; fall back
      // to a real navigation instead, same as the network-failure path
      // below.
      window.location.href = path;
      return;
    }
    const html = await res.text();
    const main = getMain();
    if (main) {
      main.innerHTML = html;
      // Re-run any module scripts in the new content
      main.querySelectorAll('script[type="module"]').forEach((old) => {
        const next = document.createElement('script');
        next.type = 'module';
        if (old.src) {
          next.src = old.src;
        } else {
          next.textContent = old.textContent;
        }
        old.replaceWith(next);
      });
    }
    history.pushState({}, '', path);
    syncActiveNav(path);
  } catch (_) {
    // Let normal navigation proceed on network failure
    window.location.href = path;
  }
}

function syncActiveNav(path) {
  document.querySelectorAll('[data-path]').forEach((el) => {
    const match = el.dataset.path === path;
    el.setAttribute('aria-current', match ? 'page' : 'false');
    el.closest('[aria-current]')?.setAttribute('aria-current', match ? 'page' : 'false');
  });
}

// Intercept clicks on any element with data-path or .bim-nav-link
document.addEventListener('click', (e) => {
  const link = e.target.closest('[data-path], .bim-nav-link[href]');
  if (!link) return;
  const path = link.dataset.path || link.getAttribute('href');
  if (!path || path.startsWith('http') || path.startsWith('//')) return;
  // Only intercept same-origin paths that start with /
  if (!path.startsWith('/')) return;
  // Skip download and external links
  if (path.includes('/download/') || path.includes('.zip') || path.includes('.ifc')) return;
  e.preventDefault();
  navigate(path);
});

// Browser back/forward
window.addEventListener('popstate', () => {
  navigate(location.pathname);
});

// ── Envelope jurisdiction-overlay toggle ────────────────────────────────────
// Homepage envelope diagram: switches which pre-rendered overlay-state frame
// is visible (municipal / +provincial / +accessibility). Each frame is a
// complete SVG (render::envelope generates one per state) rather than one
// shared SVG with toggled sub-groups, since the tiers' shapes genuinely
// differ between states, not just their visibility.

document.addEventListener('click', (e) => {
  const btn = e.target.closest('.bim-envelope__overlay-btn');
  if (!btn) return;
  const key = btn.dataset.overlayTarget;
  const envelope = btn.closest('.bim-envelope');
  if (!envelope) return;
  envelope.setAttribute('data-active-overlay', key);
  envelope.querySelectorAll('.bim-envelope__frame').forEach((frame) => {
    frame.hidden = frame.dataset.overlay !== key;
  });
  envelope.querySelectorAll('.bim-envelope__overlay-btn').forEach((b) => {
    b.setAttribute('aria-pressed', b === btn ? 'true' : 'false');
  });
});

// ── Theme toggle ──────────────────────────────────────────────────────────

function syncThemeControls(theme) {
  document.querySelectorAll('.bim-theme-toggle').forEach((btn) => {
    btn.setAttribute('aria-pressed', theme === 'dark' ? 'true' : 'false');
    btn.setAttribute('aria-label', theme === 'dark' ? 'Switch to light theme' : 'Switch to dark theme');
  });
}

document.addEventListener('click', (e) => {
  const toggle = e.target.closest('.bim-theme-toggle');
  if (!toggle || toggle.disabled) return;
  const current = document.documentElement.getAttribute('data-theme') === 'dark' ? 'dark' : 'light';
  const next = current === 'dark' ? 'light' : 'dark';
  document.documentElement.setAttribute('data-theme', next);
  try { localStorage.setItem('bim-theme', next); } catch (_) {}
  syncThemeControls(next);
});

syncThemeControls(document.documentElement.getAttribute('data-theme') === 'dark' ? 'dark' : 'light');

// ── Force Important Information open when printing ─────────────────────────
// A CSS-only `display: block !important` on the closed <details> body is not
// reliable across Chromium's print-to-PDF path (verified: getComputedStyle
// reports the body as visible/non-zero-height under print media, but the
// actual PDF output omits it) — so open the element for real via the DOM
// attribute, which every engine honors, and restore whatever state the
// reader had it in afterward.
let disclosureWasOpen = null;
window.addEventListener('beforeprint', () => {
  const details = document.querySelector('.bim-disclosure__details');
  if (!details) return;
  disclosureWasOpen = details.open;
  details.open = true;
});
window.addEventListener('afterprint', () => {
  const details = document.querySelector('.bim-disclosure__details');
  if (!details || disclosureWasOpen === null) return;
  details.open = disclosureWasOpen;
  disclosureWasOpen = null;
});

// ── SSE hot-reload ──────────────────────────────────────────────────────────

let sseRetries = 0;

function connectSSE() {
  const evs = new EventSource('/api/events');
  evs.onopen = () => { sseRetries = 0; };
  evs.onmessage = (e) => {
    try {
      const msg = JSON.parse(e.data);
      if (msg.event === 'token-updated') {
        // Re-fetch current fragment to show updated content
        navigate(location.pathname);
      }
    } catch (_) {}
  };
  evs.onerror = () => {
    evs.close();
    sseRetries++;
    const delay = Math.min(1000 * Math.pow(2, sseRetries), 30000);
    setTimeout(connectSSE, delay);
  };
}

connectSSE();

// ── SchemaState ─────────────────────────────────────────────────────────────
// Bidirectional state between the visual pane and the CodeMirror code pane.

const SchemaState = {
  data: {},
  _listeners: [],

  get(path) {
    return path.split('.').reduce((o, k) => (o != null ? o[k] : undefined), this.data);
  },

  set(path, value) {
    const parts = path.split('.');
    const last = parts.pop();
    const target = parts.reduce((o, k) => (o[k] = o[k] || {}), this.data);
    target[last] = value;
    this._notify();
  },

  replace(obj) {
    this.data = obj;
    this._notify();
  },

  subscribe(fn) {
    this._listeners.push(fn);
    return () => {
      this._listeners = this._listeners.filter((f) => f !== fn);
    };
  },

  _notify() {
    this._listeners.forEach((fn) => fn(this.data));
  },
};

// ── Mobile nav drawer ────────────────────────────────────────────────────
// The drawer itself is a native <details class="bim-drawer">, so open/close
// already works with zero JavaScript (the <summary> is the hamburger
// button). This just layers on the affordances native <details> doesn't
// give for free: tap-outside-to-close, an explicit close button, and
// Escape — plus locking page scroll behind the open panel.

document.querySelectorAll('details.bim-drawer').forEach((drawer) => {
  drawer.addEventListener('toggle', () => {
    document.body.classList.toggle('bim-drawer-open', drawer.open);
  });
});

document.addEventListener('click', (e) => {
  const closer = e.target.closest('.bim-drawer__backdrop, .bim-drawer__close');
  if (!closer) return;
  const drawer = closer.closest('details.bim-drawer');
  if (drawer) drawer.open = false;
});

document.addEventListener('keydown', (e) => {
  if (e.key !== 'Escape') return;
  document.querySelectorAll('details.bim-drawer[open]').forEach((drawer) => {
    drawer.open = false;
  });
});

// ── Key-plan draw-on animation ──────────────────────────────────────────
// The composition detail page's main key-plan SVG "inks itself in" via
// stroke-dashoffset instead of appearing instantly. Only every stroked
// line-work element (walls, zone-boundary dashes, mullion ticks, furniture
// symbol outlines) animates — filled areas (zone washes) render immediately
// as normal, since dasharray only affects the stroke, which reads as
// "the color settles in, then the line-work draws itself" rather than a
// jarring reveal. Catalog thumbnail SVGs (/objects, /compositions grids)
// are deliberately left static — a stated scope cut: animating dozens of
// small on-screen SVGs at once cost real paint time for no real payoff: the
// single large plan on the detail page is the one moment that reads as an
// actual "drawing."
function drawOnAnimate(svg) {
  if (!svg || svg.dataset.drawnOn === '1') return;
  svg.dataset.drawnOn = '1';
  if (window.matchMedia && window.matchMedia('(prefers-reduced-motion: reduce)').matches) return;
  const shapes = svg.querySelectorAll('path, rect, line, circle, polyline, polygon');
  shapes.forEach((el, i) => {
    const stroke = el.getAttribute('stroke');
    if (!stroke || stroke === 'none') return;
    if (typeof el.getTotalLength !== 'function') return;
    let len;
    try { len = el.getTotalLength(); } catch (_) { return; }
    if (!len || !isFinite(len)) return;
    el.style.strokeDasharray = String(len);
    el.style.strokeDashoffset = String(len);
    el.style.transition = 'stroke-dashoffset 0.6s ease';
    el.style.transitionDelay = Math.min(i, 40) * 12 + 'ms';
    // Two nested rAFs: the first commits the dashoffset-at-full-length
    // style so the browser has actually painted that starting state before
    // the second rAF flips it to 0 — setting both in the same frame would
    // let the browser coalesce them and skip straight to the end state.
    requestAnimationFrame(() => {
      requestAnimationFrame(() => {
        el.style.strokeDashoffset = '0';
      });
    });
  });
}

function initPlanDrawOn() {
  const svg = document.querySelector('.bim-inspector-page__plan .bim-kp-diagram');
  if (!svg) return;
  if ('IntersectionObserver' in window) {
    const io = new IntersectionObserver((entries) => {
      entries.forEach((entry) => {
        if (entry.isIntersecting) {
          drawOnAnimate(svg);
          io.disconnect();
        }
      });
    }, { threshold: 0.15 });
    io.observe(svg);
  } else {
    drawOnAnimate(svg);
  }
}
initPlanDrawOn();

// ── Plan <-> parts-list hover linkage ───────────────────────────────────
// Hovering a "PARTS LIST" row highlights its paired furniture group on the
// plan SVG, and vice versa. Pairing is a shared `data-plan-obj` ordinal
// assigned at render time (render/svg.rs, render/catalog.rs's bill_html) —
// see the notes there.
function setPlanHighlight(idx, on) {
  if (idx == null) return;
  document.querySelectorAll('.bim-plan-obj[data-plan-obj="' + idx + '"]').forEach((g) => {
    g.classList.toggle('bim-plan-obj--hi', on);
  });
  document.querySelectorAll('.bim-bill-row[data-plan-obj="' + idx + '"]').forEach((r) => {
    r.classList.toggle('bim-bill-row--hi', on);
  });
}

document.addEventListener('mouseover', (e) => {
  const row = e.target.closest('.bim-bill-row[data-plan-obj]');
  if (row) { setPlanHighlight(row.dataset.planObj, true); return; }
  const grp = e.target.closest('.bim-plan-obj[data-plan-obj]');
  if (grp) setPlanHighlight(grp.dataset.planObj, true);
});
document.addEventListener('mouseout', (e) => {
  const row = e.target.closest('.bim-bill-row[data-plan-obj]');
  if (row) { setPlanHighlight(row.dataset.planObj, false); return; }
  const grp = e.target.closest('.bim-plan-obj[data-plan-obj]');
  if (grp) setPlanHighlight(grp.dataset.planObj, false);
});

// Tap/click parity (Round 5, 2026-07-10): the hover linkage above has no
// touch equivalent — `mouseover`/`mouseout` don't fire the same way on a
// tap, so this flagship interaction was dead on every phone and tablet
// (live-verified finding). Tap-to-pin instead of tap-to-preview: tapping a
// row or plan group toggles a persistent highlight (independent of the
// hover classes above, so it can't get stuck mid-hover-state on a device
// with no real pointer); tapping the same one again, or tapping elsewhere
// on the page, clears it. This also benefits keyboard/mouse users as a
// "pin it" affordance layered on top of hover, not a replacement for it.
function clearPinnedPlanHighlight() {
  document.querySelectorAll('.bim-plan-obj--hi-pinned, .bim-bill-row--hi-pinned').forEach((el) => {
    el.classList.remove('bim-plan-obj--hi-pinned', 'bim-bill-row--hi-pinned');
  });
}
function setPinnedPlanHighlight(idx) {
  document.querySelectorAll('.bim-plan-obj[data-plan-obj="' + idx + '"]').forEach((g) => {
    g.classList.add('bim-plan-obj--hi-pinned');
  });
  document.querySelectorAll('.bim-bill-row[data-plan-obj="' + idx + '"]').forEach((r) => {
    r.classList.add('bim-bill-row--hi-pinned');
  });
}
document.addEventListener('click', (e) => {
  const row = e.target.closest('.bim-bill-row[data-plan-obj]');
  const grp = e.target.closest('.bim-plan-obj[data-plan-obj]');
  // Bug found + fixed same session (Round 5 final verification): a linked
  // bill row (`.bim-bill-row--linked`) is a real `<a href>` to the object's
  // own page — the original version of this handler tried to pin AND never
  // called preventDefault, so the click just navigated away every time and
  // the pin never visibly applied. Fix: a linked row's tap keeps its
  // existing, valuable "go see the object" navigation untouched — it does
  // not also try to pin. Only the plan-SVG groups (never navigational) and
  // unlinked rows (plain <div>, nothing to navigate to) get the tap-to-pin
  // behavior, which is where real touch parity was actually missing.
  if (row && row.tagName === 'A') return;
  const target = row || grp;
  if (!target) { clearPinnedPlanHighlight(); return; }
  const idx = target.dataset.planObj;
  const wasPinned = target.classList.contains('bim-plan-obj--hi-pinned')
    || target.classList.contains('bim-bill-row--hi-pinned');
  clearPinnedPlanHighlight();
  if (!wasPinned) setPinnedPlanHighlight(idx);
});

// ── Objects compare (item 8, 2026-07-09) ────────────────────────────────
// The compare feature is a real <form method="get" action="/objects/
// compare"> with checkboxes named "ids" (bim-planroom.css / catalog.rs) —
// it already works with JavaScript disabled via the plain "Compare
// selected" submit button. This just adds the live count + "appears once
// 2+ selected" polish: disabling the button below 2 checked, and a Clear
// button that unchecks everything.
function syncCompareBar() {
  const bar = document.getElementById('bim-compare-bar');
  if (!bar) return;
  const n = document.querySelectorAll('.bim-compare-check:checked').length;
  const go = document.getElementById('bim-compare-go');
  const labelEl = document.getElementById('bim-compare-label');
  if (labelEl) {
    labelEl.innerHTML = n >= 2
      ? 'Compare <b>' + n + '</b> selected Objects, side by side.'
      : (n === 1
        ? 'Check 1 more Object to compare dimensions.'
        : 'Check 2 or more Objects to compare their dimensions.');
  }
  bar.classList.toggle('is-ready', n >= 2);
  if (go) go.disabled = n < 2;
}

document.addEventListener('change', (e) => {
  if (e.target.closest('.bim-compare-check')) syncCompareBar();
});

document.addEventListener('click', (e) => {
  if (!e.target.closest('#bim-compare-clear')) return;
  document.querySelectorAll('.bim-compare-check:checked').forEach((cb) => {
    cb.checked = false;
  });
  syncCompareBar();
});

syncCompareBar();

// Expose globally for inline editor scripts
window.SchemaState = SchemaState;
window.BimNavigate = navigate;
