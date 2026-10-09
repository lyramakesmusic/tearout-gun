// Fit helper: builds the same objective as the fitter, then scores candidate patches handed over in batches.
// Results go into shared memory: f[k] for each candidate, then the done counter is bumped and the fitter woken.
import { fitLoss } from '../dsp/fitstages.js';
let loss = null, F = null, done = null;
self.onmessage = (e) => {
  const d = e.data;
  if (d.init) { loss = fitLoss(d.init.x, d.init.P0).loss; F = new Float64Array(d.init.fBuf); done = new Int32Array(d.init.ctl); self.postMessage({ ready: true }); return; }
  if (d.jobs) { for (const [k, P] of d.jobs) F[k] = loss(P); Atomics.add(done, 0, d.jobs.length); Atomics.notify(done, 0); }
};
