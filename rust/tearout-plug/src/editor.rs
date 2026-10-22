// Editor: stems (per-layer sharpened spectrograms with their waveforms and handles),
// knobs (every parameter), play (note behaviour and dice). The core strip sits under the stems.
use crate::analysis::{freq_y, y_freq};
use crate::params::{fmt_value, note_name, PlugParams};
use crate::worker::{Shared, View, DICE, VIEW_MS};

use crossbeam_channel::Sender;
use nih_plug::prelude::*;
use nih_plug_egui::egui::{self, pos2, vec2, Color32, CornerRadius, FontFamily, FontId, Pos2, Rect, Sense, Shape, Stroke, StrokeKind, TextureHandle};
use nih_plug_egui::{create_egui_editor, egui::Context};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tearout_dsp::id::*;
use tearout_dsp::{from_norm, to_norm, SPEC};

pub const W: f32 = 1040.0;
pub const H: f32 = 600.0;
pub static AUDITION: AtomicBool = AtomicBool::new(false);

const BG: Color32 = Color32::from_rgb(0x0f, 0x0f, 0x10);
const TEXT: Color32 = Color32::from_rgb(0xd4, 0xd4, 0xd4);
const DIM: Color32 = Color32::from_rgb(0x8a, 0x8a, 0x8a);
const DIMMER: Color32 = Color32::from_rgb(0x6a, 0x6a, 0x6c);
const BRIGHT: Color32 = Color32::from_rgb(0xe6, 0xe6, 0xe6);
const TRACK: Color32 = Color32::from_rgb(0x2c, 0x2c, 0x2f);
const SUB_C: Color32 = Color32::from_rgb(0x56, 0xb4, 0xe9);
const TR_C: Color32 = Color32::from_rgb(0xe6, 0x9f, 0x00);
const BODY_C: Color32 = Color32::from_rgb(0x3f, 0xc7, 0x9a);
const SYN_C: Color32 = Color32::from_rgb(0xcc, 0x79, 0xa7);
const SPICE_C: Color32 = Color32::from_rgb(0xf0, 0xe4, 0x42);
const REV_C: Color32 = Color32::from_rgb(0xe2, 0x4a, 0x54);
const MST_C: Color32 = Color32::from_rgb(0xc8, 0xc8, 0xc8);

#[derive(PartialEq, Clone, Copy)]
enum Tab { Stems, Knobs }

struct Ui {
    tab: Tab,
    preset: usize,
    /// the preset menu: factory presets, then saved ones
    list: Vec<crate::presets::Entry>,
    /// name being typed for a preset to save
    naming: Option<String>,
    view: Option<Arc<View>>,
    tex: Vec<TextureHandle>,
    title: Option<TextureHandle>,
    /// spectrograms of the zoomed-in window, and which zoom they came from
    ztex: Vec<TextureHandle>,
    zoom: Option<Arc<crate::worker::Zoom>>,
    /// signature, GitHub and X marks
    credit: Option<[TextureHandle; 3]>,
    seen: u64,
    sel: usize,
    /// level a switched-off section returns to
    mem: [f32; 8],
    presets_open: bool,
    anchor: Option<tearout_dsp::Params>,
    last_mutation: Option<tearout_dsp::Params>,
    /// computer keyboard playing (standalone): on, octave of the A key, velocity
    keys_on: bool,
    keys_oct: i32,
    keys_vel: f32,
    /// notes seen at the last frame, and when the latest arrived (egui time)
    notes_seen: u64,
    note_t: f64,
    /// when the octave last moved (egui time)
    keys_moved: f64,
    /// live output bands (dB re the patch's loudest band) and when they were last updated
    live: [f64; crate::analysis::TOB_BANDS],
    live_t: f64,
    /// where the header's left controls end
    header_x: f32,
    /// a sample lane being renamed: its slot and the name so far
    renaming: Option<(usize, String)>,
    /// the time window the lanes show (start, span), ms
    t0: f64,
    span: f64,
    /// how far the stems are scrolled
    lane_scroll: f32,
    /// lanes draw their spectrogram and their waveform
    show_spec: bool,
    show_wave: bool,
    /// handles and curves over the lanes
    show_handles: bool,
    /// value being typed into: gun knob index and the text so far
    editing: Option<(usize, String)>,
    /// sample slot whose lane gets selected once it appears
    want_sel: Option<usize>,
    rng: tearout_dsp::dsp::Rng,
}

pub fn create(params: Arc<PlugParams>, shared: Arc<Shared>, _notes: Sender<u8>) -> Option<Box<dyn Editor>> {
    let list = crate::presets::list();
    let saved = params.preset.read().map(|n| n.clone()).unwrap_or_default();
    let preset = list.iter().position(|e| e.key() == saved).or_else(|| crate::presets::index_of(&saved)).unwrap_or_else(crate::presets::default_index);
    let st = Ui { tab: Tab::Stems, preset, list, naming: None, view: None, tex: Vec::new(), title: None, ztex: Vec::new(), zoom: None, credit: None, seen: 0, sel: 3, mem: [0.8; 8], presets_open: false, anchor: None, last_mutation: None, want_sel: None, editing: None, keys_on: crate::STANDALONE.load(Ordering::Relaxed), keys_oct: 2, keys_vel: 0.8, keys_moved: -10.0, notes_seen: 0, note_t: -10.0, live: [-60.0; crate::analysis::TOB_BANDS], live_t: 0.0, header_x: 400.0, renaming: None, t0: 0.0, span: VIEW_MS, lane_scroll: 0.0, show_spec: true, show_wave: true, show_handles: true, rng: tearout_dsp::dsp::Rng::new((std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.subsec_nanos()).unwrap_or(7) % 9973) as f64 + 1.0) };
    let p2 = params.clone();
    params.editor_state.set_host_resizable(fit_size);
    create_egui_editor(params.editor_state.clone(), st, |ctx, _| setup(ctx), move |ctx, setter, st| frame(ctx, setter, st, &p2, &shared))
}

fn setup(ctx: &Context) {
    let mut f = egui::FontDefinitions::default();
    f.font_data.insert("plex".into(), Arc::new(egui::FontData::from_static(include_bytes!("../assets/IBMPlexSans-Regular.ttf"))));
    f.font_data.insert("plex-semi".into(), Arc::new(egui::FontData::from_static(include_bytes!("../assets/IBMPlexSans-SemiBold.ttf"))));
    f.font_data.insert("mono".into(), Arc::new(egui::FontData::from_static(include_bytes!("../assets/IBMPlexMono-Regular.ttf"))));
    f.families.get_mut(&FontFamily::Proportional).unwrap().insert(0, "plex".into());
    f.families.get_mut(&FontFamily::Monospace).unwrap().insert(0, "mono".into());
    f.families.insert(FontFamily::Name("semi".into()), vec!["plex-semi".into(), "plex".into()]);
    ctx.set_fonts(f);
    let mut v = egui::Visuals::dark();
    v.panel_fill = BG;
    ctx.set_visuals(v);
    ctx.options_mut(|o| o.zoom_with_keyboard = false);
    #[cfg(target_os = "macos")]
    crate::macos::watch_pinch();
}

fn sans(s: f32) -> FontId { FontId::new(s, FontFamily::Proportional) }
fn semi(s: f32) -> FontId { FontId::new(s, FontFamily::Name("semi".into())) }
fn mono(s: f32) -> FontId { FontId::new(s, FontFamily::Monospace) }
fn rgb(c: [u8; 3]) -> Color32 { Color32::from_rgb(c[0], c[1], c[2]) }
fn alpha(c: Color32, a: f32) -> Color32 { Color32::from_rgba_unmultiplied(c.r(), c.g(), c.b(), (a * 255.0) as u8) }

struct Cx<'a> {
    ui: &'a mut egui::Ui,
    setter: &'a ParamSetter<'a>,
    p: &'a PlugParams,
    sh: &'a Shared,
}

impl<'a> Cx<'a> {
    fn painter(&self) -> egui::Painter { self.ui.painter().clone() }
    fn gun(&self, i: usize) -> f64 { from_norm(&SPEC[i], self.p.gun.p[i].unmodulated_normalized_value() as f64) }
    fn set_norm(&self, i: usize, u: f32) {
        let fp = &self.p.gun.p[i];
        self.setter.begin_set_parameter(fp);
        self.setter.set_parameter_normalized(fp, u.clamp(0.0, 1.0));
        self.setter.end_set_parameter(fp);
    }
    fn set_val(&self, i: usize, v: f64) { self.set_norm(i, to_norm(&SPEC[i], v) as f32) }
    fn text(&self, pos: Pos2, align: egui::Align2, s: &str, f: FontId, c: Color32) -> Rect { self.ui.painter().text(pos, align, s, f, c) }
    fn text_w(&self, s: &str, f: FontId) -> f32 { self.ui.painter().layout_no_wrap(s.to_string(), f, TEXT).size().x }
    /// A labelled button; returns clicked.
    fn button(&mut self, r: Rect, id: &str, label: &str, fill: Color32, fg: Color32, font: FontId) -> bool {
        let resp = self.ui.interact(r, egui::Id::new(id), Sense::click());
        if fill != Color32::TRANSPARENT { self.ui.painter().rect_filled(r, CornerRadius::same(3), fill); }
        let fg = if resp.hovered() && fill == Color32::TRANSPARENT { BRIGHT } else { fg };
        self.text(r.center(), egui::Align2::CENTER_CENTER, label, font, fg);
        resp.clicked()
    }
}

fn frame(ctx: &Context, setter: &ParamSetter, st: &mut Ui, params: &Arc<PlugParams>, sh: &Arc<Shared>) {
    let v = sh.view_version.load(Ordering::Acquire);
    if v != st.seen {
        if let Some(view) = sh.view.lock().unwrap().clone() {
            st.tex = view.lanes.iter().map(|l| ctx.load_texture(l.name, egui::ColorImage::from_rgba_unmultiplied([view.cols, crate::worker::SPEC_ROWS], &l.rgba), egui::TextureOptions::LINEAR)).collect();
            if let Some(k) = st.want_sel { if let Some(i) = view.lanes.iter().position(|l| crate::worker::sample_slot(l.name) == Some(k)) { st.sel = i; st.want_sel = None; } }
            st.view = Some(view);
            st.seen = v;
        }
    }
    ctx.request_repaint_after(std::time::Duration::from_millis(33));
    // the whole layout scales with the window as it actually is, keeping its aspect
    let zoom = ctx.zoom_factor();
    let win = ctx.input(|i| i.screen_rect().size()) * zoom;
    let scale = (win.x / W).min(win.y / H).clamp(MIN_SCALE, MAX_SCALE);
    if (zoom - scale).abs() > 1e-3 { ctx.set_zoom_factor(scale); }
    egui::CentralPanel::default().frame(egui::Frame::NONE.fill(BG)).show(ctx, |ui| {
        let mut cx = Cx { ui, setter, p: params, sh };
        header(&mut cx, st);
        match st.tab {
            Tab::Stems => { stems(&mut cx, st); core_strip(&mut cx, st); }
            Tab::Knobs => knobs_tab(&mut cx, st),
        }
        if st.presets_open { preset_menu(&mut cx, st); }
        if crate::STANDALONE.load(Ordering::Relaxed) { computer_keys(&mut cx, st); }
        drop_zone(&mut cx, st);
        resize_corner(&mut cx, scale);
    });
}

const AUDIO_EXT: [&str; 5] = ["wav", "aif", "aiff", "flac", "mp3"];

/// An audio file from outside the plugin (its own dragged-out stems don't layer back in).
fn is_audio(p: &std::path::Path) -> bool {
    p.extension().and_then(|e| e.to_str()).is_some_and(|e| AUDIO_EXT.contains(&e.to_ascii_lowercase().as_str())) && !p.starts_with(crate::export::root())
}

/// Audio files dragged over the editor layer into a sample slot: the one under the pointer, else the first
/// empty one, else the selected sample lane's.
fn drop_zone(cx: &mut Cx, st: &mut Ui) {
    let (hover, dropped) = cx.ui.input(|i| (i.raw.hovered_files.iter().filter_map(|f| f.path.clone()).any(|p| is_audio(&p)), i.raw.dropped_files.iter().filter_map(|f| f.path.clone()).filter(|p| is_audio(p)).collect::<Vec<_>>()));
    if hover {
        let zone = Rect::from_min_max(pos2(14.0, LANES_TOP - 4.0), pos2(W - 14.0, STRIP_TOP - 8.0));
        let pt = cx.ui.ctx().layer_painter(egui::LayerId::new(egui::Order::Foreground, egui::Id::new("drop")));
        pt.rect_filled(zone, CornerRadius::same(4), Color32::from_rgba_unmultiplied(0x10, 0x10, 0x11, 200));
        pt.rect_stroke(zone, CornerRadius::same(4), Stroke::new(1.5, DIM), StrokeKind::Inside);
        pt.text(zone.center(), egui::Align2::CENTER_CENTER, "layer", semi(13.0), BRIGHT);
    }
    if dropped.is_empty() { return; }
    let paths = cx.p.samples.read().map(|s| s.clone()).unwrap_or_default();
    let sel = st.view.as_ref().and_then(|v| v.lanes.get(st.sel)).and_then(|ln| crate::worker::sample_slot(ln.name));
    let mut used: Vec<usize> = Vec::new();
    for path in dropped {
        let slot = (1..=3).find(|&k| paths[k - 1].is_empty() && !used.contains(&k)).or(sel).unwrap_or(3);
        used.push(slot);
        if let Ok(mut s) = cx.p.samples.write() { s[slot - 1] = path.to_string_lossy().into_owned(); }
        if let Ok(mut n) = cx.p.sample_names.write() { n[slot - 1] = "sample".into(); }
        let lvl = S1_LVL + 10 * (slot - 1);
        if cx.gun(lvl) <= -59.0 { cx.set_val(lvl, -6.0); }
        st.want_sel = Some(slot);
    }
}

/// Scale a new editor opens at.
pub const DEFAULT_SCALE: f32 = 1.2;
const MIN_SCALE: f32 = 0.75;
const MAX_SCALE: f32 = 2.0;

/// Host window sizes snap to the editor's aspect, between 75% and 200%.
fn fit_size((w, h): (u32, u32)) -> (u32, u32) {
    let s = ((w as f32 * h as f32) / (W * H)).sqrt().clamp(MIN_SCALE, MAX_SCALE);
    ((W * s).round() as u32, (H * s).round() as u32)
}

/// Bottom-right grip: scales with the pointer's travel on screen from where the drag began; a
/// double-click returns to the default size.
fn resize_corner(cx: &mut Cx, scale: f32) {
    let r = Rect::from_min_max(pos2(W - 16.0, H - 16.0), pos2(W, H));
    let resp = cx.ui.interact(r, egui::Id::new("resize"), Sense::click_and_drag()).on_hover_cursor(egui::CursorIcon::ResizeNwSe);
    let set = |s: f32| cx.p.editor_state.set_requested_size(((W * s).round() as u32, (H * s).round() as u32));
    let start_id = egui::Id::new("resize-start");
    if resp.drag_started() {
        let at = crate::grab::screen_cursor().map(|c| (c.0 as f32, c.1 as f32)).or_else(|| resp.interact_pointer_pos().map(|p| (p.x * scale, p.y * scale)));
        if let Some(at) = at { cx.ui.memory_mut(|m| m.data.insert_temp(start_id, (at, scale))); }
    }
    if resp.double_clicked() { set(DEFAULT_SCALE); }
    else if resp.dragged() {
        let start = cx.ui.memory(|m| m.data.get_temp::<((f32, f32), f32)>(start_id));
        let now = crate::grab::screen_cursor().map(|c| (c.0 as f32, c.1 as f32)).or_else(|| resp.interact_pointer_pos().map(|p| (p.x * scale, p.y * scale)));
        if let (Some(((x0, y0), s0)), Some((x, y))) = (start, now) {
            let s = (s0 + ((x - x0) / W).max((y - y0) / H)).clamp(MIN_SCALE, MAX_SCALE);
            if (s - scale).abs() > 0.004 { set(s); }
        }
    }
    let col = if resp.hovered() || resp.dragged() { DIM } else { DIMMER };
    let pt = cx.painter();
    for k in [4.0, 8.0, 12.0] { pt.line_segment([pos2(W - 3.0 - k, H - 3.0), pos2(W - 3.0, H - 3.0 - k)], Stroke::new(1.0, col)); }
}

// ───────────────────────── header

fn header(cx: &mut Cx, st: &mut Ui) {
    let pt = cx.painter();
    let y = 20.0;
    // wordmark
    const TITLE: &[u8] = include_bytes!("../assets/title/title.rgba");
    const TITLE_SIZE: [usize; 2] = [1176, 120];
    let tex = st.title.get_or_insert_with(|| cx.ui.ctx().load_texture("title", egui::ColorImage::from_rgba_unmultiplied(TITLE_SIZE, TITLE), egui::TextureOptions::LINEAR)).id();
    let th = 13.0;
    let tw = th * TITLE_SIZE[0] as f32 / TITLE_SIZE[1] as f32;
    pt.image(tex, Rect::from_min_size(pos2(16.0, y - th / 2.0), vec2(tw, th)), Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)), Color32::from_rgba_unmultiplied(200, 200, 200, 190));
    let mut x = 16.0 + tw + 20.0;
    let n = st.list.len().max(1);
    if cx.button(Rect::from_min_size(pos2(x - 4.0, 8.0), vec2(16.0, 24.0)), "prev", "‹", Color32::TRANSPARENT, DIM, sans(16.0)) { load_preset(cx, st, (st.preset + n - 1) % n); }
    x += 16.0;
    let name = st.list.get(st.preset).map(|p| p.name.clone()).unwrap_or_default();
    let nw = cx.text_w(&name, semi(13.0)).max(70.0);
    let nr = Rect::from_min_size(pos2(x - 4.0, 8.0), vec2(nw + 8.0, 24.0));
    if let Some(buf) = st.naming.as_mut() {
        // naming a preset to save: enter saves, escape leaves it
        let er = Rect::from_min_size(pos2(x - 4.0, 8.0), vec2(nw.max(120.0) + 8.0, 24.0));
        cx.ui.painter().rect_filled(er, CornerRadius::same(3), TRACK);
        let resp = cx.ui.put(er.shrink2(vec2(4.0, 3.0)), egui::TextEdit::singleline(buf).font(semi(13.0)).frame(false).text_color(BRIGHT).desired_width(er.width() - 8.0));
        if !resp.has_focus() && !resp.lost_focus() { resp.request_focus(); }
        if resp.lost_focus() {
            let cancel = cx.ui.input(|i| i.key_pressed(egui::Key::Escape));
            let name = buf.trim().to_string();
            st.naming = None;
            if !cancel && !name.is_empty() { save_preset(cx, st, &name); }
        }
    } else {
        let nresp = cx.ui.interact(nr, egui::Id::new("preset-name"), Sense::click());
        if nresp.clicked() { st.presets_open = !st.presets_open; if st.presets_open { st.list = crate::presets::list(); } }
        cx.text(pos2(x, y), egui::Align2::LEFT_CENTER, &name, semi(13.0), if nresp.hovered() || st.presets_open { BRIGHT } else { TEXT });
        // edited since it was loaded
        if st.list.get(st.preset).is_some_and(|e| !same(&e.p, &cx.p.gun.snapshot())) {
            cx.text(pos2(x + cx.text_w(&name, semi(13.0)) + 4.0, y - 1.0), egui::Align2::LEFT_CENTER, "•", semi(13.0), DIM);
        }
    }
    x += nw.max(if st.naming.is_some() { 120.0 } else { 0.0 }) + 6.0;
    if cx.button(Rect::from_min_size(pos2(x - 4.0, 8.0), vec2(16.0, 24.0)), "next", "›", Color32::TRANSPARENT, DIM, sans(16.0)) { load_preset(cx, st, (st.preset + 1) % n); }
    x += 18.0;
    // save: a floppy disk
    let sr = Rect::from_center_size(pos2(x + 6.0, y), vec2(20.0, 22.0));
    let sresp = cx.ui.interact(sr, egui::Id::new("save"), Sense::click());
    let sc = if sresp.hovered() || st.naming.is_some() { BRIGHT } else { DIM };
    let c = sr.center();
    let stroke = Stroke::new(1.3, sc);
    let (l, t, r, b) = (c.x - 6.0, c.y - 6.0, c.x + 6.0, c.y + 6.0);
    pt.add(Shape::closed_line(vec![pos2(l, t), pos2(r - 3.0, t), pos2(r, t + 3.0), pos2(r, b), pos2(l, b)], stroke));
    pt.rect_stroke(Rect::from_min_max(pos2(l + 3.0, t), pos2(r - 4.0, t + 4.0)), CornerRadius::ZERO, stroke, StrokeKind::Middle);
    pt.rect_stroke(Rect::from_min_max(pos2(l + 2.5, c.y + 1.0), pos2(r - 2.5, b)), CornerRadius::ZERO, stroke, StrokeKind::Middle);
    if sresp.clicked() { st.naming = Some(if st.list.get(st.preset).is_some_and(|e| e.user) { name.clone() } else { String::new() }); }
    x += 22.0;
    // tabs
    let tabs = [("stems", Tab::Stems), ("knobs", Tab::Knobs)];
    let widths: Vec<f32> = tabs.iter().map(|t| cx.text_w(t.0, sans(12.0)) + 20.0).collect();
    let tw: f32 = widths.iter().sum::<f32>() + 2.0 * (tabs.len() as f32 + 1.0);
    pt.rect_filled(Rect::from_min_size(pos2(x, 8.0), vec2(tw, 24.0)), CornerRadius::same(4), Color32::from_rgb(0x16, 0x16, 0x17));
    let mut tx = x + 2.0;
    for (i, (label, t)) in tabs.iter().enumerate() {
        let r = Rect::from_min_size(pos2(tx, 10.0), vec2(widths[i], 20.0));
        let on = st.tab == *t;
        if cx.button(r, label, label, if on { Color32::from_rgb(0x26, 0x26, 0x29) } else { Color32::TRANSPARENT }, if on { TEXT } else { DIMMER }, sans(12.0)) { st.tab = *t; }
        tx += widths[i] + 2.0;
    }
    x += tw + 10.0;
    // view toggles: spectrogram, waveform, handles
    for k in 0..3 {
        let r = Rect::from_min_size(pos2(x, 8.0), vec2(28.0, 24.0));
        let resp = cx.ui.interact(r, egui::Id::new(("view", k)), Sense::click());
        let on = [st.show_spec, st.show_wave, st.show_handles][k];
        if resp.clicked() { match k { 0 => st.show_spec = !on, 1 => st.show_wave = !on, _ => st.show_handles = !on } }
        let fill = if on { Color32::from_rgb(0x22, 0x22, 0x25) } else if resp.hovered() { Color32::from_rgb(0x1c, 0x1c, 0x1e) } else { Color32::TRANSPARENT };
        pt.rect_filled(r, CornerRadius::same(4), fill);
        let ic = if on { DIM } else { Color32::from_rgb(0x48, 0x48, 0x4c) };
        let c = r.center();
        if k == 0 {
            // spectrogram: bars of falling height
            for (j, hh) in [10.0f32, 7.0, 8.5, 5.0, 3.5].iter().enumerate() {
                let bx = c.x - 8.0 + j as f32 * 4.0;
                pt.rect_filled(Rect::from_min_max(pos2(bx, c.y + 5.0 - hh), pos2(bx + 2.5, c.y + 5.0)), CornerRadius::ZERO, ic);
            }
        } else if k == 1 {
            let pts: Vec<Pos2> = (0..=24).map(|j| { let t = j as f32 / 24.0; pos2(c.x - 9.0 + 18.0 * t, c.y - 5.0 * (t * std::f32::consts::TAU * 1.5).sin() * (1.0 - 0.6 * t)) }).collect();
            pt.add(Shape::line(pts, Stroke::new(1.5, ic)));
        } else {
            // a falling curve with a handle on it
            let pts: Vec<Pos2> = (0..=16).map(|j| { let t = j as f32 / 16.0; pos2(c.x - 9.0 + 18.0 * t, c.y - 6.0 + 12.0 * (1.0 - (-3.0 * t).exp())) }).collect();
            pt.add(Shape::line(pts, Stroke::new(1.3, ic)));
            pt.circle_filled(pos2(c.x - 1.0, c.y + 3.2), 2.6, ic);
        }
        x += 30.0;
    }
    st.header_x = x;
    // right side: dice, note, audition
    let mut rx = W - 20.0;
    let pr = Rect::from_center_size(pos2(rx - 14.0, y), vec2(28.0, 28.0));
    let resp = cx.ui.interact(pr, egui::Id::new("audition"), Sense::click());
    pt.circle_filled(pr.center(), 13.0, if resp.hovered() { Color32::from_rgb(0x3a, 0x3a, 0x3e) } else { Color32::from_rgb(0x26, 0x26, 0x29) });
    let c = pr.center();
    pt.add(Shape::convex_polygon(vec![pos2(c.x - 3.5, c.y - 5.5), pos2(c.x + 6.5, c.y), pos2(c.x - 3.5, c.y + 5.5)], if resp.hovered() { BRIGHT } else { TEXT }, Stroke::NONE));
    if resp.clicked() { AUDITION.store(true, Ordering::Relaxed); }
    rx -= 28.0 + 12.0;
    if crate::STANDALONE.load(Ordering::Relaxed) {
        // computer keyboard: lit while the keys play notes (M toggles)
        let kr = Rect::from_center_size(pos2(rx - 14.0, y), vec2(28.0, 24.0));
        let resp = cx.ui.interact(kr, egui::Id::new("keys"), Sense::click());
        if resp.clicked() { st.keys_on = !st.keys_on; }
        let c = if st.keys_on { DIM } else if resp.hovered() { DIMMER } else { Color32::from_rgb(0x40, 0x40, 0x44) };
        // seven white keys with the five black keys over them
        let body = Rect::from_center_size(kr.center(), vec2(21.0, 13.0));
        pt.rect_stroke(body, CornerRadius::same(1), Stroke::new(1.2, c), StrokeKind::Middle);
        let kw = body.width() / 7.0;
        for k in 1..7 { let x = body.left() + k as f32 * kw; pt.line_segment([pos2(x, body.top()), pos2(x, body.bottom())], Stroke::new(1.0, c)); }
        for k in [1, 2, 4, 5, 6] {
            let x = body.left() + k as f32 * kw;
            pt.rect_filled(Rect::from_min_max(pos2(x - kw * 0.32, body.top()), pos2(x + kw * 0.32, body.top() + body.height() * 0.6)), CornerRadius::ZERO, c);
        }
        // the range the keys play, shown for a moment after the octave moves
        let since = cx.ui.input(|i| i.time) - st.keys_moved;
        if st.keys_on && since < 1.5 {
            let a = (1.0 - (since as f32 - 1.0).max(0.0) * 2.0).clamp(0.0, 1.0);
            cx.text(pos2(kr.center().x, kr.bottom() + 6.0), egui::Align2::CENTER_CENTER, &format!("C{}–E{}", st.keys_oct - 1, st.keys_oct), mono(9.0), alpha(TEXT, a));
        }
        rx -= 28.0 + 10.0;
    }
    // lights as notes arrive
    let now = cx.ui.input(|i| i.time);
    let seen = cx.sh.notes_seen.load(Ordering::Relaxed);
    if seen != st.notes_seen { st.notes_seen = seen; st.note_t = now; }
    let glow = (1.0 - ((now - st.note_t) / 0.35) as f32).clamp(0.0, 1.0);
    pt.circle_filled(pos2(rx - 4.0, y), 3.0, Color32::from_rgb(0x2e, 0x2e, 0x31).lerp_to_gamma(Color32::from_rgb(0x7c, 0xd9, 0x7a), glow));
    if glow > 0.0 { cx.ui.ctx().request_repaint(); }
    rx -= 16.0;
    let note = note_name(cx.gun(NOTE));
    let nw = cx.text_w(&note, mono(12.0));
    cx.text(pos2(rx, y), egui::Align2::RIGHT_CENTER, &note, mono(12.0), DIM);
    rx -= nw + 12.0;
    let dice_on = cx.p.dice.value();
    let amt = cx.p.dice_amt.value();
    let label = format!("{:.0}%", amt * 100.0);
    let dw = cx.text_w(&label, mono(12.0)) + 34.0;
    let dr = Rect::from_min_size(pos2(rx - dw, 8.0), vec2(dw, 24.0));
    let resp = cx.ui.interact(dr, egui::Id::new("dice"), Sense::click_and_drag());
    let (fill, fg) = if dice_on { (Color32::from_rgb(0x3a, 0x3a, 0x3e), BRIGHT) } else { (Color32::from_rgb(0x16, 0x16, 0x17), DIMMER) };
    pt.rect_filled(dr, CornerRadius::same(4), fill);
    let ic = pos2(dr.left() + 14.0, y);
    for (dx, dy) in [(-4.0, -4.0), (4.0, -4.0), (0.0, 0.0), (-4.0, 4.0), (4.0, 4.0)] { pt.circle_filled(pos2(ic.x + dx, ic.y + dy), 1.6, fg); }
    cx.text(pos2(dr.left() + 26.0, y), egui::Align2::LEFT_CENTER, &label, mono(12.0), fg);
    if resp.clicked() {
        cx.setter.begin_set_parameter(&cx.p.dice);
        cx.setter.set_parameter(&cx.p.dice, !dice_on);
        cx.setter.end_set_parameter(&cx.p.dice);
    }
    if resp.drag_started() { cx.setter.begin_set_parameter(&cx.p.dice_amt); }
    if resp.dragged() { cx.setter.set_parameter(&cx.p.dice_amt, (amt + resp.drag_delta().x / 200.0).clamp(0.0, 1.0)); }
    if resp.drag_stopped() { cx.setter.end_set_parameter(&cx.p.dice_amt); }
    if resp.hovered() { cx.ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeHorizontal); }
    // vol: the output gain as a small knob left of the dice, named beside it; its value shows while touched
    let kc = pos2(dr.left() - 22.0, y);
    let resp = knob(cx, (4, 0), &cx.p.out, kc, 8.0, BRIGHT, None);
    let hot = resp.hovered() || resp.dragged();
    cx.text(pos2(kc.x - 14.0, y), egui::Align2::RIGHT_CENTER, "vol", sans(11.0), if hot { TEXT } else { DIMMER });
    if hot { cx.text(pos2(kc.x, kc.y + 17.0), egui::Align2::CENTER_CENTER, &format!("{} dB", format!("{:+.1}", cx.p.out.value()).replace('-', "−")), mono(9.0), TEXT); }
    // oscilloscope of the output between the view toggles and vol: the trace starts on a rising zero
    // crossing, so a steady tone stands still
    let sr_ = Rect::from_min_max(pos2(st.header_x + 16.0, 9.0), pos2(kc.x - 46.0, 31.0));
    if sr_.width() > 40.0 {
        let buf = cx.sh.scope.lock().map(|sc| { let (b, p, _) = &*sc; let mut v = b[*p..].to_vec(); v.extend_from_slice(&b[..*p]); v }).unwrap_or_default();
        let span = 1600usize;
        if buf.len() > 2 * span {
            let last = buf.len() - span - 1;
            let trig = (span..last).rev().find(|&i| buf[i - 1] < 0.0 && buf[i] >= 0.0).unwrap_or(last);
            let a = sr_.height() * 0.5;
            let c = sr_.center().y;
            let pts: Vec<Pos2> = (0..span).step_by(2).map(|k| pos2(sr_.left() + k as f32 / span as f32 * sr_.width(), c - buf[trig + k].clamp(-1.0, 1.0) * a)).collect();
            pt.add(Shape::line(pts, Stroke::new(1.0, Color32::from_rgb(0x5a, 0x5a, 0x5e))));
        }
    }
}

fn load_preset(cx: &Cx, st: &mut Ui, i: usize) {
    let Some(pre) = st.list.get(i).cloned() else { return };
    st.preset = i;
    for k in 0..tearout_dsp::N { cx.set_val(k, pre.p.0[k]); }
    if pre.user { if let Ok(mut s) = cx.p.samples.write() { *s = pre.samples.clone(); } }
    if let Ok(mut n) = cx.p.preset.write() { *n = pre.key(); }
}

/// Ask for a new Tearout Farm folder.
fn pick_folder() {
    #[cfg(any(target_os = "macos", windows))]
    if let Some(p) = rfd::FileDialog::new().set_title("Tearout Farm folder").set_directory(crate::export::root()).pick_folder() { crate::export::set_root(p); }
    #[cfg(not(any(target_os = "macos", windows)))]
    crate::export::reveal(&crate::export::root());
}

fn save_preset(cx: &Cx, st: &mut Ui, name: &str) {
    let samples = cx.p.samples.read().map(|s| s.clone()).unwrap_or_default();
    if let Err(e) = crate::presets::save(name, &cx.p.gun.snapshot(), &samples) { nih_log!("save failed: {e}"); return; }
    st.list = crate::presets::list();
    if let Some(i) = st.list.iter().position(|e| e.user && e.name == name) {
        st.preset = i;
        if let Ok(mut n) = cx.p.preset.write() { *n = st.list[i].key(); }
    }
}

// ───────────────────────── stems

const LANE_X: f32 = 126.0;
const LANES_TOP: f32 = 50.0;
const STRIP_TOP: f32 = 506.0;

fn stems(cx: &mut Cx, st: &mut Ui) {
    let Some(view) = st.view.clone() else {
        cx.text(pos2(W / 2.0, 280.0), egui::Align2::CENTER_CENTER, "rendering", sans(12.0), DIMMER);
        return;
    };
    time_window(cx, st, &view);
    // the mix stays at the top; the stems below it keep their height and scroll
    let (mix_h, gap) = (130.0, 6.0);
    lane(cx, st, &view, 0, LANES_TOP, mix_h);
    let rule = LANES_TOP + mix_h + 8.0;
    cx.painter().line_segment([pos2(10.0, rule), pos2(W - 20.0, rule)], Stroke::new(1.0, Color32::from_rgb(0x22, 0x22, 0x25)));
    let area = Rect::from_min_max(pos2(0.0, rule + 8.0), pos2(W, STRIP_TOP - 12.0));
    let n = view.lanes.len().saturating_sub(1);
    // stems share the space while each keeps a usable height; past that they scroll, sized so the next peeks in
    let fit = (area.height() + gap) / n.max(1) as f32 - gap;
    let stem_h = if fit >= 72.0 { 72.0 } else if fit >= 58.0 { fit } else { (area.height() + gap) / 4.4 - gap };
    let total = n as f32 * (stem_h + gap) - gap;
    let room = (total - area.height()).max(0.0);
    let over = cx.ui.input(|i| i.pointer.hover_pos()).is_some_and(|p| area.contains(p));
    let plain = cx.ui.input(|i| !(i.modifiers.command || i.modifiers.ctrl || i.modifiers.shift));
    if over && plain { st.lane_scroll -= wheel(cx).1; }
    st.lane_scroll = st.lane_scroll.clamp(0.0, room);
    let clip = cx.ui.clip_rect();
    cx.ui.set_clip_rect(area.intersect(clip));
    for li in 1..view.lanes.len() {
        let y = area.top() + (li - 1) as f32 * (stem_h + gap) - st.lane_scroll;
        if y + stem_h < area.top() || y > area.bottom() { continue; }
        lane(cx, st, &view, li, y, stem_h);
    }
    cx.ui.set_clip_rect(clip);
    if room > 0.0 {
        // where the view sits among the stems
        let th = area.height() * area.height() / total;
        let ty = area.top() + (area.height() - th) * st.lane_scroll / room;
        cx.painter().rect_filled(Rect::from_min_size(pos2(W - 15.0, ty), vec2(3.0, th)), CornerRadius::same(2), Color32::from_rgb(0x2e, 0x2e, 0x31));
    }
}

fn lane(cx: &mut Cx, st: &mut Ui, view: &View, li: usize, y: f32, h: f32) {
    let pt = cx.painter();
    let ln = &view.lanes[li];
    let col = rgb(ln.color);
    // mute and solo: what plays, leaving the patch alone
    if let Some(bit) = crate::worker::lane_bit(ln.name) {
        for (k, (tag, flags)) in [("M", &cx.sh.mute), ("S", &cx.sh.solo)].into_iter().enumerate() {
            let on = flags.load(Ordering::Relaxed) & (1 << bit) != 0;
            let r = Rect::from_center_size(pos2(25.0, y + h / 2.0 + if k == 0 { -17.0 } else { 0.0 }), vec2(16.0, 15.0));
            let resp = cx.ui.interact(r, egui::Id::new(("ms", li, k)), Sense::click());
            let (fill, fg) = match (on, k) { (true, 0) => (Color32::from_rgb(0x8a, 0x8a, 0x8e), BG), (true, _) => (col, BG), _ => (if resp.hovered() { Color32::from_rgb(0x38, 0x38, 0x3c) } else { TRACK }, DIM) };
            pt.rect_filled(r, CornerRadius::same(3), fill);
            pt.text(r.center(), egui::Align2::CENTER_CENTER, tag, semi(9.0), fg);
            if resp.clicked() { flags.fetch_xor(1 << bit, Ordering::Relaxed); }
        }
    }
    // label
    let lr = Rect::from_min_size(pos2(40.0, y), vec2(78.0, h));
    if cx.ui.interact(lr, egui::Id::new(("lane", li)), Sense::click()).clicked() { st.sel = li; }
    if let Some(slot) = crate::worker::sample_slot(ln.name) {
        // a sample lane shows its name; click it to rename in place
        let named = cx.p.sample_names.read().map(|n| n[slot - 1].clone()).unwrap_or_default();
        let shown = if named.is_empty() { ln.name.to_string() } else { named };
        let nr = Rect::from_min_size(pos2(46.0, y + h / 2.0 - 15.0), vec2(LANE_X - 72.0, 16.0));
        if let Some((k, buf)) = st.renaming.as_mut().filter(|e| e.0 == slot) {
            let _ = k;
            cx.ui.painter().rect_filled(nr, CornerRadius::same(3), TRACK);
            let resp = cx.ui.put(nr.shrink2(vec2(3.0, 1.0)), egui::TextEdit::singleline(buf).font(semi(12.0)).frame(false).text_color(BRIGHT).desired_width(nr.width() - 6.0));
            if !resp.has_focus() && !resp.lost_focus() { resp.request_focus(); }
            if resp.lost_focus() {
                let cancel = cx.ui.input(|i| i.key_pressed(egui::Key::Escape));
                if !cancel { let v = buf.trim().to_string(); if let Ok(mut n) = cx.p.sample_names.write() { n[slot - 1] = v; } }
                st.renaming = None;
            }
        } else {
            let resp = cx.ui.interact(nr, egui::Id::new(("rename", slot)), Sense::click()).on_hover_cursor(egui::CursorIcon::Text);
            let galley = cx.ui.painter().layout_no_wrap(shown.clone(), semi(12.0), col);
            cx.ui.painter().with_clip_rect(nr).galley(pos2(nr.left() + 4.0, nr.center().y - galley.size().y / 2.0), galley, col);
            if resp.clicked() { st.sel = li; st.renaming = Some((slot, shown)); }
        }
    } else if li != 0 { cx.text(pos2(50.0, y + h / 2.0 - 7.0), egui::Align2::LEFT_CENTER, ln.name, semi(12.0), col); }
    // a stem's level reads live from its knob and drags up and down; double-click resets it
    let lvl_id = crate::worker::LAYERS.iter().find(|l| l.0 == ln.name).map(|l| l.1).or((ln.name == "reverb").then_some(REV_LVL));
    let lvl = match lvl_id { Some(i) => format!("{:.1}", cx.gun(i)), None => format!("{:.1} pk", view.peak_db) }.replace('-', "−");
    let mut lc = DIMMER;
    if let Some(i) = lvl_id {
        let vr = Rect::from_min_size(pos2(46.0, y + h / 2.0 + 1.0), vec2(48.0, 16.0));
        let resp = cx.ui.interact(vr, egui::Id::new(("lane-lvl", li)), Sense::click_and_drag());
        let fp = &cx.p.gun.p[i];
        vertical_drag(cx, &resp, fp);
        if resp.double_clicked() { cx.setter.begin_set_parameter(fp); cx.setter.set_parameter_normalized(fp, fp.default_normalized_value()); cx.setter.end_set_parameter(fp); }
        if resp.hovered() || resp.dragged() { cx.ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeVertical); lc = TEXT; }
    }
    if li == 0 {
        // the whole gun: its tonal balance against the gun reference, with the live output underneath;
        // below, every stem at once, the output level (drag, double-click resets) and the peak
        let chart = Rect::from_min_max(pos2(10.0, y + 2.0), pos2(118.0, y + h - 22.0));
        tonal_chart(cx, st, view, chart);
        let row = y + h - 10.0;
        let ar = Rect::from_center_size(pos2(18.0, row), vec2(16.0, 18.0));
        let aresp = cx.ui.interact(ar, egui::Id::new("all-stems"), Sense::drag());
        let ac = if aresp.hovered() || aresp.dragged() { TEXT } else { Color32::from_rgb(0x55, 0x55, 0x5a) };
        for k in 0..3 {
            let o = k as f32 * 2.5;
            let page = Rect::from_min_size(pos2(ar.left() + 2.0 + o, ar.top() + 2.0 + o), vec2(8.0, 10.0));
            pt.rect_filled(page, CornerRadius::same(1), BG);
            pt.rect_stroke(page, CornerRadius::same(1), Stroke::new(1.0, ac), StrokeKind::Middle);
        }
        if aresp.hovered() { cx.ui.ctx().set_cursor_icon(egui::CursorIcon::Grab); }
        if aresp.drag_started() { drag_out(st, view, &(1..view.lanes.len()).collect::<Vec<_>>()); }
        cx.text(pos2(118.0, row), egui::Align2::RIGHT_CENTER, &format!("{:.1} pk", view.peak_db).replace('-', "−"), mono(9.0), Color32::from_rgb(0x4c, 0x4c, 0x50));
    } else {
        cx.text(pos2(50.0, y + h / 2.0 + 9.0), egui::Align2::LEFT_CENTER, &lvl, mono(10.0), lc);
    }
    if let Some(slot) = crate::worker::sample_slot(ln.name) {
        let xr = Rect::from_center_size(pos2(LANE_X - 14.0, y + h / 2.0 - 7.0), vec2(16.0, 16.0));
        let resp = cx.ui.interact(xr, egui::Id::new(("unload", li)), Sense::click());
        let c = if resp.hovered() { BRIGHT } else { Color32::from_rgb(0x55, 0x55, 0x5a) };
        let k = 3.0;
        pt.line_segment([xr.center() - vec2(k, k), xr.center() + vec2(k, k)], Stroke::new(1.3, c));
        pt.line_segment([xr.center() + vec2(-k, k), xr.center() + vec2(k, -k)], Stroke::new(1.3, c));
        if resp.clicked() {
            if let Ok(mut s) = cx.p.samples.write() { s[slot - 1].clear(); }
            cx.set_val(S1_LVL + 10 * (slot - 1), -60.0);
        }
    }
    if let Some(g) = lane_group(ln.name) {
        let lr = Rect::from_center_size(pos2(25.0, y + h / 2.0 + 18.0), vec2(16.0, 16.0));
        let resp = cx.ui.interact(lr, egui::Id::new(("lock", li)), Sense::click());
        let on = cx.sh.locked.lock().unwrap().contains(&g);
        if resp.clicked() { let mut l = cx.sh.locked.lock().unwrap(); if on { l.retain(|&x| x != g); } else { l.push(g); } }
        lock_icon(&pt, lr.center(), on, if on { col } else if resp.hovered() { BRIGHT } else { Color32::from_rgb(0x55, 0x55, 0x5a) });
    }
    // spectrogram; dragging it takes the lane out as a file (option-drag on the mix takes every stem)
    let r = Rect::from_min_size(pos2(LANE_X, y), vec2(W - 20.0 - LANE_X, h));
    let sresp = cx.ui.interact(r, egui::Id::new(("lane-drag", li)), Sense::click_and_drag());
    // a click plays the lane: the mix as a note, a stem on its own
    if sresp.clicked() {
        if li == 0 { AUDITION.store(true, Ordering::Relaxed); }
        else { let _ = cx.sh.preview.0.try_send(Arc::new(crate::worker::Sound { l: (*ln.l).clone(), r: (*ln.r).clone(), note: view.p.0[NOTE] })); }
    }
    if sresp.hovered() && !cx.ui.ctx().is_using_pointer() {
        cx.ui.ctx().set_cursor_icon(egui::CursorIcon::Grab);
        pt.rect_filled(r, CornerRadius::ZERO, Color32::from_white_alpha(10));
    }
    if sresp.drag_started() {
        let all = li == 0 && cx.ui.input(|i| i.modifiers.alt);
        let pick: Vec<usize> = if all { (1..view.lanes.len()).collect() } else { vec![li] };
        drag_out(st, view, &pick);
    }
    let (u0, u1) = ((view_t0() / view.len_ms) as f32, ((view_t0() + view_span()) / view.len_ms) as f32);
    let sharp = st.zoom.as_ref().filter(|z| z.view == view.version && (z.t0 - view_t0()).abs() < 1e-6 && (z.span - view_span()).abs() < 1e-6).and_then(|_| st.ztex.get(li));
    if let Some(t) = sharp.filter(|_| st.show_spec) {
        pt.image(t.id(), r, Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)), Color32::WHITE);
    } else if let Some(t) = st.tex.get(li).filter(|_| st.show_spec) {
        // the part of the sound in the view; past its end the lane stays dark
        let rr = Rect::from_min_max(r.min, pos2(r.left() + r.width() * ((1.0 - u0) / (u1 - u0)).min(1.0), r.bottom()));
        pt.image(t.id(), rr, Rect::from_min_max(pos2(u0, 0.0), pos2(u1.min(1.0), 1.0)), Color32::WHITE);
    }
    if !st.show_wave {
        if !st.show_spec {
            // with both views off the lane is a placeholder: a dotted centre line and the layer's name
            let cy = r.center().y;
            pt.extend(Shape::dashed_line(&[pos2(r.left(), cy), pos2(r.right(), cy)], Stroke::new(1.0, Color32::from_rgb(0x24, 0x24, 0x27)), 2.0, 5.0));
            let label = if li == 0 { "mix" } else { ln.name };
            let g = pt.layout_no_wrap(label.to_string(), sans(13.0), Color32::from_rgb(0x3c, 0x3c, 0x40));
            let br = Rect::from_center_size(r.center(), g.size() + vec2(16.0, 4.0));
            pt.rect_filled(br, CornerRadius::ZERO, BG);
            pt.galley(br.center() - g.size() / 2.0, g, Color32::from_rgb(0x3c, 0x3c, 0x40));
        }
        if st.show_handles { lane_overlays(cx, ln.name, r, col, sounding(ln)); }
        return;
    }
    // waveform
    // peaks per pixel column; zoomed in far enough they come straight from the audio, down to the samples
    let sr_ms = tearout_dsp::SR / 1000.0;
    let (s0, s1) = ((view_t0() * sr_ms) as usize, ((view_end() * sr_ms) as usize).min(ln.l.len()));
    let px = r.width().max(1.0) as usize;
    let per = (s1.saturating_sub(s0)) as f32 / px as f32;
    let fill = Color32::from_rgba_unmultiplied(255, 255, 255, 5);
    let (cy, a) = (r.center().y, r.height() * 0.46);
    let mut top = Vec::with_capacity(px);
    let mut bot = Vec::with_capacity(px);
    if per <= 1.5 && s1 > s0 {
        // a sample or less per pixel: the waveform itself
        let pts: Vec<Pos2> = (s0..s1).map(|i| pos2(r.left() + (i - s0) as f32 / per.max(1e-3), cy - 0.5 * (ln.l[i] + ln.r[i]) * a)).collect();
        pt.add(Shape::line(pts, Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 40))));
    } else if per <= 64.0 && s1 > s0 {
        for k in 0..px {
            let (i0, i1) = (s0 + (k as f32 * per) as usize, (s0 + ((k + 1) as f32 * per) as usize).min(s1).max(s0 + (k as f32 * per) as usize + 1));
            let (mut hi, mut lo) = (f32::MIN, f32::MAX);
            for i in i0..i1.min(ln.l.len()) { let v = 0.5 * (ln.l[i] + ln.r[i]); hi = hi.max(v); lo = lo.min(v); }
            if hi < lo { continue; }
            let x = r.left() + k as f32 + 0.5;
            let (y0, y1) = (cy - hi * a, cy - lo * a);
            pt.rect_filled(Rect::from_min_max(pos2(x - 0.5, y0), pos2(x + 0.5, y1.max(y0 + 0.5))), CornerRadius::ZERO, fill);
            top.push(pos2(x, y0));
            bot.push(pos2(x, y1));
        }
    } else {
        let nall = ln.wave.len();
        let (c0, c1) = (((u0 * nall as f32) as usize).min(nall), ((u1 * nall as f32).ceil() as usize).min(nall));
        let colw = r.width() / ((u1 - u0) * nall as f32);
        for (c, &(hi, lo)) in ln.wave[c0..c1].iter().enumerate() {
            let x = r.left() + ((c0 + c) as f32 + 0.5 - u0 * nall as f32) * colw;
            let (y0, y1) = (cy - hi * a, cy - lo * a);
            pt.rect_filled(Rect::from_min_max(pos2(x - 0.5 * colw, y0), pos2(x + 0.5 * colw, y1.max(y0 + 0.5))), CornerRadius::ZERO, fill);
            top.push(pos2(x, y0));
            bot.push(pos2(x, y1));
        }
    }
    let edge = Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 14));
    pt.add(Shape::line(top, edge));
    pt.add(Shape::line(bot, edge));
    if st.show_handles { lane_overlays(cx, ln.name, r, col, sounding(ln)); }
}

/// The knob group a stem lane's lock holds.
fn lane_group(name: &str) -> Option<&'static str> {
    Some(match name { "sub" => "sub", "transient" => "tr", "body" => "body", "synth" => "synth", "spice" => "spice", "reverb" => "rev", _ => return None })
}

fn lock_icon(pt: &egui::Painter, c: Pos2, on: bool, col: Color32) {
    let body = Rect::from_center_size(pos2(c.x, c.y + 2.0), vec2(8.0, 6.0));
    pt.rect_filled(body, CornerRadius::same(1), col);
    // shackle: closed over the body, or lifted and swung open
    let (lx, lift) = if on { (0.0, 0.0) } else { (2.5, 2.0) };
    let top = c.y - 5.0 - lift;
    let pts: Vec<Pos2> = (0..=12).map(|i| { let a = std::f32::consts::PI * i as f32 / 12.0; pos2(c.x + lx - 2.5 * a.cos(), top + 2.5 - 2.5 * a.sin()) }).collect();
    let mut sh = vec![pos2(c.x + lx - 2.5, body.top())];
    sh.extend(pts);
    sh.push(pos2(c.x + lx + 2.5, if on { body.top() } else { top + 4.0 }));
    pt.add(Shape::line(sh, Stroke::new(1.3, col)));
}

/// Tonal balance: the gun reference (grey) and this patch (orange), dB re their loudest band from +3 to
/// −48, with the live output's bands filled underneath at the same scale.
fn tonal_chart(cx: &mut Cx, st: &mut Ui, view: &View, r: Rect) {
    use crate::analysis::{gun_target, third_oct, TOB_BANDS};
    let pt = cx.painter();
    let xs = |i: usize| r.left() + i as f32 / (TOB_BANDS - 1) as f32 * r.width();
    let ys = |db: f64| r.top() + ((3.0 - db.clamp(-48.0, 3.0)) / 51.0) as f32 * r.height();
    for db in [-12.0, -24.0, -36.0] { pt.line_segment([pos2(r.left(), ys(db)), pos2(r.right(), ys(db))], Stroke::new(1.0, Color32::from_rgb(0x1e, 0x1e, 0x21))); }
    // live: the output's last 8192 samples against the patch's loudest band, falling back at 40 dB/s
    let (buf, sr) = cx.sh.scope.lock().map(|sc| { let (b, p, sr) = &*sc; let mut v = b[*p..].to_vec(); v.extend_from_slice(&b[..*p]); (v, *sr) }).unwrap_or_default();
    let now = cx.ui.input(|i| i.time);
    let dt = (now - st.live_t).clamp(0.0, 0.2);
    st.live_t = now;
    if !buf.is_empty() {
        let bands = third_oct(&buf, sr);
        for k in 0..TOB_BANDS { st.live[k] = (bands[k] - view.tob_max).max(st.live[k] - 40.0 * dt); }
    }
    let base = r.bottom();
    let mut poly = vec![pos2(r.left(), base)];
    poly.extend((0..TOB_BANDS).map(|k| pos2(xs(k), ys(st.live[k]))));
    poly.push(pos2(r.right(), base));
    for k in 0..TOB_BANDS - 1 {
        let q = [pos2(xs(k), base), pos2(xs(k), ys(st.live[k])), pos2(xs(k + 1), ys(st.live[k + 1])), pos2(xs(k + 1), base)];
        pt.add(Shape::convex_polygon(q.to_vec(), Color32::from_white_alpha(14), Stroke::NONE));
    }
    let line = |vals: &[f64], col: Color32, w: f32| pt.add(Shape::line(vals.iter().enumerate().map(|(k, &v)| pos2(xs(k), ys(v))).collect(), Stroke::new(w, col)));
    line(gun_target(), Color32::from_rgb(0x77, 0x77, 0x7a), 1.2);
    line(&view.tob, Color32::from_rgb(0xf2, 0x8a, 0x12), 1.6);
}

/// A layer's amplitude envelope (dB scale, 0 dB at the top, −48 at the bottom) with a dot where its gate
/// closes. Dragging the dot left or right scales the envelope's times; up and down bends an exponential decay
/// (fuller or faster) with the end held.
fn env_curve(cx: &mut Cx, id: &str, r: Rect, knobs: [usize; 5]) {
    let [att, dec, hold, rel, shape] = knobs;
    let exp = cx.gun(shape).round() == 0.0;
    let n = view_end().ceil() as usize + 1;
    let e = tearout_dsp::dsp::envelope(n, 1000.0, cx.gun(att), cx.gun(dec), cx.gun(hold), cx.gun(rel), cx.gun(shape).round() as i64);
    let yd = |db: f64| r.top() + 4.0 + ((-db.clamp(-48.0, 0.0)) / 48.0) as f32 * (r.height() - 8.0);
    let end_ms = if exp { cx.gun(hold) + cx.gun(rel) } else { cx.gun(hold) };
    let last = (end_ms.ceil() as usize).min(n - 1);
    let first = view_t0().floor().max(0.0) as usize;
    let pts: Vec<Pos2> = (first..=last.max(first)).map(|i| pos2(tx(r, i as f64), yd(20.0 * (e[i] as f64).max(1e-6).log10()))).collect();
    cx.painter().add(Shape::line(pts, Stroke::new(1.0, alpha(BRIGHT, 0.3))));
    if end_ms >= view_end() || end_ms < view_t0() { return; }
    if handle(cx, &format!("{id}_len"), pos2(tx(r, end_ms), yd(-48.0)), BRIGHT, true, (true, exp)).is_some() {
        let d = cx.ui.input(|i| i.pointer.delta());
        if d.x != 0.0 {
            let want = (end_ms + (d.x / r.width()) as f64 * view_span()).max(cx.gun(att) + 2.0);
            let k = want / end_ms.max(1.0);
            let set = |cx: &mut Cx, i: usize, v: f64| cx.set_val(i, v.clamp(SPEC[i].min, SPEC[i].max));
            if exp { for i in [hold, rel, dec] { let v = cx.gun(i) * k; set(cx, i, v); } } else { set(cx, hold, want); }
        }
        if exp && d.y != 0.0 {
            let v = cx.gun(dec) * (-d.y as f64 * 0.015).exp();
            cx.set_val(dec, v.clamp(SPEC[dec].min, SPEC[dec].max));
        }
    }
}

/// When a lane first and last sounds (within 30 dB of its peak), in ms.
fn sounding(ln: &crate::worker::Lane) -> Option<(f64, f64)> {
    let pk = ln.l.iter().fold(0f32, |a, &v| a.max(v.abs()));
    if pk <= 0.0 { return None; }
    let ms = |i: usize| i as f64 * 1000.0 / tearout_dsp::SR;
    Some((ms(ln.l.iter().position(|&v| v.abs() > pk * 0.03)?), ms(ln.l.iter().rposition(|&v| v.abs() > pk * 0.03)?)))
}

thread_local! {
    /// the time window every lane shows: start and span, ms
    static WINDOW: std::cell::Cell<(f64, f64)> = const { std::cell::Cell::new((0.0, VIEW_MS)) };
}
fn view_t0() -> f64 { WINDOW.with(|w| w.get().0) }
fn view_span() -> f64 { WINDOW.with(|w| w.get().1) }
fn view_end() -> f64 { view_t0() + view_span() }

fn tx(r: Rect, ms: f64) -> f32 { r.left() + ((ms - view_t0()) / view_span()) as f32 * r.width() }
/// The time under `x`, kept inside the lane so a handle dragged past its edge stays reachable.
fn tms(r: Rect, x: f32) -> f64 { view_t0() + ((x.clamp(r.left(), r.right() - 4.0) - r.left()) / r.width()) as f64 * view_span() }
fn fy(r: Rect, f: f64) -> f32 { r.top() + freq_y(f) * r.height() }
fn yf(r: Rect, y: f32) -> f64 { y_freq((y - r.top()) / r.height()) }

/// A draggable handle. `axes`: which coordinates the drag edits. Returns the new position while dragging.

fn handle(cx: &mut Cx, id: &str, at: Pos2, col: Color32, round: bool, axes: (bool, bool)) -> Option<Pos2> {
    let pt = cx.painter();
    let hr = Rect::from_center_size(at, vec2(16.0, 16.0));
    let resp = cx.ui.interact(hr, egui::Id::new(id), Sense::drag());

    let s = if resp.hovered() || resp.dragged() { 12.0 } else { 10.0 };
    let rr = Rect::from_center_size(at, vec2(s, s));
    if round { pt.circle(at, s / 2.0, col, Stroke::new(2.0, BG)); } else { pt.rect(rr, CornerRadius::same(1), col, Stroke::new(2.0, BG), StrokeKind::Outside); }
    if resp.hovered() || resp.dragged() {
        cx.ui.ctx().set_cursor_icon(match axes { (true, false) => egui::CursorIcon::ResizeHorizontal, (false, true) => egui::CursorIcon::ResizeVertical, _ => egui::CursorIcon::Move });
    }
    if resp.dragged() { resp.interact_pointer_pos() } else { None }
}

/// Hold marker: thin line over the lane with a square tab on top, dragged in time.
/// A time marker with a tab that drags it (the reverb's size).
fn hold_marker(cx: &mut Cx, id: &str, r: Rect, param: usize, label: &str) {
    let ms = cx.gun(param);
    let pt = cx.painter();
    if ms > view_end() {
        let s = format!("{} {:.0} →", label, ms);
        cx.text(pos2(r.right() - 6.0, r.top() + 10.0), egui::Align2::RIGHT_CENTER, &s, mono(10.0), BRIGHT);
        return;
    }
    let x = tx(r, ms);
    pt.line_segment([pos2(x, r.top()), pos2(x, r.bottom())], Stroke::new(1.0, alpha(BRIGHT, 0.45)));
    let tab = Rect::from_min_size(pos2(x - 4.0, r.top()), vec2(8.0, 8.0));
    let resp = cx.ui.interact(tab.expand(4.0), egui::Id::new(id), Sense::drag());
    pt.rect_filled(tab, CornerRadius::ZERO, BRIGHT);
    if resp.hovered() || resp.dragged() { cx.ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeHorizontal); }
    if resp.dragged() { if let Some(p) = resp.interact_pointer_pos() { cx.set_val(param, tms(r, p.x).clamp(SPEC[param].min, SPEC[param].max)); } }
}

fn curve(cx: &Cx, r: Rect, col: Color32, f: impl Fn(f64) -> f64) {
    let pts: Vec<Pos2> = (0..=240).map(|i| { let ms = view_end() * (i as f64 / 240.0).powf(2.0); pos2(tx(r, ms), fy(r, f(ms / 1000.0))) }).filter(|p| p.x >= r.left() - 1.0).collect();
    cx.painter().add(Shape::line(pts, Stroke::new(2.0, col)));
}

fn dashed(cx: &Cx, a: Pos2, b: Pos2, col: Color32) {
    cx.painter().extend(Shape::dashed_line(&[a, b], Stroke::new(1.0, col), 3.0, 4.0));
}

/// A lane's handles and curves, kept inside the lane: anything outside the time in view stays hidden.
fn lane_overlays(cx: &mut Cx, name: &str, r: Rect, col: Color32, spans_ms: Option<(f64, f64)>) {
    let clip = cx.ui.clip_rect();
    cx.ui.set_clip_rect(r.expand2(vec2(7.0, 6.0)).intersect(clip));
    overlays(cx, name, r, col, spans_ms);
    cx.ui.set_clip_rect(clip);
}

fn overlays(cx: &mut Cx, name: &str, r: Rect, col: Color32, spans_ms: Option<(f64, f64)>) {
    let pt = cx.painter();
    let clip = pt.clip_rect();
    let _ = clip;
    match name {
        "sub" => {
            let fe = tearout_dsp::dsp::midi_hz(cx.gun(NOTE) + cx.gun(SUB_TUNE));
            let (d, tau, k) = (cx.gun(SUB_DIVE) / 12.0, cx.gun(SUB_TAU) / 1000.0, cx.gun(SUB_K));
            curve(cx, r, col, |t| fe * 2f64.powf(d * (-(t / tau).powf(k)).exp()));
            env_curve(cx, "sub", r, [SUB_ATT, SUB_DEC, SUB_HOLD, SUB_REL, SUB_SHAPE]);
            if let Some(p) = handle(cx, "sub_dive", pos2(r.left() + 1.0, fy(r, fe * 2f64.powf(d))), col, true, (false, true)) {
                cx.set_val(SUB_DIVE, (12.0 * (yf(r, p.y) / fe).log2()).clamp(SPEC[SUB_DIVE].min, SPEC[SUB_DIVE].max));
            }
            let ft = fe * 2f64.powf(d * (-1f64).exp());
            if let Some(p) = handle(cx, "sub_tau", pos2(tx(r, tau * 1000.0), fy(r, ft)), col, true, (true, false)) {
                cx.set_val(SUB_TAU, tms(r, p.x).clamp(SPEC[SUB_TAU].min, SPEC[SUB_TAU].max));
            }
        }
        "transient" => {
            env_curve(cx, "tr", r, [TR_ATT, TR_DEC, TR_HOLD, TR_REL, TR_SHAPE]);
            let _ = col;
        }
        "body" => {
            let (f0, f1, ft) = (cx.gun(BODY_FLT_F0), cx.gun(BODY_FLT_F1), cx.gun(BODY_FLT_TAU) / 1000.0);
            if cx.gun(BODY_FLT_MIX) > 0.001 {
                curve(cx, r, col, |t| (f1.ln() + (f0.ln() - f1.ln()) * (-t / ft).exp()).exp());
            }
            let my = fy(r, cx.gun(BODY_MODE_F));
            dashed(cx, pos2(r.left(), my), pos2(r.right(), my), col);
            env_curve(cx, "body", r, [BODY_ATT, BODY_DEC, BODY_HOLD, BODY_REL, BODY_SHAPE]);
            if let Some(p) = handle(cx, "flt_f0", pos2(r.left() + 1.0, fy(r, f0)), col, false, (false, true)) {
                cx.set_val(BODY_FLT_F0, yf(r, p.y).clamp(SPEC[BODY_FLT_F0].min, SPEC[BODY_FLT_F0].max));
            }
            let fat = (f1.ln() + (f0.ln() - f1.ln()) * (-1f64).exp()).exp();
            if let Some(p) = handle(cx, "flt_tau", pos2(tx(r, ft * 1000.0), fy(r, fat)), col, false, (true, true)) {
                cx.set_val(BODY_FLT_TAU, tms(r, p.x).clamp(SPEC[BODY_FLT_TAU].min, SPEC[BODY_FLT_TAU].max));
                // vertical drag moves the closed frequency so the curve passes through the pointer at t = τ
                let target = yf(r, p.y).ln();
                let e = (-1f64).exp();
                let lf1 = (target - f0.ln() * e) / (1.0 - e);
                cx.set_val(BODY_FLT_F1, lf1.exp().clamp(SPEC[BODY_FLT_F1].min, SPEC[BODY_FLT_F1].max));
            }
            if let Some(p) = handle(cx, "mode_f", pos2(r.right() - 90.0, my), col, true, (false, true)) {
                cx.set_val(BODY_MODE_F, yf(r, p.y).clamp(SPEC[BODY_MODE_F].min, SPEC[BODY_MODE_F].max));
            }
        }
        "reverb" => hold_marker(cx, "rev_size", r, REV_SIZE, "size"),
        "spice" => {
            // the dots sit on the first and last bleeps; the start dot moves the layer (left of zero crops
            // into it), the end dot rescales the length so the last bleep lands under it and the first stays
            let (d, l) = (cx.gun(SPICE_DELAY), cx.gun(SPICE_LEN));
            let y = r.center().y;
            let (on, off) = spans_ms.unwrap_or((d.max(0.0), d + l));
            if let Some(_p) = handle(cx, "spice_delay", pos2(tx(r, on), y), col, true, (true, false)) {
                let dx = cx.ui.input(|i| i.pointer.delta().x);
                if dx != 0.0 { cx.set_val(SPICE_DELAY, (d + (dx / r.width()) as f64 * view_span()).clamp(SPEC[SPICE_DELAY].min, SPEC[SPICE_DELAY].max)); }
            }
            if off < view_end() && off > on + 1.0 {
                if let Some(_p) = handle(cx, "spice_len", pos2(tx(r, off), y), col, true, (true, false)) {
                    let dx = cx.ui.input(|i| i.pointer.delta().x);
                    if dx != 0.0 {
                        let want = (off + (dx / r.width()) as f64 * view_span()).max(on + 2.0);
                        // both bleeps sit at fixed fractions of the length past the delay
                        let (a, b) = ((on - d) / l.max(1.0), (off - d) / l.max(1.0));
                        let nl = ((want - on) / (b - a).max(1e-3)).clamp(SPEC[SPICE_LEN].min, SPEC[SPICE_LEN].max);
                        cx.set_val(SPICE_LEN, nl);
                        cx.set_val(SPICE_DELAY, (on - a * nl).clamp(SPEC[SPICE_DELAY].min, SPEC[SPICE_DELAY].max));
                    }
                }
            }
        }
        n if crate::worker::sample_slot(n).is_some() => {
            // start crops into the file (drag right to skip more of it); the end sets how long it plays
            let base = S1_LVL + 10 * (crate::worker::sample_slot(n).unwrap() - 1);
            let (start, len) = (base + 3, base + 4);
            let y = r.center().y;
            let sx = r.left() + 1.0;
            if let Some(p) = handle(cx, &format!("{n}_start"), pos2(sx, y), col, false, (true, false)) {
                let drag = cx.ui.input(|i| i.pointer.delta().x);
                cx.set_val(start, (cx.gun(start) + (drag / r.width()) as f64 * view_span()).clamp(SPEC[start].min, SPEC[start].max));
                let _ = p;
            }
            if cx.gun(start) > 0.5 { cx.text(pos2(sx + 10.0, y - 12.0), egui::Align2::LEFT_CENTER, &format!("+{:.0} ms", cx.gun(start)), mono(9.0), DIM); }
            if cx.gun(len) < view_end() {
                if let Some(p) = handle(cx, &format!("{n}_len"), pos2(tx(r, cx.gun(len)), y), col, true, (true, false)) { cx.set_val(len, tms(r, p.x).clamp(SPEC[len].min, SPEC[len].max)); }
            }
        }
        _ => {}
    }
}

// ───────────────────────── knobs

fn arc_pts(c: Pos2, rad: f32, u0: f32, u1: f32) -> Vec<Pos2> {
    let a = |u: f32| 0.75 * std::f32::consts::PI + 1.5 * std::f32::consts::PI * u.clamp(0.0, 1.0);
    let (a0, a1) = (a(u0), a(u1).max(a(u0) + 0.01));
    let n = ((a1 - a0) * 12.0).ceil().max(2.0) as usize;
    (0..=n).map(|i| { let t = a0 + (a1 - a0) * i as f32 / n as f32; pos2(c.x + rad * t.cos(), c.y + rad * t.sin()) }).collect()
}

/// Knob bound to a parameter: vertical drag (shift = fine), double-click = default.
fn knob(cx: &mut Cx, id: (usize, usize), fp: &FloatParam, center: Pos2, rad: f32, col: Color32, range: Option<f32>) -> egui::Response {
    let pt = cx.painter();
    let r = Rect::from_center_size(center, vec2(rad * 2.0 + 10.0, rad * 2.0 + 10.0));
    let resp = cx.ui.interact(r, egui::Id::new(("knob", id)), Sense::click_and_drag());
    let u = fp.unmodulated_normalized_value();
    let w = rad * 0.2;
    // hover lifts the track; a held knob sits on a disc of its own color
    let held = resp.dragged() || resp.is_pointer_button_down_on();
    if held { pt.circle_filled(center, rad + w * 1.5 + 3.0, alpha(col, 0.16)); }
    let track = if held || resp.hovered() { Color32::from_rgb(0x44, 0x44, 0x48) } else { TRACK };
    pt.add(Shape::line(arc_pts(center, rad, 0.0, 1.0), Stroke::new(w, track)));
    if let Some(s) = range { if s > 0.0 { pt.add(Shape::line(arc_pts(center, rad, u - s, u + s), Stroke::new(w * 2.3, alpha(col, 0.3)))); } }
    pt.add(Shape::line(arc_pts(center, rad, 0.0, u), Stroke::new(w, col)));
    vertical_drag(cx, &resp, fp);
    // scrolling over a knob turns it, through the same unstepped value as a drag
    if resp.hovered() && !resp.dragged() {
        let dy = wheel(cx).1;
        if dy != 0.0 {
            let raw_id = resp.id.with("wheel");
            let u0 = cx.ui.memory(|m| m.data.get_temp::<(f32, f32)>(raw_id)).filter(|&(set, _)| (set - u).abs() < 1e-4).map_or(u, |(_, raw)| raw);
            let raw = (u0 + dy / if cx.ui.input(|i| i.modifiers.shift) { 2000.0 } else { 400.0 }).clamp(0.0, 1.0);
            cx.setter.begin_set_parameter(fp);
            cx.setter.set_parameter_normalized(fp, raw);
            cx.setter.end_set_parameter(fp);
            let set = fp.unmodulated_normalized_value();
            cx.ui.memory_mut(|m| m.data.insert_temp(raw_id, (set, raw)));
        }
    }
    if resp.double_clicked() {
        cx.setter.begin_set_parameter(fp);
        cx.setter.set_parameter_normalized(fp, fp.default_normalized_value());
        cx.setter.end_set_parameter(fp);
    }
    if resp.hovered() || resp.dragged() { cx.ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeVertical); }
    resp
}

/// A choice between named options, or a stepped value with at most eight steps.
fn is_choice(s: &tearout_dsp::Spec) -> bool { crate::params::is_named(s) || (s.scale == tearout_dsp::Scale::Int && s.max - s.min <= 7.0 && s.unit != "midi") }

/// Gun knob `i`: a knob, or for a choice a button that steps through the options (right-click steps back).
/// Returns whether it drew a knob.
fn control(cx: &mut Cx, id: (usize, usize), i: usize, center: Pos2, rad: f32, col: Color32, range: Option<f32>) -> bool {
    let s = &SPEC[i];
    if !is_choice(s) { knob(cx, id, &cx.p.gun.p[i], center, rad, col, range); return true; }
    let names: Vec<String> = (0..=(s.max - s.min) as usize).map(|k| fmt_value(s, s.min + k as f64)).collect();
    let opts: Vec<&str> = names.iter().map(|x| x.as_str()).collect();
    let k = ((cx.gun(i).round() - s.min) as usize).min(opts.len() - 1);
    // a fixed-size button; the option shown shrinks only as far as it must to fit inside
    let w = if rad > 12.0 { 52.0 } else { 32.0 };
    let size = if rad > 12.0 { 11.0 } else { 9.5 };
    let tw = cx.text_w(opts[k], sans(size));
    let font = sans((size * (w - 6.0) / tw.max(1.0)).min(size));
    // in the main strip a button sits a little low, level with the knobs' weight
    let center = if rad > 12.0 { center + vec2(0.0, 6.0) } else { center };
    let r = Rect::from_center_size(center, vec2(w, if rad > 12.0 { 22.0 } else { 18.0 }));
    let resp = cx.ui.interact(r, egui::Id::new(("knob", id)), Sense::click());
    let fill = if resp.hovered() { Color32::from_rgb(0x38, 0x38, 0x3c) } else { TRACK };
    let pt = cx.painter();
    pt.rect_filled(r, CornerRadius::same(4), fill);
    pt.text(r.center(), egui::Align2::CENTER_CENTER, opts[k], font, if k > 0 { col } else { TEXT });
    if resp.clicked() { cx.set_val(i, s.min + ((k + 1) % opts.len()) as f64); }
    if resp.secondary_clicked() { cx.set_val(i, s.min + ((k + opts.len() - 1) % opts.len()) as f64); }
    false
}

/// A knob's value as typed into its field.
fn edit_text(s: &tearout_dsp::Spec, v: f64) -> String {
    if s.scale == tearout_dsp::Scale::Int { return format!("{}", v.round()); }
    let t = format!("{:.2}", v);
    t.trim_end_matches('0').trim_end_matches('.').to_string()
}

/// A typed value in the knob's units: the first number in the text (a "k" after it means thousands).
fn parse_value(s: &tearout_dsp::Spec, text: &str) -> Option<f64> {
    let t = text.replace('−', "-").replace(',', ".").to_lowercase();
    let start = t.find(|c: char| c.is_ascii_digit() || c == '-' || c == '.')?;
    let rest = &t[start..];
    let end = rest.char_indices().skip(1).find(|&(_, c)| !(c.is_ascii_digit() || c == '.')).map_or(rest.len(), |(j, _)| j);
    let mut v: f64 = rest[..end].parse().ok()?;
    if rest[end..].trim_start().starts_with('k') { v *= 1000.0; }
    Some(v.clamp(s.min, s.max))
}

/// The value under a knob: double-click to type a new one (enter sets it, escape leaves it).
fn value_field(cx: &mut Cx, st: &mut Ui, i: usize, at: Pos2, font: FontId, col: Color32) {
    let s = &SPEC[i];
    let shown = fmt_value(s, cx.gun(i));
    let w = cx.text_w(&shown, font.clone()).max(36.0) + 8.0;
    let r = Rect::from_center_size(at, vec2(w, font.size + 6.0));
    if let Some((ei, buf)) = st.editing.as_mut() {
        if *ei == i {
            cx.ui.painter().rect_filled(r, CornerRadius::same(3), TRACK);
            let resp = cx.ui.put(r, egui::TextEdit::singleline(buf).font(font).horizontal_align(egui::Align::Center).frame(false).text_color(BRIGHT).desired_width(w));
            if !resp.has_focus() && !resp.lost_focus() { resp.request_focus(); }
            if resp.lost_focus() {
                let cancel = cx.ui.input(|inp| inp.key_pressed(egui::Key::Escape));
                if !cancel { if let Some(v) = parse_value(s, buf) { cx.set_val(i, v); } }
                st.editing = None;
            }
            return;
        }
    }
    let resp = cx.ui.interact(r, egui::Id::new(("value", i)), Sense::click());
    cx.text(at, egui::Align2::CENTER_CENTER, &shown, font, if resp.hovered() { BRIGHT } else { col });
    if resp.double_clicked() { st.editing = Some((i, edit_text(s, cx.gun(i)))); }
}

/// The lanes' time window: pinch, or command/control with the wheel, zooms around the pointer; sideways
/// scrolling (or shift with the wheel) pans. It spans 20 ms up to the whole sound.
fn time_window(cx: &mut Cx, st: &mut Ui, view: &View) {
    let lanes = Rect::from_min_max(pos2(LANE_X, LANES_TOP), pos2(W - 20.0, STRIP_TOP - 12.0));
    let len = view.len_ms.max(20.0);
    let hover = cx.ui.input(|i| i.pointer.hover_pos()).filter(|p| lanes.contains(*p));
    #[cfg(target_os = "macos")]
    let pinch = crate::macos::take_pinch();
    #[cfg(not(target_os = "macos"))]
    let pinch = 0.0f32;
    if let Some(p) = hover {
        let (dx, dy) = wheel(cx);
        let m = cx.ui.input(|i| i.modifiers);
        let zoom = if m.command || m.ctrl { (-dy as f64 * 0.01).exp() } else { 1.0 } * (-pinch as f64).exp();
        let at = st.t0 + ((p.x - lanes.left()) / lanes.width()) as f64 * st.span;
        if zoom != 1.0 {
            let span = (st.span * zoom).clamp(20.0, len);
            st.t0 = at - (at - st.t0) * span / st.span;
            st.span = span;
        }
        // egui-baseview hands over sideways scrolling mirrored on macOS
        let pan = if m.shift && !(m.command || m.ctrl) { dy } else if cfg!(target_os = "macos") { -dx } else { dx };
        if pan != 0.0 { st.t0 -= (pan / lanes.width()) as f64 * st.span; }
    }
    st.span = st.span.clamp(20.0, len);
    st.t0 = st.t0.clamp(0.0, (len - st.span).max(0.0));
    WINDOW.with(|w| w.set((st.t0, st.span)));
    // zoomed in, ask for sharp spectrograms of the window, and take them up as they come
    *cx.sh.zoom_req.lock().unwrap() = (st.span < view.len_ms / 1.5 && st.span < VIEW_MS * 0.67).then_some((st.t0, st.span));
    let z = cx.sh.zoom.lock().unwrap().clone();
    if let Some(z) = z {
        if st.zoom.as_ref().map_or(true, |o| !Arc::ptr_eq(o, &z)) {
            let ctx = cx.ui.ctx().clone();
            st.ztex = z.rgba.iter().enumerate().map(|(k, px)| ctx.load_texture(format!("zoom{k}"), egui::ColorImage::from_rgba_unmultiplied([crate::worker::SPEC_COLS, crate::worker::SPEC_ROWS], px), egui::TextureOptions::LINEAR)).collect();
            st.zoom = Some(z);
        }
    }
}

/// This frame's mouse wheel and trackpad scrolling, in points (x, y).
fn wheel(cx: &Cx) -> (f32, f32) {
    cx.ui.input(|i| i.events.iter().fold((0.0, 0.0), |acc, e| match e {
        egui::Event::MouseWheel { unit, delta, .. } => {
            let k = match unit { egui::MouseWheelUnit::Point => 1.0, egui::MouseWheelUnit::Line => 24.0, egui::MouseWheelUnit::Page => 300.0 };
            (acc.0 + delta.x * k, acc.1 + delta.y * k)
        }
        _ => acc,
    }))
}

/// Vertical drag on `fp` with the cursor pinned; shift is fine. The drag runs on an unstepped value, so stepped
/// knobs move once the drag crosses a step.
fn vertical_drag(cx: &mut Cx, resp: &egui::Response, fp: &FloatParam) {
    let u = fp.unmodulated_normalized_value();
    let raw_id = resp.id.with("raw");
    if resp.drag_started() {
        cx.setter.begin_set_parameter(fp);
        cx.ui.memory_mut(|m| m.data.insert_temp(raw_id, u));
        crate::grab::begin();
    }
    if resp.dragged() {
        let fine = cx.ui.input(|i| i.modifiers.shift);
        let zoom = cx.ui.ctx().zoom_factor();
        let dy = crate::grab::delta().map(|d| d.1 as f32 / zoom).unwrap_or(resp.drag_delta().y);
        let du = -dy / if fine { 1000.0 } else { 200.0 };
        let raw = (cx.ui.memory(|m| m.data.get_temp::<f32>(raw_id)).unwrap_or(u) + du).clamp(0.0, 1.0);
        cx.ui.memory_mut(|m| m.data.insert_temp(raw_id, raw));
        if (raw - u).abs() > 1e-7 { cx.setter.set_parameter_normalized(fp, raw); }
    }
    if resp.drag_stopped() { cx.setter.end_set_parameter(fp); crate::grab::end(); }
}

const CORE: [(usize, &str, Color32); 12] = [
    (SUB_LVL, "sub", SUB_C), (SUB_TAU, "dive time", SUB_C), (SUB_SWELL, "swell", SUB_C), (SUB_DRIVE, "drive", SUB_C),
    (TR_LVL, "transient", TR_C),
    (BODY_LVL, "body", BODY_C), (BODY_DEC, "decay", BODY_C), (BODY_SHAPE, "shape", BODY_C), (BODY_HOLD, "hold", BODY_C), (BODY_DRIVE, "drive", BODY_C),
    (CR_MIX, "crunch", MST_C), (MST_HIGH, "high", MST_C),
];

fn core_strip(cx: &mut Cx, st: &mut Ui) {
    strip_buttons(cx, st);
    let x0 = 118.0;
    let picked = st.view.as_ref().and_then(|v| v.lanes.get(st.sel)).and_then(|ln| crate::worker::sample_slot(ln.name).map(|k| (k, rgb(ln.color))));
    if let Some((slot, col)) = picked {
        let step = (W - 20.0 - x0) / CORE.len() as f32;
        for j in 0..10 {
            let i = S1_LVL + 10 * (slot - 1) + j;
            let c = pos2(x0 + step * (j as f32 + 0.5), STRIP_TOP + 30.0);
            if control(cx, (2, i), i, c, 15.0, col, None) { value_field(cx, st, i, pos2(c.x, c.y + 28.0), mono(10.0), TEXT); }
            cx.text(pos2(c.x, c.y + 42.0), egui::Align2::CENTER_CENTER, SPEC[i].label, sans(10.0), Color32::from_rgb(0x7a, 0x7a, 0x7c));
        }
        return;
    }
    let step = (W - 20.0 - x0) / CORE.len() as f32;
    let dice = cx.p.dice.value();
    let amt = cx.p.dice_amt.value();
    for (k, &(i, name, col)) in CORE.iter().enumerate() {
        let c = pos2(x0 + step * (k as f32 + 0.5), STRIP_TOP + 30.0);
        let range = DICE.iter().find(|d| d.0 == i).map(|d| d.1 as f32 * amt * 2.0).filter(|_| dice);
        if control(cx, (0, k), i, c, 15.0, col, range) { value_field(cx, st, i, pos2(c.x, c.y + 28.0), mono(10.0), TEXT); }
        cx.text(pos2(c.x, c.y + 42.0), egui::Align2::CENTER_CENTER, name, sans(10.0), Color32::from_rgb(0x7a, 0x7a, 0x7c));
    }
}

// sections: (title, spec group, colour, switch param)
const SECTIONS: [(&str, &str, Color32, Option<usize>); 10] = [
    ("sub", "sub", SUB_C, Some(SUB_LVL)),
    ("transient", "tr", TR_C, Some(TR_LVL)),
    ("body", "body", BODY_C, Some(BODY_LVL)),
    ("synth", "synth", SYN_C, Some(SYN_LVL)),
    ("spice", "spice", SPICE_C, Some(SPICE_LVL)),
    ("reverb", "rev", REV_C, Some(REV_LVL)),
    ("crunch", "crunch", MST_C, Some(CR_MIX)),
    ("global", "global", MST_C, None),
    ("master", "master", MST_C, None),
    ("burst", "burst", MST_C, None),
];
const COLS: [(usize, &[usize]); 7] = [(3, &[0]), (4, &[1]), (6, &[2]), (4, &[3]), (2, &[4, 5]), (3, &[6]), (3, &[8, 7, 9])];

/// The signature in the knobs tab's lower left, with links to GitHub and X beside it.
fn credit(cx: &mut Cx, st: &mut Ui) {
    const SIG: &[u8] = include_bytes!("../assets/credit/handsig.rgba");
    const SIG_SIZE: [usize; 2] = [556, 280];
    const GH: &[u8] = include_bytes!("../assets/credit/github.rgba");
    const X: &[u8] = include_bytes!("../assets/credit/x.rgba");
    let ctx = cx.ui.ctx().clone();
    let t = st.credit.get_or_insert_with(|| {
        let load = |name: &str, size: [usize; 2], px: &[u8]| ctx.load_texture(name, egui::ColorImage::from_rgba_unmultiplied(size, px), egui::TextureOptions::LINEAR);
        [load("sig", SIG_SIZE, SIG), load("github", [64, 64], GH), load("x", [64, 64], X)]
    });
    let ids = [t[0].id(), t[1].id(), t[2].id()];
    let faint = Color32::from_rgb(0x2a, 0x2a, 0x2d);
    let uv = Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0));
    let pt = cx.painter();
    let h = 34.0;
    let sig = Rect::from_min_size(pos2(12.0, H - h - 4.0), vec2(h * SIG_SIZE[0] as f32 / SIG_SIZE[1] as f32, h));
    pt.image(ids[0], sig, uv, faint);
    let mut x = sig.right() + 4.0;
    for (k, url) in [(1, "https://github.com/lyramakesmusic/tearout-gun"), (2, "https://x.com/_lyraaaa_")] {
        // centred on the writing's x-height band
        let r = Rect::from_center_size(pos2(x + 6.0, sig.top() + h * 0.54), vec2(12.0, 12.0));
        let resp = cx.ui.interact(r.expand(3.0), egui::Id::new(("credit", k)), Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand);
        pt.image(ids[k], r, uv, if resp.hovered() { DIM } else { faint });
        if resp.clicked() { ctx.open_url(egui::OpenUrl::new_tab(url)); }
        x += 18.0;
    }
}

fn knobs_tab(cx: &mut Cx, st: &mut Ui) {
    credit(cx, st);
    let (kw, kh) = (35.0, 50.0);
    // sections spread across the width; faint rules sit in the gaps between them
    let used: f32 = COLS.iter().map(|c| c.0 as f32 * kw).sum();
    let gap = (W - 40.0 - used) / (COLS.len() - 1) as f32;
    let mut x = 20.0;
    for (ci, &(ncol, secs)) in COLS.iter().enumerate() {
        if ci > 0 { let rx = x - gap / 2.0; cx.ui.painter().line_segment([pos2(rx, 54.0), pos2(rx, H - 44.0)], Stroke::new(1.0, Color32::from_rgb(0x1c, 0x1c, 0x1f))); }
        let mut y = 54.0;
        for &si in secs {
            let (title, g, col, sw) = SECTIONS[si];
            let on = sw.map_or(true, |i| cx.p.gun.p[i].unmodulated_normalized_value() > 0.0005);
            let (tc, kc) = if on { (col, col) } else { (Color32::from_rgb(0x5c, 0x5c, 0x5e), Color32::from_rgb(0x4a, 0x4a, 0x4d)) };
            cx.text(pos2(x, y + 7.0), egui::Align2::LEFT_CENTER, title, semi(12.0), tc);
            if let Some(i) = sw {
                let br = Rect::from_center_size(pos2(x + ncol as f32 * kw - 14.0, y + 7.0), vec2(16.0, 16.0));
                let resp = cx.ui.interact(br, egui::Id::new(("power", si)), Sense::click());
                power_icon(cx, br.center(), if on { col } else { Color32::from_rgb(0x5c, 0x5c, 0x5e) }, on);
                if resp.clicked() {
                    if on { st.mem[si.min(7)] = cx.p.gun.p[i].unmodulated_normalized_value(); cx.set_norm(i, 0.0); } else { cx.set_norm(i, st.mem[si.min(7)].max(0.3)); }
                }
            }
            y += 20.0;
            let ids: Vec<usize> = (0..SPEC.len()).filter(|&i| SPEC[i].group == g).collect();
            for (k, &i) in ids.iter().enumerate() {
                let (c, r) = (k % ncol, k / ncol);
                let center = pos2(x + kw * (c as f32 + 0.5), y + r as f32 * kh + 14.0);
                let is_knob = control(cx, (1, i), i, center, 10.5, kc, None);
                let resp_on = is_knob && cx.ui.ctx().read_response(egui::Id::new(("knob", (1usize, i)))).is_some_and(|r| r.hovered() || r.dragged());
                let lr = Rect::from_center_size(pos2(center.x, center.y + 25.0), vec2(kw, 20.0));
                if st.editing.as_ref().is_some_and(|e| e.0 == i) || (is_knob && cx.ui.interact(lr, egui::Id::new(("klabel", i)), Sense::click()).double_clicked()) {
                    if st.editing.as_ref().map_or(true, |e| e.0 != i) { st.editing = Some((i, edit_text(&SPEC[i], cx.gun(i)))); }
                    value_field(cx, st, i, pos2(center.x, center.y + 20.0), sans(8.5), TEXT);
                    continue;
                }
                let label = if resp_on { fmt_value(&SPEC[i], cx.gun(i)) } else { SPEC[i].label.to_string() };
                let mut lines: Vec<String> = Vec::new();
                for w in label.split(' ') {
                    match lines.last_mut() {
                        Some(l) if cx.text_w(&format!("{l} {w}"), sans(8.5)) <= kw - 3.0 => { l.push(' '); l.push_str(w); }
                        _ => lines.push(w.to_string()),
                    }
                }
                for (j, l) in lines.iter().take(2).enumerate() {
                    cx.text(pos2(center.x, center.y + 20.0 + j as f32 * 10.0), egui::Align2::CENTER_CENTER, l, sans(8.5), Color32::from_rgb(0x7a, 0x7a, 0x7c));
                }
            }
            let mut count = ids.len();
            if g == "burst" || g == "master" {
                let (c, r) = (count % ncol, count / ncol);
                let center = pos2(x + kw * (c as f32 + 0.5), y + r as f32 * kh + 14.0);
                let (text, on, label) = if g == "burst" {
                    let fp = &cx.p.shots_grid;
                    (fp.to_string(), fp.value() > 0.5, "grid")
                } else {
                    (if cx.p.limit.value() { "on" } else { "off" }.to_string(), cx.p.limit.value(), "limiter")
                };
                let w = cx.text_w(&text, sans(9.0)).max(22.0) + 12.0;
                let br = Rect::from_center_size(center, vec2(w.min(kw - 2.0).max(30.0), 18.0));
                let resp = cx.ui.interact(br, egui::Id::new(("extra", g)), Sense::click());
                cx.ui.painter().rect_filled(br, CornerRadius::same(4), if resp.hovered() { Color32::from_rgb(0x38, 0x38, 0x3c) } else { TRACK });
                cx.text(br.center(), egui::Align2::CENTER_CENTER, &text, sans(9.0), if on { kc } else { TEXT });
                cx.text(pos2(center.x, center.y + 20.0), egui::Align2::CENTER_CENTER, label, sans(8.5), Color32::from_rgb(0x7a, 0x7a, 0x7c));
                if g == "burst" {
                    let fp = &cx.p.shots_grid;
                    let n = crate::params::GRID.len() as f32;
                    let v = fp.value().round();
                    let next = if resp.clicked() { Some((v + 1.0) % n) } else if resp.secondary_clicked() { Some((v + n - 1.0) % n) } else { None };
                    if let Some(nv) = next { cx.setter.begin_set_parameter(fp); cx.setter.set_parameter(fp, nv); cx.setter.end_set_parameter(fp); }
                } else if resp.clicked() {
                    let fp = &cx.p.limit;
                    cx.setter.begin_set_parameter(fp); cx.setter.set_parameter(fp, !fp.value()); cx.setter.end_set_parameter(fp);
                }
                count += 1;
            }
            y += ((count + ncol - 1) / ncol) as f32 * kh + 10.0;
        }
        x += ncol as f32 * kw + gap;
    }
}

/// A speaker: sounding (waves) or muted (a cross).
fn power_icon(cx: &Cx, c: Pos2, col: Color32, on: bool) {
    let pt = cx.painter();
    let x = c.x - 3.0;
    pt.add(Shape::convex_polygon(vec![pos2(x - 4.0, c.y - 2.5), pos2(x - 1.5, c.y - 2.5), pos2(x + 2.5, c.y - 6.0), pos2(x + 2.5, c.y + 6.0), pos2(x - 1.5, c.y + 2.5), pos2(x - 4.0, c.y + 2.5)], col, Stroke::NONE));
    let s = Stroke::new(1.3, col);
    if on {
        for rad in [3.5f32, 6.5] {
            let pts: Vec<Pos2> = (0..=8).map(|i| { let a = -0.9 + 1.8 * i as f32 / 8.0; pos2(x + 2.5 + rad * a.cos(), c.y + rad * a.sin()) }).collect();
            pt.add(Shape::line(pts, s));
        }
    } else {
        pt.line_segment([pos2(x + 5.0, c.y - 3.0), pos2(x + 10.0, c.y + 3.0)], s);
        pt.line_segment([pos2(x + 5.0, c.y + 3.0), pos2(x + 10.0, c.y - 3.0)], s);
    }
}

// ───────────────────────── play


fn drag_out(st: &Ui, view: &View, lanes: &[usize]) {
    let preset = st.list.get(st.preset).map(|p| p.name.clone()).unwrap_or_default();
    let sr = tearout_dsp::SR as u32;
    let paths: Vec<_> = lanes
        .iter()
        .filter_map(|&i| view.lanes.get(i))
        .filter_map(|ln| crate::export::lane_file(&preset, ln.name, &note_name(view.p.0[NOTE]).replace('♯', "#"), view.patch, &ln.l, &ln.r, sr).map_err(|e| nih_log!("export failed: {e}")).ok())
        .collect();
    if paths.is_empty() { return; }
    nih_log!("drag out: {} file(s), first {}", paths.len(), paths[0].display());
    #[cfg(target_os = "macos")]
    if !crate::macos::drag_files(&paths) { nih_log!("drag did not start"); }
    #[cfg(windows)]
    if !crate::win::drag_files(&paths) { nih_log!("drag did not start"); }
    // on Linux the stems are written and their folder opens once per session, for dragging from the file manager
    #[cfg(not(any(target_os = "macos", windows)))]
    {
        static OPENED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
        if !OPENED.swap(true, std::sync::atomic::Ordering::Relaxed) { let _ = std::process::Command::new("xdg-open").arg(crate::export::dir()).spawn(); }
    }
}

fn preset_menu(cx: &mut Cx, st: &mut Ui) {
    let presets = st.list.clone();
    let row = 22.0;
    let users = presets.iter().filter(|p| p.user).count();
    let foot = 30.0;
    let h = (presets.len() as f32 * row + 8.0 + if users > 0 { 9.0 } else { 0.0 } + foot).min(H - 60.0);
    let area = Rect::from_min_size(pos2(12.0, 40.0), vec2(220.0, h));
    let ctx = cx.ui.ctx().clone();
    let mut pick = None;
    let mut delete = None;
    let mut change = false;
    let resp = egui::Area::new(egui::Id::new("preset-menu")).fixed_pos(area.min).order(egui::Order::Foreground).show(&ctx, |ui| {
        let pt = ui.painter().clone();
        pt.rect_filled(area.translate(vec2(0.0, 2.0)).expand(1.0), CornerRadius::same(5), Color32::from_black_alpha(90));
        pt.rect_filled(area, CornerRadius::same(4), Color32::from_rgb(0x1c, 0x1c, 0x1e));
        ui.set_min_size(area.size());
        egui::ScrollArea::vertical().max_height(h - 8.0 - foot).show(ui, |ui| {
            ui.add_space(4.0);
            for (i, p) in presets.iter().enumerate() {
                // saved presets follow the factory ones after a gap; right-click deletes one
                if p.user && (i == 0 || !presets[i - 1].user) { ui.add_space(9.0); }
                let (r, rr) = ui.allocate_exact_size(vec2(220.0, row), Sense::click());
                if p.user && rr.secondary_clicked() { delete = Some(p.name.clone()); }
                let on = i == st.preset;
                if rr.hovered() { ui.painter().rect_filled(r.shrink2(vec2(4.0, 1.0)), CornerRadius::same(3), Color32::from_rgb(0x2e, 0x2e, 0x31)); }
                ui.painter().text(pos2(r.left() + 14.0, r.center().y), egui::Align2::LEFT_CENTER, &p.name, if on { semi(12.0) } else { sans(12.0) }, if on { BRIGHT } else { TEXT });
                if rr.clicked() { pick = Some(i); }
            }
        });
        // the folder presets and stems live in: its name (full path on hover) opens it; "change" picks another
        let fy = area.bottom() - foot / 2.0;
        ui.painter().line_segment([pos2(area.left() + 8.0, area.bottom() - foot), pos2(area.right() - 8.0, area.bottom() - foot)], Stroke::new(1.0, Color32::from_rgb(0x2a, 0x2a, 0x2d)));
        let path = crate::export::root();
        let name = path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
        let pr = Rect::from_min_max(pos2(area.left() + 8.0, fy - 9.0), pos2(area.right() - 60.0, fy + 9.0));
        let presp = ui.interact(pr, egui::Id::new("folder-path"), Sense::click());
        let fc = if presp.hovered() { TEXT } else { DIM };
        let fx = pr.left() + 6.0;
        let st_ = Stroke::new(1.1, fc);
        ui.painter().add(Shape::closed_line(vec![pos2(fx, fy - 4.0), pos2(fx + 4.0, fy - 4.0), pos2(fx + 5.5, fy - 2.5), pos2(fx + 11.0, fy - 2.5), pos2(fx + 11.0, fy + 4.5), pos2(fx, fy + 4.5)], st_));
        ui.painter().text(pos2(fx + 17.0, fy), egui::Align2::LEFT_CENTER, &name, sans(11.0), fc);
        if presp.clicked() { crate::export::reveal(&path); }
        let cr = Rect::from_min_max(pos2(area.right() - 60.0, fy - 9.0), pos2(area.right() - 8.0, fy + 9.0));
        let cresp = ui.interact(cr, egui::Id::new("folder-change"), Sense::click());
        ui.painter().text(pos2(cr.right() - 4.0, fy), egui::Align2::RIGHT_CENTER, "change…", sans(10.0), if cresp.hovered() { TEXT } else { DIMMER });
        if cresp.clicked() { change = true; }
    });
    if change { pick_folder(); st.list = crate::presets::list(); }
    if let Some(name) = delete { let _ = crate::presets::delete(&name); st.list = crate::presets::list(); st.preset = st.preset.min(st.list.len().saturating_sub(1)); }
    if let Some(i) = pick { load_preset(cx, st, i); st.presets_open = false; }
    else if ctx.input(|i| i.pointer.any_pressed()) && !resp.response.rect.contains(ctx.input(|i| i.pointer.interact_pos()).unwrap_or_default()) && !Rect::from_min_size(pos2(12.0, 8.0), vec2(200.0, 24.0)).contains(ctx.input(|i| i.pointer.interact_pos()).unwrap_or_default()) {
        st.presets_open = false;
    }
}

/// Ableton's computer MIDI keyboard: A..; play from C of the current octave, W E T Y U O P are the sharps,
/// Z/X step the octave, C/V the velocity, M switches it on and off.
fn computer_keys(cx: &mut Cx, st: &mut Ui) {
    use egui::Key::*;
    const NOTES: [(egui::Key, i32); 17] = [(A, 0), (W, 1), (S, 2), (E, 3), (D, 4), (F, 5), (T, 6), (G, 7), (Y, 8), (H, 9), (U, 10), (J, 11), (K, 12), (O, 13), (L, 14), (P, 15), (Semicolon, 16)];
    if cx.ui.ctx().wants_keyboard_input() { return; }
    // keys by position on the keyboard, so caps lock and layouts don't change them
    let presses: Vec<egui::Key> = cx.ui.input(|i| i.events.iter().filter_map(|e| match e { egui::Event::Key { key, physical_key, pressed: true, repeat: false, .. } => Some(physical_key.unwrap_or(*key)), _ => None }).collect());
    for key in presses {
        if key == M { st.keys_on = !st.keys_on; continue; }
        if !st.keys_on { continue; }
        match key {
            Z => { st.keys_oct = (st.keys_oct - 1).max(0); st.keys_moved = cx.ui.input(|i| i.time); }
            X => { st.keys_oct = (st.keys_oct + 1).min(8); st.keys_moved = cx.ui.input(|i| i.time); }
            C => st.keys_vel = (st.keys_vel - 0.15).max(0.05),
            V => st.keys_vel = (st.keys_vel + 0.15).min(1.0),
            _ => if let Some(&(_, k)) = NOTES.iter().find(|n| n.0 == key) {
                let note = (12 * st.keys_oct + k).clamp(0, 127) as u8;
                let _ = cx.sh.keys.0.try_send((note, st.keys_vel));
            },
        }
    }
}

/// Apply a patch through the host so every change is undoable and automatable.
fn apply(cx: &Cx, q: &tearout_dsp::Params) {
    for k in 0..tearout_dsp::N {
        if (cx.gun(k) - q.0[k]).abs() > 1e-9 { cx.set_val(k, q.0[k]); }
    }
}

fn strip_buttons(cx: &mut Cx, st: &mut Ui) {
    let cy = STRIP_TOP + 30.0;
    let items = [("random", 40.0), ("mutate", 88.0)];
    for (k, &(name, x)) in items.iter().enumerate() {
        let c = pos2(x, cy);
        let r = Rect::from_center_size(c, vec2(40.0, 40.0));
        let resp = cx.ui.interact(r, egui::Id::new(("strip-btn", k)), Sense::click());
        let pt = cx.painter();
        let col = if resp.hovered() { BRIGHT } else { Color32::from_rgb(0xa8, 0xa8, 0xaa) };
        pt.rect_stroke(Rect::from_center_size(c, vec2(26.0, 26.0)), CornerRadius::same(5), Stroke::new(1.6, col), StrokeKind::Middle);
        let pips: &[(f32, f32)] = if k == 0 { &[(-6.0, -6.0), (6.0, -6.0), (0.0, 0.0), (-6.0, 6.0), (6.0, 6.0)] } else { &[(-5.0, 5.0), (5.0, -5.0)] };
        for &(dx, dy) in pips { pt.circle_filled(pos2(c.x + dx, c.y + dy), 2.0, col); }
        cx.text(pos2(c.x, c.y + 42.0), egui::Align2::CENTER_CENTER, name, sans(10.0), Color32::from_rgb(0x7a, 0x7a, 0x7c));
        if resp.clicked() {
            let cur = cx.p.gun.snapshot();
            let seed = 1.0 + (st.rng.next() * 1e6).floor();
            let mut q = if k == 0 {
                crate::roll::roll(crate::roll::Mode::Randomize, &cur, seed, 4, 2, &cx.sh.locked.lock().unwrap().clone())
            } else {
                // mutations stay around the patch they started from until something else changes it
                if st.last_mutation.as_ref().map_or(true, |m| !same(m, &cur)) { st.anchor = Some(cur.clone()); }
                crate::roll::roll(crate::roll::Mode::Mutate, st.anchor.as_ref().unwrap_or(&cur), seed, 4, 2, &cx.sh.locked.lock().unwrap().clone())
            };
            q.0[NOTE] = cur.0[NOTE];
            apply(cx, &q);
            st.last_mutation = Some(q);
        }
    }
}

fn same(a: &tearout_dsp::Params, b: &tearout_dsp::Params) -> bool {
    (0..tearout_dsp::N).all(|i| (to_norm(&SPEC[i], a.0[i]) - to_norm(&SPEC[i], b.0[i])).abs() < 1e-4)
}
