// Lookahead peak limiter for the plugin output: the gain each sample needs to stay under the ceiling, held
// over the lookahead, released smoothly and averaged across the lookahead so it is already down when a peak
// arrives. The audio runs one lookahead late; the host is told the latency.
pub struct Limiter {
    n: usize,
    pos: usize,
    dl: Vec<f32>,
    dr: Vec<f32>,
    need: Vec<f32>,
    env: f32,
    avg: Vec<f32>,
    sum: f32,
    rel: f32,
    ceil: f32,
}

impl Limiter {
    pub fn new(sr: f64) -> Self {
        let n = ((sr * 0.0015).round() as usize).max(8);
        Self { n, pos: 0, dl: vec![0.0; n], dr: vec![0.0; n], need: vec![1.0; n], env: 1.0, avg: vec![1.0; n], sum: n as f32, rel: (-1.0 / (0.08 * sr)).exp() as f32, ceil: 10f32.powf(-0.3 / 20.0) }
    }

    /// Lookahead in samples.
    pub fn latency(&self) -> u32 { self.n as u32 }

    pub fn reset(&mut self) {
        self.dl.fill(0.0); self.dr.fill(0.0); self.need.fill(1.0); self.avg.fill(1.0);
        self.env = 1.0; self.sum = self.n as f32;
    }

    /// Limit in place; `on` false passes the audio through with the same delay.
    pub fn process(&mut self, l: &mut [f32], r: &mut [f32], on: bool) {
        for i in 0..l.len() {
            let (xl, xr) = (l[i], r[i]);
            let pk = xl.abs().max(xr.abs());
            let k = self.pos;
            self.need[k] = if pk > self.ceil { self.ceil / pk } else { 1.0 };
            let held = self.need.iter().cloned().fold(1.0f32, f32::min);
            self.env = if held < self.env { held } else { held + (self.env - held) * self.rel };
            self.sum += self.env - self.avg[k];
            self.avg[k] = self.env;
            let g = if on { (self.sum / self.n as f32).min(1.0) } else { 1.0 };
            let (ol, or) = (self.dl[k], self.dr[k]);
            self.dl[k] = xl;
            self.dr[k] = xr;
            self.pos = (k + 1) % self.n;
            l[i] = if on { (ol * g).clamp(-self.ceil, self.ceil) } else { ol };
            r[i] = if on { (or * g).clamp(-self.ceil, self.ceil) } else { or };
        }
    }
}
