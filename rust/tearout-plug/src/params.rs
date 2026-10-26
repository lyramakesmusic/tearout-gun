// Host-facing parameters. Every engine knob is a host parameter (normalized 0..1, shown in its own units),
// plus the play-side controls.
use nih_plug::prelude::*;
use nih_plug_egui::EguiState;
use std::sync::Arc;
use tearout_dsp::{from_norm, to_norm, Scale, Spec, N, SPEC};

pub struct GunKnobs {
    pub p: Vec<FloatParam>,
}

pub fn group_name(g: &str) -> &'static str {
    match g {
        "global" => "global",
        "sub" => "sub",
        "tr" => "transient",
        "body" => "body",
        "synth" => "synth",
        "spice" => "spice",
        "rev" => "reverb",
        "crunch" => "crunch",
        "master" => "master",
        "burst" => "burst",
        g if g.starts_with("sample") => "samples",
        _ => "other",
    }
}

/// A stepped knob whose steps have names ("saw/square/..."), as opposed to a rate unit like "/s".
pub fn is_named(s: &Spec) -> bool { s.scale == Scale::Int && s.unit.contains('/') }

pub fn fmt_value(s: &Spec, v: f64) -> String {
    if is_named(s) {
        let opts: Vec<&str> = s.unit.split('/').collect();
        let i = (v.round() as i64 - s.min as i64).clamp(0, opts.len() as i64 - 1) as usize;
        return opts[i].to_string();
    }
    match s.unit {
        "Hz" if v >= 1000.0 => format!("{:.1} kHz ({})", v / 1000.0, hz_note(v)),
        "Hz" if v >= 20.0 => format!("{:.0} Hz ({})", v, hz_note(v)),
        "Hz" => format!("{:.1} Hz", v),
        "ms" if v >= 100.0 => format!("{:.0} ms", v),
        "ms" => format!("{:.1} ms", v),
        "dB" => format!("{:+.1}", v).replace('-', "−"),
        "st" => format!("{:.0} st", v),
        "midi" => note_name(v),
        _ if s.scale == Scale::Int => format!("{:.0}", v),
        _ => format!("{:.2}", v),
    }
}

/// The note nearest a frequency, as "F4".
pub fn hz_note(f: f64) -> String { note_name(69.0 + 12.0 * (f / 440.0).log2()) }

/// A note typed as "F#4", "Bb2" or "c-1", as a MIDI note.
pub fn parse_note(t: &str) -> Option<f64> {
    let t = t.trim().to_lowercase();
    let mut c = t.chars();
    let base = match c.next()? { 'c' => 0, 'd' => 2, 'e' => 4, 'f' => 5, 'g' => 7, 'a' => 9, 'b' => 11, _ => return None };
    let rest: String = c.collect();
    let (shift, oct) = if let Some(o) = rest.strip_prefix('#') { (1, o) } else if let Some(o) = rest.strip_prefix('b').filter(|o| !o.is_empty()) { (-1, o) } else { (0, rest.as_str()) };
    let oct: i64 = oct.trim().parse().ok().filter(|o: &i64| (-2..=8).contains(o))?;
    Some(((oct + 2) * 12 + base + shift) as f64)
}

/// A MIDI note's name the way Ableton, Logic, Cubase and Bitwig write it: 60 is C3.
pub fn note_name(m: f64) -> String {
    const NAMES: [&str; 12] = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
    let m = m.round() as i64;
    format!("{}{}", NAMES[m.rem_euclid(12) as usize], m.div_euclid(12) - 2)
}

impl Default for GunKnobs {
    fn default() -> Self {
        let pre = &crate::presets::all()[crate::presets::default_index()].p;
        let p = SPEC
            .iter()
            .enumerate()
            .map(|(i, s)| {
                let s2 = *s;
                let mut fp = FloatParam::new(format!("{} {}", group_name(s.group), s.label), to_norm(s, pre.0[i]) as f32, FloatRange::Linear { min: 0.0, max: 1.0 })
                    .with_value_to_string(Arc::new(move |u| fmt_value(&s2, from_norm(&s2, u as f64))));
                if s.scale == Scale::Int && s.max > s.min { fp = fp.with_step_size(1.0 / (s.max - s.min) as f32); }
                fp
            })
            .collect();
        Self { p }
    }
}

unsafe impl Params for GunKnobs {
    fn param_map(&self) -> Vec<(String, ParamPtr, String)> {
        SPEC.iter().zip(&self.p).map(|(s, fp)| (s.id.to_string(), fp.as_ptr(), group_name(s.group).to_string())).collect()
    }
}

impl GunKnobs {
    /// Current patch in engine units.
    pub fn snapshot(&self) -> tearout_dsp::Params {
        let mut out = tearout_dsp::Params::default();
        for i in 0..N { out.0[i] = from_norm(&SPEC[i], self.p[i].value() as f64); }
        out
    }
}

#[derive(Params)]
pub struct PlugParams {
    #[persist = "editor-state"]
    pub editor_state: Arc<EguiState>,
    #[persist = "preset"]
    pub preset: Arc<std::sync::RwLock<String>>,
    /// Audio files layered in sample slots 1–3 (empty = no sample).
    #[persist = "samples"]
    pub samples: Arc<std::sync::RwLock<[String; 3]>>,
    /// Their names in the editor.
    #[persist = "sample-names"]
    pub sample_names: Arc<std::sync::RwLock<[String; 3]>>,

    #[nested(group = "gun")]
    pub gun: GunKnobs,

    /// How far a new note pulls earlier notes down (0 = rings, 1 = cut).
    #[id = "duck"]
    pub duck: FloatParam,
    #[id = "duck_rel"]
    pub duck_rel: FloatParam,
    #[id = "vel"]
    pub vel: FloatParam,
    /// Dice: while on, every note re-rolls the patch inside the dice ranges.
    #[id = "dice"]
    pub dice: BoolParam,
    #[id = "dice_amt"]
    pub dice_amt: FloatParam,
    /// Output gain, ahead of the limiter.
    #[id = "out"]
    pub out: FloatParam,
    /// Output peak limiter, holding the plugin's output under -0.3 dBFS.
    #[id = "limit"]
    pub limit: BoolParam,
    /// Shots spread evenly across this many host beats (0 = timed by the interval knob).
    #[id = "shots_grid"]
    pub shots_grid: FloatParam,
}

/// Grid lengths for burst shots, in beats.
pub const GRID: [(f64, &str); 7] = [(0.0, "off"), (0.125, "1/32"), (0.25, "1/16"), (0.5, "1/8"), (1.0, "1/4"), (2.0, "1/2"), (4.0, "1 bar")];

impl Default for PlugParams {
    fn default() -> Self {
        Self {
            editor_state: EguiState::from_size((crate::editor::W * crate::editor::DEFAULT_SCALE).round() as u32, (crate::editor::H * crate::editor::DEFAULT_SCALE).round() as u32),
            preset: Arc::new(std::sync::RwLock::new(String::new())),
            samples: Arc::new(std::sync::RwLock::new(Default::default())),
            sample_names: Arc::new(std::sync::RwLock::new(Default::default())),
            gun: GunKnobs::default(),
            duck: FloatParam::new("duck", 0.7, FloatRange::Linear { min: 0.0, max: 1.0 }).with_value_to_string(Arc::new(|v| format!("{:.0}%", v * 100.0))),
            duck_rel: FloatParam::new("duck release", 120.0, FloatRange::Skewed { min: 5.0, max: 2000.0, factor: 0.3 }).with_unit(" ms").with_value_to_string(Arc::new(|v| format!("{:.0}", v))),
            vel: FloatParam::new("velocity", 0.5, FloatRange::Linear { min: 0.0, max: 1.0 }).with_value_to_string(Arc::new(|v| format!("{:.0}%", v * 100.0))),
            dice: BoolParam::new("dice", false),
            out: FloatParam::new("output", 0.0, FloatRange::Linear { min: -24.0, max: 6.0 }).with_unit(" dB").with_value_to_string(Arc::new(|v| format!("{:+.1}", v).replace('-', "−"))),
            limit: BoolParam::new("limiter", true),
            shots_grid: FloatParam::new("shots grid", 0.0, FloatRange::Linear { min: 0.0, max: (GRID.len() - 1) as f32 }).with_step_size(1.0).with_value_to_string(Arc::new(|v| GRID[(v.round() as usize).min(GRID.len() - 1)].1.to_string())),
            dice_amt: FloatParam::new("dice amount", 0.35, FloatRange::Linear { min: 0.0, max: 1.0 }).with_value_to_string(Arc::new(|v| format!("{:.0}%", v * 100.0))),
        }
    }
}
