// Editor: stems (per-layer sharpened spectrograms with their waveforms and handles), combined (the mix alone),
// knobs (every parameter), play (note behaviour and dice). The core strip sits under the stems.
use crate::analysis::{freq_y, y_freq};
use crate::params::{fmt_value, group_name, note_name, PlugParams};
use crate::worker::{Shared, View, DICE, VIEW_MS};
use crossbeam_channel::Sender;
use nih_plug::prelude::*;
use nih_plug_egui::egui::{self, pos2, vec2, Color32, CornerRadius, FontFamily, FontId, Pos2, Rect, Sense, Shape, Stroke, StrokeKind, TextureHandle};
use nih_plug_egui::{create_egui_editor, egui::Context};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tearout_dsp::id::*;
use tearout_dsp::{from_norm, to_norm, SPEC};

pub const W: f32 = 960.0;
pub const H: f32 = 600.0;
pub static AUDITION: AtomicBool = AtomicBool::new(false);

const BG: Color32 = Color32::from_rgb(0x0f, 0x0f, 0x10);
const HEAD: Color32 = Color32::from_rgb(0x15, 0x15, 0x16);
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
const REV_C: Color32 = Color32::from_rgb(0x9a, 0xa0, 0xa6);
const MST_C: Color32 = Color32::from_rgb(0xc8, 0xc8, 0xc8);

#[derive(PartialEq, Clone, Copy)]
enum Tab { Stems, Knobs, Play }

struct Ui {
    tab: Tab,
    combined: bool,
    preset: usize,
    view: Option<Arc<View>>,
    tex: Vec<TextureHandle>,
    seen: u64,
    sel: usize,
    /// level a switched-off section returns to
    mem: [f32; 8],
    presets_open: bool,
    anchor: Option<tearout_dsp::Params>,
    last_mutation: Option<tearout_dsp::Params>,
    rng: tearout_dsp::dsp::Rng,
}

pub fn create(params: Arc<PlugParams>, shared: Arc<Shared>, _notes: Sender<u8>) -> Option<Box<dyn Editor>> {
    let st = Ui { tab: Tab::Stems, combined: false, preset: crate::presets::default_index(), view: None, tex: Vec::new(), seen: 0, sel: 3, mem: [0.8; 8], presets_open: false, anchor: None, last_mutation: None, rng: tearout_dsp::dsp::Rng::new((std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.subsec_nanos()).unwrap_or(7) % 9973) as f64 + 1.0) };
    let p2 = params.clone();
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
            st.tex = view.lanes.iter().map(|l| ctx.load_texture(l.name, egui::ColorImage::from_rgba_unmultiplied([crate::worker::SPEC_COLS, crate::worker::SPEC_ROWS], &l.rgba), egui::TextureOptions::LINEAR)).collect();
            st.view = Some(view);
            st.seen = v;
        }
    }
    ctx.request_repaint_after(std::time::Duration::from_millis(33));
    // the whole layout scales with the window, keeping its aspect
    let (ww, wh) = params.editor_state.size();
    let scale = (ww as f32 / W).min(wh as f32 / H).clamp(MIN_SCALE, MAX_SCALE);
    if (ctx.zoom_factor() - scale).abs() > 1e-3 { ctx.set_zoom_factor(scale); }
    egui::CentralPanel::default().frame(egui::Frame::NONE.fill(BG)).show(ctx, |ui| {
        let mut cx = Cx { ui, setter, p: params };
        header(&mut cx, st);
        match st.tab {
            Tab::Stems => { stems(&mut cx, st); core_strip(&mut cx, st); }
            Tab::Knobs => knobs_tab(&mut cx, st),
            Tab::Play => play_tab(&mut cx),
        }
        if st.presets_open { preset_menu(&mut cx, st); }
        resize_corner(&mut cx, scale);
    });
}

const MIN_SCALE: f32 = 0.75;
const MAX_SCALE: f32 = 2.5;

fn resize_corner(cx: &mut Cx, scale: f32) {
    let r = Rect::from_min_max(pos2(W - 16.0, H - 16.0), pos2(W, H));
    let resp = cx.ui.interact(r, egui::Id::new("resize"), Sense::drag()).on_hover_cursor(egui::CursorIcon::ResizeNwSe);
    if resp.dragged() {
        if let Some(p) = resp.interact_pointer_pos() {
            // pointer in window pixels, so the scale follows the mouse as the zoom changes under it
            let s = ((p.x + 4.0) * scale / W).max((p.y + 4.0) * scale / H).clamp(MIN_SCALE, MAX_SCALE);
            if (s - scale).abs() > 0.004 { cx.p.editor_state.set_requested_size(((W * s).round() as u32, (H * s).round() as u32)); }
        }
    }
    let col = if resp.hovered() || resp.dragged() { DIM } else { DIMMER };
    let pt = cx.painter();
    for k in [4.0, 8.0, 12.0] { pt.line_segment([pos2(W - 3.0 - k, H - 3.0), pos2(W - 3.0, H - 3.0 - k)], Stroke::new(1.0, col)); }
}

// ───────────────────────── header

fn header(cx: &mut Cx, st: &mut Ui) {
    let pt = cx.painter();
    pt.rect_filled(Rect::from_min_size(pos2(0.0, 0.0), vec2(W, 40.0)), CornerRadius::ZERO, HEAD);
    let presets = crate::presets::all();
    let y = 20.0;
    let mut x = 20.0;
    if cx.button(Rect::from_min_size(pos2(x - 4.0, 8.0), vec2(16.0, 24.0)), "prev", "‹", Color32::TRANSPARENT, DIM, sans(16.0)) { load_preset(cx, st, (st.preset + presets.len() - 1) % presets.len()); }
    x += 16.0;
    let name = presets.get(st.preset).map(|p| p.name.as_str()).unwrap_or("");
    let nw = cx.text_w(name, semi(14.0)).max(70.0);
    let nr = Rect::from_min_size(pos2(x - 4.0, 8.0), vec2(nw + 8.0, 24.0));
    let nresp = cx.ui.interact(nr, egui::Id::new("preset-name"), Sense::click());
    if nresp.clicked() { st.presets_open = !st.presets_open; }
    cx.text(pos2(x, y), egui::Align2::LEFT_CENTER, name, semi(14.0), if nresp.hovered() || st.presets_open { BRIGHT } else { TEXT });
    x += nw + 6.0;
    if cx.button(Rect::from_min_size(pos2(x - 4.0, 8.0), vec2(16.0, 24.0)), "next", "›", Color32::TRANSPARENT, DIM, sans(16.0)) { load_preset(cx, st, (st.preset + 1) % presets.len()); }
    x += 24.0;
    // tabs
    let tabs = [("stems", Tab::Stems), ("knobs", Tab::Knobs), ("play", Tab::Play)];
    let widths: Vec<f32> = tabs.iter().map(|t| cx.text_w(t.0, sans(12.0)) + 20.0).collect();
    let tw: f32 = widths.iter().sum::<f32>() + 2.0 * (tabs.len() as f32 + 1.0);
    pt.rect_filled(Rect::from_min_size(pos2(x, 8.0), vec2(tw, 24.0)), CornerRadius::same(4), Color32::from_rgb(0x1c, 0x1c, 0x1e));
    let mut tx = x + 2.0;
    for (i, (label, t)) in tabs.iter().enumerate() {
        let r = Rect::from_min_size(pos2(tx, 10.0), vec2(widths[i], 20.0));
        let on = st.tab == *t;
        if cx.button(r, label, label, if on { Color32::from_rgb(0x2e, 0x2e, 0x31) } else { Color32::TRANSPARENT }, if on { TEXT } else { DIM }, sans(12.0)) { st.tab = *t; }
        tx += widths[i] + 2.0;
    }
    x += tw + 10.0;
    let cw = cx.text_w("combined", sans(12.0)) + 20.0;
    let on = st.combined && st.tab == Tab::Stems;
    if cx.button(Rect::from_min_size(pos2(x, 8.0), vec2(cw, 24.0)), "combined", "combined", if on { Color32::from_rgb(0x3a, 0x3a, 0x3e) } else { Color32::from_rgb(0x1f, 0x1f, 0x21) }, if on { BRIGHT } else { DIM }, sans(12.0)) {
        if st.tab == Tab::Stems { st.combined = !st.combined; } else { st.tab = Tab::Stems; st.combined = true; }
    }
    // right side: dice, note, audition
    let mut rx = W - 20.0;
    let pr = Rect::from_center_size(pos2(rx - 14.0, y), vec2(28.0, 28.0));
    let resp = cx.ui.interact(pr, egui::Id::new("audition"), Sense::click());
    pt.circle_filled(pr.center(), 14.0, if resp.hovered() { Color32::WHITE } else { BRIGHT });
    let c = pr.center();
    pt.add(Shape::convex_polygon(vec![pos2(c.x - 4.0, c.y - 6.0), pos2(c.x + 7.0, c.y), pos2(c.x - 4.0, c.y + 6.0)], BG, Stroke::NONE));
    if resp.clicked() { AUDITION.store(true, Ordering::Relaxed); }
    rx -= 28.0 + 12.0;
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
    let (fill, fg) = if dice_on { (BRIGHT, BG) } else { (Color32::from_rgb(0x1f, 0x1f, 0x21), DIM) };
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
}

fn load_preset(cx: &Cx, st: &mut Ui, i: usize) {
    let Some(pre) = crate::presets::all().get(i) else { return };
    st.preset = i;
    for k in 0..tearout_dsp::N { cx.set_val(k, pre.p.0[k]); }
    if let Ok(mut n) = cx.p.preset.write() { *n = pre.name.clone(); }
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
    let lanes: Vec<usize> = if st.combined { vec![0] } else { (0..view.lanes.len()).collect() };
    let avail = STRIP_TOP - LANES_TOP - 12.0;
    let gap = 6.0;
    let want: f32 = lanes.iter().map(|&i| if i == 0 { 92.0 } else { 78.0 }).sum::<f32>() + gap * (lanes.len() as f32 - 1.0);
    let k = if st.combined { avail / 92.0 } else { (avail / want).min(1.0) };
    let mut y = LANES_TOP;
    for &li in &lanes {
        let h = (if li == 0 { 92.0 } else { 78.0 }) * k;
        lane(cx, st, &view, li, y, h);
        y += h + gap;
    }
}

fn lane(cx: &mut Cx, st: &mut Ui, view: &View, li: usize, y: f32, h: f32) {
    let pt = cx.painter();
    let ln = &view.lanes[li];
    let col = rgb(ln.color);
    // grip: drag the lane out as a file (option-drag on the mix takes every stem)
    let gr = Rect::from_min_size(pos2(14.0, y), vec2(24.0, h));
    let gresp = cx.ui.interact(gr, egui::Id::new(("grip", li)), Sense::drag());
    let gc = if gresp.hovered() || gresp.dragged() { BRIGHT } else { Color32::from_rgb(0x55, 0x55, 0x5a) };
    for (dx, dy) in [(0.0, -5.0), (4.0, -5.0), (0.0, 0.0), (4.0, 0.0), (0.0, 5.0), (4.0, 5.0)] { pt.circle_filled(pos2(24.0 + dx, y + h / 2.0 + dy), 1.2, gc); }
    if gresp.hovered() { cx.ui.ctx().set_cursor_icon(egui::CursorIcon::Grab); }
    if gresp.drag_started() {
        let all = li == 0 && cx.ui.input(|i| i.modifiers.alt);
        let pick: Vec<usize> = if all { (1..view.lanes.len()).collect() } else { vec![li] };
        drag_out(st, view, &pick);
    }
    // label
    let lr = Rect::from_min_size(pos2(40.0, y), vec2(78.0, h));
    if cx.ui.interact(lr, egui::Id::new(("lane", li)), Sense::click()).clicked() { st.sel = li; }
    if st.sel == li { pt.rect_filled(Rect::from_min_size(pos2(40.0, y + h / 2.0 - 15.0), vec2(2.0, 30.0)), CornerRadius::ZERO, col); }
    cx.text(pos2(50.0, y + h / 2.0 - 7.0), egui::Align2::LEFT_CENTER, ln.name, semi(12.0), col);
    let lvl = if li == 0 { format!("{:.1} pk", view.peak_db) } else { format!("{:.1}", ln.level_db) }.replace('-', "−");
    cx.text(pos2(50.0, y + h / 2.0 + 9.0), egui::Align2::LEFT_CENTER, &lvl, mono(10.0), DIMMER);
    // spectrogram
    let r = Rect::from_min_size(pos2(LANE_X, y), vec2(W - 20.0 - LANE_X, h));
    if let Some(t) = st.tex.get(li) { pt.image(t.id(), r, Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)), Color32::WHITE); }
    // waveform
    let n = ln.wave.len();
    let (cy, a) = (r.center().y, r.height() * 0.46);
    let xs = |c: usize| r.left() + (c as f32 + 0.5) / n as f32 * r.width();
    let fill = Color32::from_rgba_unmultiplied(255, 255, 255, 18);
    let mut top = Vec::with_capacity(n);
    let mut bot = Vec::with_capacity(n);
    for (c, &(hi, lo)) in ln.wave.iter().enumerate() {
        let x = xs(c);
        let (y0, y1) = (cy - hi * a, cy - lo * a);
        pt.rect_filled(Rect::from_min_max(pos2(x - 0.5 * r.width() / n as f32, y0), pos2(x + 0.5 * r.width() / n as f32, y1.max(y0 + 0.5))), CornerRadius::ZERO, fill);
        top.push(pos2(x, y0));
        bot.push(pos2(x, y1));
    }
    let edge = Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 46));
    pt.add(Shape::line(top, edge));
    pt.add(Shape::line(bot, edge));
    overlays(cx, ln.name, r, col);
}

fn tx(r: Rect, ms: f64) -> f32 { r.left() + (ms / VIEW_MS) as f32 * r.width() }
fn tms(r: Rect, x: f32) -> f64 { ((x - r.left()) / r.width()) as f64 * VIEW_MS }
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
fn hold_marker(cx: &mut Cx, id: &str, r: Rect, param: usize, label: &str) {
    let ms = cx.gun(param);
    let pt = cx.painter();
    if ms > VIEW_MS {
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
    let pts: Vec<Pos2> = (0..=240).map(|i| { let ms = VIEW_MS * (i as f64 / 240.0).powf(2.0); pos2(tx(r, ms), fy(r, f(ms / 1000.0))) }).collect();
    cx.painter().add(Shape::line(pts, Stroke::new(2.0, col)));
}

fn dashed(cx: &Cx, a: Pos2, b: Pos2, col: Color32) {
    cx.painter().extend(Shape::dashed_line(&[a, b], Stroke::new(1.0, col), 3.0, 4.0));
}

fn overlays(cx: &mut Cx, name: &str, r: Rect, col: Color32) {
    let pt = cx.painter();
    let clip = pt.clip_rect();
    let _ = clip;
    match name {
        "sub" => {
            let fe = tearout_dsp::dsp::midi_hz(cx.gun(NOTE) + cx.gun(SUB_TUNE));
            let (d, tau, k) = (cx.gun(SUB_DIVE) / 12.0, cx.gun(SUB_TAU) / 1000.0, cx.gun(SUB_K));
            curve(cx, r, col, |t| fe * 2f64.powf(d * (-(t / tau).powf(k)).exp()));
            hold_marker(cx, "sub_hold", r, SUB_HOLD, "hold");
            if let Some(p) = handle(cx, "sub_dive", pos2(r.left() + 1.0, fy(r, fe * 2f64.powf(d))), col, true, (false, true)) {
                cx.set_val(SUB_DIVE, (12.0 * (yf(r, p.y) / fe).log2()).clamp(SPEC[SUB_DIVE].min, SPEC[SUB_DIVE].max));
            }
            let ft = fe * 2f64.powf(d * (-1f64).exp());
            if let Some(p) = handle(cx, "sub_tau", pos2(tx(r, tau * 1000.0), fy(r, ft)), col, true, (true, false)) {
                cx.set_val(SUB_TAU, tms(r, p.x).clamp(SPEC[SUB_TAU].min, SPEC[SUB_TAU].max));
            }
        }
        "transient" => {
            hold_marker(cx, "tr_hold", r, TR_HOLD, "hold");
            let hx = tx(r, cx.gun(TR_HOLD).min(VIEW_MS));
            let y = fy(r, cx.gun(TR_PK_F));
            dashed(cx, pos2(r.left(), y), pos2(hx, y), col);
            if let Some(p) = handle(cx, "tr_pk_f", pos2(hx, y), col, true, (false, true)) {
                cx.set_val(TR_PK_F, yf(r, p.y).clamp(SPEC[TR_PK_F].min, SPEC[TR_PK_F].max));
            }
        }
        "body" => {
            let (f0, f1, ft) = (cx.gun(BODY_FLT_F0), cx.gun(BODY_FLT_F1), cx.gun(BODY_FLT_TAU) / 1000.0);
            if cx.gun(BODY_FLT_MIX) > 0.001 {
                curve(cx, r, col, |t| (f1.ln() + (f0.ln() - f1.ln()) * (-t / ft).exp()).exp());
            }
            let my = fy(r, cx.gun(BODY_MODE_F));
            dashed(cx, pos2(r.left(), my), pos2(r.right(), my), col);
            hold_marker(cx, "body_hold", r, BODY_HOLD, "hold");
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
    pt.add(Shape::line(arc_pts(center, rad, 0.0, 1.0), Stroke::new(w, TRACK)));
    if let Some(s) = range { if s > 0.0 { pt.add(Shape::line(arc_pts(center, rad, u - s, u + s), Stroke::new(w * 2.3, alpha(col, 0.3)))); } }
    pt.add(Shape::line(arc_pts(center, rad, 0.0, u), Stroke::new(w, col)));
    if resp.drag_started() { cx.setter.begin_set_parameter(fp); }
    if resp.dragged() {
        let fine = cx.ui.input(|i| i.modifiers.shift);
        let du = -resp.drag_delta().y / if fine { 1000.0 } else { 200.0 };
        cx.setter.set_parameter_normalized(fp, (u + du).clamp(0.0, 1.0));
    }
    if resp.drag_stopped() { cx.setter.end_set_parameter(fp); }
    if resp.double_clicked() {
        cx.setter.begin_set_parameter(fp);
        cx.setter.set_parameter_normalized(fp, fp.default_normalized_value());
        cx.setter.end_set_parameter(fp);
    }
    if resp.hovered() || resp.dragged() { cx.ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeVertical); }
    resp
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
    let step = (W - 20.0 - x0) / CORE.len() as f32;
    let dice = cx.p.dice.value();
    let amt = cx.p.dice_amt.value();
    for (k, &(i, name, col)) in CORE.iter().enumerate() {
        let c = pos2(x0 + step * (k as f32 + 0.5), STRIP_TOP + 30.0);
        let range = DICE.iter().find(|d| d.0 == i).map(|d| d.1 as f32 * amt * 2.0).filter(|_| dice);
        let fp = &cx.p.gun.p[i];
        knob(cx, (0, k), fp, c, 15.0, col, range);
        let v = fmt_value(&SPEC[i], cx.gun(i));
        cx.text(pos2(c.x, c.y + 28.0), egui::Align2::CENTER_CENTER, &v, mono(10.0), TEXT);
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
const COLS: [(usize, &[usize]); 7] = [(3, &[0]), (4, &[1]), (6, &[2]), (4, &[3]), (2, &[4, 5]), (3, &[6]), (3, &[7, 8, 9])];

fn knobs_tab(cx: &mut Cx, st: &mut Ui) {
    let (kw, kh) = (35.0, 50.0);
    let mut x = 20.0;
    for &(ncol, secs) in COLS.iter() {
        let mut y = 54.0;
        for &si in secs {
            let (title, g, col, sw) = SECTIONS[si];
            let on = sw.map_or(true, |i| cx.p.gun.p[i].unmodulated_normalized_value() > 0.0005);
            let (tc, kc) = if on { (col, col) } else { (Color32::from_rgb(0x5c, 0x5c, 0x5e), Color32::from_rgb(0x4a, 0x4a, 0x4d)) };
            cx.text(pos2(x, y + 7.0), egui::Align2::LEFT_CENTER, title, semi(12.0), tc);
            if let Some(i) = sw {
                let br = Rect::from_center_size(pos2(x + ncol as f32 * kw - 14.0, y + 7.0), vec2(16.0, 16.0));
                let resp = cx.ui.interact(br, egui::Id::new(("power", si)), Sense::click());
                power_icon(cx, br.center(), if on { col } else { Color32::from_rgb(0x5c, 0x5c, 0x5e) });
                if resp.clicked() {
                    if on { st.mem[si.min(7)] = cx.p.gun.p[i].unmodulated_normalized_value(); cx.set_norm(i, 0.0); } else { cx.set_norm(i, st.mem[si.min(7)].max(0.3)); }
                }
            }
            y += 20.0;
            let ids: Vec<usize> = (0..SPEC.len()).filter(|&i| SPEC[i].group == g).collect();
            for (k, &i) in ids.iter().enumerate() {
                let (c, r) = (k % ncol, k / ncol);
                let center = pos2(x + kw * (c as f32 + 0.5), y + r as f32 * kh + 14.0);
                let fp = &cx.p.gun.p[i];
                let resp = knob(cx, (1, i), fp, center, 10.5, kc, None);
                let label = if resp.hovered() || resp.dragged() { fmt_value(&SPEC[i], cx.gun(i)) } else { SPEC[i].label.to_string() };
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
            y += ((ids.len() + ncol - 1) / ncol) as f32 * kh + 10.0;
        }
        x += ncol as f32 * kw + 7.0;
    }
}

fn power_icon(cx: &Cx, c: Pos2, col: Color32) {
    let pt = cx.painter();
    let mut pts = arc_pts(c, 5.0, 0.0, 1.0);
    pts.reverse();
    pt.add(Shape::line(pts, Stroke::new(1.5, col)));
    pt.line_segment([pos2(c.x, c.y - 7.0), pos2(c.x, c.y - 1.0)], Stroke::new(1.5, col));
}

// ───────────────────────── play

fn play_tab(cx: &mut Cx) {
    let items: [(&FloatParam, &str, &str); 4] = [(&cx.p.duck, "duck", "new note"), (&cx.p.duck_rel, "release", "new note"), (&cx.p.vel, "velocity", "keys"), (&cx.p.dice_amt, "dice", "dice")];
    let mut last = "";
    let mut x = 40.0;
    for (k, (fp, name, sec)) in items.iter().enumerate() {
        if *sec != last {
            if !last.is_empty() { x += 40.0; }
            cx.text(pos2(x - 20.0, 70.0), egui::Align2::LEFT_CENTER, sec, semi(12.0), TEXT);
            last = sec;
        }
        let c = pos2(x + 10.0, 120.0);
        knob(cx, (2, k), fp, c, 18.0, BRIGHT, None);
        cx.text(pos2(c.x, c.y + 32.0), egui::Align2::CENTER_CENTER, &fp.to_string(), mono(10.0), TEXT);
        cx.text(pos2(c.x, c.y + 46.0), egui::Align2::CENTER_CENTER, name, sans(10.0), Color32::from_rgb(0x7a, 0x7a, 0x7c));
        x += 70.0;
    }
    let _ = group_name;
}

fn drag_out(st: &Ui, view: &View, lanes: &[usize]) {
    let preset = crate::presets::all().get(st.preset).map(|p| p.name.clone()).unwrap_or_default();
    let sr = tearout_dsp::SR as u32;
    let paths: Vec<_> = lanes
        .iter()
        .filter_map(|&i| view.lanes.get(i))
        .filter_map(|ln| crate::export::lane_file(&preset, ln.name, view.patch, &ln.l, &ln.r, sr).map_err(|e| nih_log!("export failed: {e}")).ok())
        .collect();
    if paths.is_empty() { return; }
    #[cfg(target_os = "macos")]
    if !crate::macos::drag_files(&paths) { nih_log!("drag did not start"); }
    // elsewhere the grip writes the stems and opens their folder for dragging from the file manager
    #[cfg(not(target_os = "macos"))]
    let _ = std::process::Command::new(if cfg!(windows) { "explorer" } else { "xdg-open" }).arg(crate::export::dir()).spawn();
}

fn preset_menu(cx: &mut Cx, st: &mut Ui) {
    let presets = crate::presets::all();
    let row = 22.0;
    let h = (presets.len() as f32 * row + 8.0).min(H - 60.0);
    let area = Rect::from_min_size(pos2(12.0, 40.0), vec2(220.0, h));
    let ctx = cx.ui.ctx().clone();
    let mut pick = None;
    let resp = egui::Area::new(egui::Id::new("preset-menu")).fixed_pos(area.min).order(egui::Order::Foreground).show(&ctx, |ui| {
        let pt = ui.painter().clone();
        pt.rect_filled(area.translate(vec2(0.0, 2.0)).expand(1.0), CornerRadius::same(5), Color32::from_black_alpha(90));
        pt.rect_filled(area, CornerRadius::same(4), Color32::from_rgb(0x1c, 0x1c, 0x1e));
        ui.set_min_size(area.size());
        egui::ScrollArea::vertical().max_height(h - 8.0).show(ui, |ui| {
            ui.add_space(4.0);
            for (i, p) in presets.iter().enumerate() {
                let (r, rr) = ui.allocate_exact_size(vec2(220.0, row), Sense::click());
                let on = i == st.preset;
                if rr.hovered() { ui.painter().rect_filled(r.shrink2(vec2(4.0, 1.0)), CornerRadius::same(3), Color32::from_rgb(0x2e, 0x2e, 0x31)); }
                ui.painter().text(pos2(r.left() + 14.0, r.center().y), egui::Align2::LEFT_CENTER, &p.name, if on { semi(12.0) } else { sans(12.0) }, if on { BRIGHT } else { TEXT });
                if rr.clicked() { pick = Some(i); }
            }
        });
    });
    if let Some(i) = pick { load_preset(cx, st, i); st.presets_open = false; }
    else if ctx.input(|i| i.pointer.any_pressed()) && !resp.response.rect.contains(ctx.input(|i| i.pointer.interact_pos()).unwrap_or_default()) && !Rect::from_min_size(pos2(12.0, 8.0), vec2(200.0, 24.0)).contains(ctx.input(|i| i.pointer.interact_pos()).unwrap_or_default()) {
        st.presets_open = false;
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
                crate::roll::roll(crate::roll::Mode::Randomize, &cur, seed, 4, 2)
            } else {
                // mutations stay around the patch they started from until something else changes it
                if st.last_mutation.as_ref().map_or(true, |m| !same(m, &cur)) { st.anchor = Some(cur.clone()); }
                crate::roll::roll(crate::roll::Mode::Mutate, st.anchor.as_ref().unwrap_or(&cur), seed, 4, 2)
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
