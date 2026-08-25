// Cross-origin isolation service worker.
// Adds COOP/COEP headers to every same-origin response so that
// SharedArrayBuffer (and transferring it to workers) works.

self.addEventListener('install',  () => self.skipWaiting());
self.addEventListener('activate', e => e.waitUntil(self.clients.claim()));

self.addEventListener('fetch', e => {
  e.respondWith(fetch(e.request).then(r => {
    if (!r.url.startsWith(self.location.origin)) return r;
    const h = new Headers(r.headers);
    h.set('Cross-Origin-Opener-Policy',  'same-origin');
    h.set('Cross-Origin-Embedder-Policy','require-corp');
    return new Response(r.body, { status: r.status, statusText: r.statusText, headers: h });
  }));
});