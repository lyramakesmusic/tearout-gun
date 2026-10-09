// Parallel loss evaluation for the fitter: a pool of helper workers sharing the target. Startup is async; once ready,
// batches are driven synchronously from inside a worker (Atomics.wait), so the staged fit runs unchanged.
// A batch that stalls is finished locally with the fallback loss. Requires a cross-origin-isolated page.
export async function makePool(x, P0, size, url, fallback) {
  if (typeof SharedArrayBuffer === 'undefined' || !self.crossOriginIsolated || size < 2) return null;
  const MAX = 256, fBuf = new SharedArrayBuffer(MAX * 8), ctl = new SharedArrayBuffer(8), F = new Float64Array(fBuf), C = new Int32Array(ctl);
  const ws = Array.from({ length: size }, () => new Worker(url, { type: 'module' }));
  const ok = await Promise.race([
    Promise.all(ws.map((w) => new Promise((res) => { w.onmessage = (e) => e.data.ready && res(); w.postMessage({ init: { x, P0, fBuf, ctl } }); }))).then(() => true),
    new Promise((res) => setTimeout(() => res(false), 15000)),
  ]);
  if (!ok) { ws.forEach((w) => w.terminate()); return null; }
  let dead = false;
  const batch = (Ps) => {
    if (dead) return Ps.map(fallback);
    F.fill(NaN, 0, Ps.length); Atomics.store(C, 0, 0);
    for (let k = 0; k < Ps.length; k++) ws[k % size].postMessage({ jobs: [[k, Ps[k]]] });
    let c; while ((c = Atomics.load(C, 0)) < Ps.length) if (Atomics.wait(C, 0, c, 20000) === 'timed-out' && Atomics.load(C, 0) === c) { dead = true; break; }
    return Array.from({ length: Ps.length }, (_, k) => (Number.isNaN(F[k]) ? fallback(Ps[k]) : F[k]));
  };
  return { batch, close: () => ws.forEach((w) => w.terminate()) };
}
