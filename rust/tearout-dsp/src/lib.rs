//! Tearout gun engine: deterministic offline renderer of one-shot (or burst) gun hits.
//! `Engine::render(&Params)` returns stereo f32 at 48 kHz.

pub mod dsp;
pub mod spec;
mod layers;
mod chain;
mod engine;

pub use engine::{Engine, Out};
pub use spec::{id, Scale, Spec, N, SPEC};

pub const SR: f64 = 48000.0;

/// One value per entry of `SPEC`, indexed by the constants in `id`.
#[derive(Clone, Debug)]
pub struct Params(pub [f64; N]);

impl Default for Params {
    fn default() -> Self {
        let mut v = [0f64; N];
        for (i, s) in SPEC.iter().enumerate() { v[i] = s.def; }
        Params(v)
    }
}
impl std::ops::Index<usize> for Params {
    type Output = f64;
    fn index(&self, i: usize) -> &f64 { &self.0[i] }
}
impl std::ops::IndexMut<usize> for Params {
    fn index_mut(&mut self, i: usize) -> &mut f64 { &mut self.0[i] }
}
impl Params {
    pub fn index_of(name: &str) -> Option<usize> { SPEC.iter().position(|s| s.id == name) }
    /// Sets a param by its id; unknown ids are ignored and reported as false.
    pub fn set(&mut self, name: &str, v: f64) -> bool {
        match Self::index_of(name) { Some(i) => { self.0[i] = v; true } None => false }
    }
    pub fn get(&self, name: &str) -> Option<f64> { Self::index_of(name).map(|i| self.0[i]) }
}

/// [0,1] <-> value, per the param's scale
pub fn to_norm(s: &Spec, v: f64) -> f64 {
    match s.scale {
        Scale::Log => (v / s.min).ln() / (s.max / s.min).ln(),
        _ => (v - s.min) / (s.max - s.min),
    }
}
pub fn from_norm(s: &Spec, u: f64) -> f64 {
    let u = u.max(0.0).min(1.0);
    match s.scale {
        Scale::Log => s.min * (s.max / s.min).powf(u),
        Scale::Int => dsp::js_round(s.min + u * (s.max - s.min)),
        Scale::Lin => s.min + u * (s.max - s.min),
    }
}
