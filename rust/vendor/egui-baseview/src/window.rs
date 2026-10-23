use std::time::Instant;

use baseview::{
    Event, EventStatus, PhySize, Window, WindowHandle, WindowHandler, WindowOpenOptions,
    WindowScalePolicy,
};
use copypasta::ClipboardProvider;
use egui::{pos2, vec2, Pos2, Rect, Rgba, ViewportCommand};
use keyboard_types::Modifiers;
use raw_window_handle::HasRawWindowHandle;

use crate::{renderer::Renderer, GraphicsConfig};

pub struct Queue<'a> {
    bg_color: &'a mut Rgba,
    close_requested: &'a mut bool,
    physical_size: &'a mut PhySize,
}

impl<'a> Queue<'a> {
    pub(crate) fn new(
        bg_color: &'a mut Rgba,
        close_requested: &'a mut bool,
        physical_size: &'a mut PhySize,
    ) -> Self {
        Self {
            bg_color,
            //renderer,
            //repaint_requested,
            close_requested,
            physical_size,
        }
    }

    /// Set the background color.
    pub fn bg_color(&mut self, bg_color: Rgba) {
        *self.bg_color = bg_color;
    }

    /// Set size of the window.
    pub fn resize(&mut self, physical_size: PhySize) {
        *self.physical_size = physical_size;
    }

    /// Close the window.
    pub fn close_window(&mut self) {
        *self.close_requested = true;
    }
}

struct OpenSettings {
    scale_policy: WindowScalePolicy,
    logical_width: f64,
    logical_height: f64,
    title: String,
}

impl OpenSettings {
    fn new(settings: &WindowOpenOptions) -> Self {
        // WindowScalePolicy does not implement copy/clone.
        let scale_policy = match &settings.scale {
            WindowScalePolicy::SystemScaleFactor => WindowScalePolicy::SystemScaleFactor,
            WindowScalePolicy::ScaleFactor(scale) => WindowScalePolicy::ScaleFactor(*scale),
        };

        Self {
            scale_policy,
            logical_width: settings.size.width,
            logical_height: settings.size.height,
            title: settings.title.clone(),
        }
    }
}

/// Handles an egui-baseview application
pub struct EguiWindow<State, U>
where
    State: 'static + Send,
    U: FnMut(&egui::Context, &mut Queue, &mut State),
    U: 'static + Send,
{
    user_state: Option<State>,
    user_update: U,

    egui_ctx: egui::Context,
    /// a text field holds the keyboard it took from the host
    borrowed_keys: bool,
    viewport_id: egui::ViewportId,
    start_time: Instant,
    egui_input: egui::RawInput,
    pointer_pos_in_points: Option<egui::Pos2>,
    current_cursor_icon: baseview::MouseCursor,

    renderer: Renderer,

    clipboard_ctx: Option<copypasta::ClipboardContext>,

    physical_size: PhySize,
    scale_policy: WindowScalePolicy,
    pixels_per_point: f32,
    points_per_pixel: f32,
    bg_color: Rgba,
    close_requested: bool,
    repaint_after: Option<Instant>,
}

impl<State, U> EguiWindow<State, U>
where
    State: 'static + Send,
    U: FnMut(&egui::Context, &mut Queue, &mut State),
    U: 'static + Send,
{
    fn new<B>(
        window: &mut baseview::Window<'_>,
        open_settings: OpenSettings,
        graphics_config: GraphicsConfig,
        mut build: B,
        update: U,
        mut state: State,
    ) -> EguiWindow<State, U>
    where
        B: FnMut(&egui::Context, &mut Queue, &mut State),
        B: 'static + Send,
    {
        let renderer = Renderer::new(window, graphics_config).unwrap_or_else(|err| {
            // TODO: better error log and not panicking, but that's gonna require baseview changes
            log::error!("oops! the gpu backend couldn't initialize! \n {err}");
            panic!("gpu backend failed to initialize: \n {err}")
        });
        let egui_ctx = egui::Context::default();

        // Assume scale for now until there is an event with a new one.
        let pixels_per_point = match open_settings.scale_policy {
            WindowScalePolicy::ScaleFactor(scale) => scale,
            WindowScalePolicy::SystemScaleFactor => 1.0,
        } as f32;
        let points_per_pixel = pixels_per_point.recip();

        let screen_rect = Rect::from_min_size(
            Pos2::new(0f32, 0f32),
            vec2(
                open_settings.logical_width as f32,
                open_settings.logical_height as f32,
            ),
        );

        let viewport_info = egui::ViewportInfo {
            parent: None,
            title: Some(open_settings.title),
            native_pixels_per_point: Some(pixels_per_point),
            focused: Some(true),
            inner_rect: Some(screen_rect),
            ..Default::default()
        };
        let viewport_id = egui::ViewportId::default();

        let mut egui_input = egui::RawInput {
            max_texture_side: Some(renderer.max_texture_side()),
            screen_rect: Some(screen_rect),
            ..Default::default()
        };
        let _ = egui_input.viewports.insert(viewport_id, viewport_info);

        let mut physical_size = PhySize {
            width: (open_settings.logical_width * pixels_per_point as f64).round() as u32,
            height: (open_settings.logical_height * pixels_per_point as f64).round() as u32,
        };

        let mut bg_color = Rgba::BLACK;
        let mut close_requested = false;
        let mut queue = Queue::new(&mut bg_color, &mut close_requested, &mut physical_size);
        (build)(&egui_ctx, &mut queue, &mut state);

        let clipboard_ctx = match copypasta::ClipboardContext::new() {
            Ok(clipboard_ctx) => Some(clipboard_ctx),
            Err(e) => {
                log::error!("Failed to initialize clipboard: {}", e);
                None
            }
        };

        let start_time = Instant::now();

        Self {
            user_state: Some(state),
            user_update: update,

            egui_ctx,
            borrowed_keys: false,
            viewport_id,
            start_time,
            egui_input,
            pointer_pos_in_points: None,
            current_cursor_icon: baseview::MouseCursor::Default,

            renderer,

            clipboard_ctx,

            physical_size,
            pixels_per_point,
            points_per_pixel,
            scale_policy: open_settings.scale_policy,
            bg_color,
            close_requested,
            repaint_after: Some(start_time),
        }
    }

    /// Open a new child window.
    ///
    /// * `parent` - The parent window.
    /// * `settings` - The settings of the window.
    /// * `state` - The initial state of your application.
    /// * `build` - Called once before the first frame. Allows you to do setup code and to
    /// call `ctx.set_fonts()`. Optional.
    /// * `update` - Called before each frame. Here you should update the state of your
    /// application and build the UI.
    pub fn open_parented<P, B>(
        parent: &P,
        #[allow(unused_mut)] mut settings: WindowOpenOptions,
        graphics_config: GraphicsConfig,
        state: State,
        build: B,
        update: U,
    ) -> WindowHandle
    where
        P: HasRawWindowHandle,
        B: FnMut(&egui::Context, &mut Queue, &mut State),
        B: 'static + Send,
    {
        #[cfg(feature = "opengl")]
        if settings.gl_config.is_none() {
            settings.gl_config = Some(Default::default());
        }

        let open_settings = OpenSettings::new(&settings);

        Window::open_parented(
            parent,
            settings,
            move |window: &mut baseview::Window<'_>| -> EguiWindow<State, U> {
                EguiWindow::new(window, open_settings, graphics_config, build, update, state)
            },
        )
    }

    /// Open a new window that blocks the current thread until the window is destroyed.
    ///
    /// * `settings` - The settings of the window.
    /// * `state` - The initial state of your application.
    /// * `build` - Called once before the first frame. Allows you to do setup code and to
    /// call `ctx.set_fonts()`. Optional.
    /// * `update` - Called before each frame. Here you should update the state of your
    /// application and build the UI.
    pub fn open_blocking<B>(
        #[allow(unused_mut)] mut settings: WindowOpenOptions,
        graphics_config: GraphicsConfig,
        state: State,
        build: B,
        update: U,
    ) where
        B: FnMut(&egui::Context, &mut Queue, &mut State),
        B: 'static + Send,
    {
        #[cfg(feature = "opengl")]
        if settings.gl_config.is_none() {
            settings.gl_config = Some(Default::default());
        }

        let open_settings = OpenSettings::new(&settings);

        Window::open_blocking(
            settings,
            move |window: &mut baseview::Window<'_>| -> EguiWindow<State, U> {
                EguiWindow::new(window, open_settings, graphics_config, build, update, state)
            },
        )
    }

    /// Update the pressed key modifiers when a mouse event has sent a new set of modifiers.
    fn update_modifiers(&mut self, modifiers: &Modifiers) {
        self.egui_input.modifiers.alt = !(*modifiers & Modifiers::ALT).is_empty();
        self.egui_input.modifiers.shift = !(*modifiers & Modifiers::SHIFT).is_empty();
        self.egui_input.modifiers.command = !(*modifiers & Modifiers::CONTROL).is_empty();
    }
}

impl<State, U> WindowHandler for EguiWindow<State, U>
where
    State: 'static + Send,
    U: FnMut(&egui::Context, &mut Queue, &mut State),
    U: 'static + Send,
{
    fn on_frame(&mut self, window: &mut Window) {
        let Some(state) = &mut self.user_state else {
            return;
        };

        self.egui_input.time = Some(self.start_time.elapsed().as_secs_f64());
        // egui's zoom factor scales the whole UI on top of the display's own scale
        self.egui_input.screen_rect = Some(calculate_screen_rect(
            self.physical_size,
            self.points_per_pixel / self.egui_ctx.zoom_factor(),
        ));

        self.egui_ctx.begin_pass(self.egui_input.take());

        //let mut repaint_requested = false;
        let mut queue = Queue::new(
            &mut self.bg_color,
            &mut self.close_requested,
            &mut self.physical_size,
        );

        (self.user_update)(&self.egui_ctx, &mut queue, state);

        if self.close_requested {
            window.close();
        }

        // Prevent data from being allocated every frame by storing this
        // in a member field.
        let mut full_output = self.egui_ctx.end_pass();

        let Some(viewport_output) = full_output.viewport_output.get(&self.viewport_id) else {
            // The main window was closed by egui.
            window.close();
            return;
        };

        for command in viewport_output.commands.iter() {
            match command {
                ViewportCommand::Close => {
                    window.close();
                }
                ViewportCommand::InnerSize(size) => window.resize(baseview::Size {
                    width: size.x.max(1.0) as f64,
                    height: size.y.max(1.0) as f64,
                }),
                _ => {}
            }
        }

        let now = Instant::now();
        let do_repaint_now = if let Some(t) = self.repaint_after {
            now >= t || viewport_output.repaint_delay.is_zero()
        } else {
            viewport_output.repaint_delay.is_zero()
        };

        if do_repaint_now {
            self.renderer.render(
                #[cfg(feature = "opengl")]
                window,
                self.bg_color,
                self.physical_size,
                full_output.pixels_per_point,
                &mut self.egui_ctx,
                &mut full_output,
            );

            self.repaint_after = None;
        } else if let Some(repaint_after) = now.checked_add(viewport_output.repaint_delay) {
            // Schedule to repaint after the requested time has elapsed.
            self.repaint_after = Some(repaint_after);
        }

        for command in full_output.platform_output.commands {
            match command {
                egui::OutputCommand::CopyText(text) => {
                    if let Some(clipboard_ctx) = &mut self.clipboard_ctx {
                        if let Err(err) = clipboard_ctx.set_contents(text) {
                            log::error!("Copy/Cut error: {}", err);
                        }
                    }
                }
                egui::OutputCommand::CopyImage(_) => {
                    log::warn!("Copying images is not supported in egui_baseview.");
                }
                egui::OutputCommand::OpenUrl(open_url) => {
                    if let Err(err) = open::that_detached(&open_url.url) {
                        log::error!("Open error: {}", err);
                    }
                }
            }
        }

        let egui_cursor = full_output.platform_output.cursor_icon;
        let cursor_icon = crate::translate::translate_cursor_icon(egui_cursor);
        // AppKit resets the cursor as the pointer moves, so a non-arrow cursor is set again every frame
        #[cfg(target_os = "macos")]
        if self.current_cursor_icon != cursor_icon || egui_cursor != egui::CursorIcon::Default {
            crate::translate::set_macos_cursor(egui_cursor);
        }
        if self.current_cursor_icon != cursor_icon {
            self.current_cursor_icon = cursor_icon;
            #[cfg(not(target_os = "macos"))]
            window.set_mouse_cursor(cursor_icon);
        }

        // a focused text field borrows the keyboard from the host, and hands it back when editing ends
        #[cfg(not(target_os = "macos"))]
        {
            let typing = self.egui_ctx.wants_keyboard_input();
            if typing && !window.has_focus() {
                window.focus();
            } else if !typing && self.borrowed_keys {
                window.release_focus();
            }
            if typing != self.borrowed_keys {
                window.capture_keys(typing);
            }
            self.borrowed_keys = typing;
        }

        // A temporary workaround for keyboard input not working sometimes in Windows.
        // See https://github.com/BillyDM/egui-baseview/issues/20
        #[cfg(feature = "windows_keyboard_workaround")]
        {
            #[cfg(target_os = "windows")]
            {
                if !full_output.platform_output.events.is_empty()
                    || full_output.platform_output.ime.is_some()
                {
                    window.focus();
                }
            }
        }
    }

    fn on_event(&mut self, _window: &mut Window, event: Event) -> EventStatus {
        let mut pass_key = false;
        match &event {
            baseview::Event::Mouse(event) => match event {
                baseview::MouseEvent::CursorMoved {
                    position,
                    modifiers,
                } => {
                    self.update_modifiers(modifiers);

                    let z = self.egui_ctx.zoom_factor();
                    let pos = pos2(position.x as f32 / z, position.y as f32 / z);
                    self.pointer_pos_in_points = Some(pos);
                    self.egui_input.events.push(egui::Event::PointerMoved(pos));
                }
                baseview::MouseEvent::ButtonPressed { button, modifiers } => {
                    self.update_modifiers(modifiers);

                    if let Some(pos) = self.pointer_pos_in_points {
                        if let Some(button) = crate::translate::translate_mouse_button(*button) {
                            self.egui_input.events.push(egui::Event::PointerButton {
                                pos,
                                button,
                                pressed: true,
                                modifiers: self.egui_input.modifiers,
                            });
                        }
                    }
                }
                baseview::MouseEvent::ButtonReleased { button, modifiers } => {
                    self.update_modifiers(modifiers);

                    if let Some(pos) = self.pointer_pos_in_points {
                        if let Some(button) = crate::translate::translate_mouse_button(*button) {
                            self.egui_input.events.push(egui::Event::PointerButton {
                                pos,
                                button,
                                pressed: false,
                                modifiers: self.egui_input.modifiers,
                            });
                        }
                    }
                }
                baseview::MouseEvent::WheelScrolled {
                    delta: scroll_delta,
                    modifiers,
                } => {
                    self.update_modifiers(modifiers);

                    #[allow(unused_mut)]
                    let (unit, mut delta) = match scroll_delta {
                        baseview::ScrollDelta::Lines { x, y } => {
                            (egui::MouseWheelUnit::Line, egui::vec2(*x, *y))
                        }

                        baseview::ScrollDelta::Pixels { x, y } => (
                            egui::MouseWheelUnit::Point,
                            egui::vec2(*x, *y) * self.points_per_pixel / self.egui_ctx.zoom_factor(),
                        ),
                    };

                    if cfg!(target_os = "macos") {
                        // This is still buggy in winit despite
                        // https://github.com/rust-windowing/winit/issues/1695 being closed
                        //
                        // TODO: See if this is an issue in baseview as well.
                        delta.x *= -1.0;
                    }

                    self.egui_input.events.push(egui::Event::MouseWheel {
                        unit,
                        delta,
                        modifiers: self.egui_input.modifiers,
                    });
                }
                baseview::MouseEvent::CursorLeft => {
                    self.pointer_pos_in_points = None;
                    self.egui_input.events.push(egui::Event::PointerGone);
                }
                // files dragged over the window arrive as egui's hovered and dropped files
                baseview::MouseEvent::DragEntered { position, data, .. } | baseview::MouseEvent::DragMoved { position, data, .. } => {
                    let z = self.egui_ctx.zoom_factor();
                    let pos = pos2(position.x as f32 / z, position.y as f32 / z);
                    self.pointer_pos_in_points = Some(pos);
                    self.egui_input.events.push(egui::Event::PointerMoved(pos));
                    let baseview::DropData::Files(files) = data else { return EventStatus::Ignored };
                    self.egui_input.hovered_files = files.iter().map(|f| egui::HoveredFile { path: Some(f.clone()), ..Default::default() }).collect();
                    return EventStatus::AcceptDrop(baseview::DropEffect::Copy);
                }
                baseview::MouseEvent::DragLeft => {
                    self.egui_input.hovered_files.clear();
                    self.pointer_pos_in_points = None;
                    self.egui_input.events.push(egui::Event::PointerGone);
                }
                baseview::MouseEvent::DragDropped { position, data, .. } => {
                    self.egui_input.hovered_files.clear();
                    let z = self.egui_ctx.zoom_factor();
                    let pos = pos2(position.x as f32 / z, position.y as f32 / z);
                    self.pointer_pos_in_points = Some(pos);
                    self.egui_input.events.push(egui::Event::PointerMoved(pos));
                    let baseview::DropData::Files(files) = data else { return EventStatus::Ignored };
                    self.egui_input.dropped_files.extend(files.iter().map(|f| egui::DroppedFile { path: Some(f.clone()), ..Default::default() }));
                    return EventStatus::AcceptDrop(baseview::DropEffect::Copy);
                }
                _ => {}
            },
            baseview::Event::Keyboard(event) => {
                use keyboard_types::Code;
                // keys go on to the host, so its computer keyboard keeps playing, unless a text field is being typed in
                pass_key = !self.egui_ctx.wants_keyboard_input() && !crate::KEEP_KEYS.load(std::sync::atomic::Ordering::Relaxed);

                let pressed = event.state == keyboard_types::KeyState::Down;

                match event.code {
                    Code::ShiftLeft | Code::ShiftRight => self.egui_input.modifiers.shift = pressed,
                    Code::ControlLeft | Code::ControlRight => {
                        self.egui_input.modifiers.ctrl = pressed;

                        #[cfg(not(target_os = "macos"))]
                        {
                            self.egui_input.modifiers.command = pressed;
                        }
                    }
                    Code::AltLeft | Code::AltRight => self.egui_input.modifiers.alt = pressed,
                    Code::MetaLeft | Code::MetaRight => {
                        #[cfg(target_os = "macos")]
                        {
                            self.egui_input.modifiers.mac_cmd = pressed;
                            self.egui_input.modifiers.command = pressed;
                        }
                        // prevent `rustfmt` from breaking this
                    }
                    _ => (),
                }

                // the physical key is the key's position on the keyboard, whatever caps lock or the layout make it type
                let physical_key = crate::translate::translate_physical_key(event.code);
                if let Some(key) = crate::translate::translate_virtual_key(&event.key).or(physical_key) {
                    self.egui_input.events.push(egui::Event::Key {
                        key,
                        physical_key,
                        pressed,
                        repeat: event.repeat,
                        modifiers: self.egui_input.modifiers,
                    });
                }

                if pressed {
                    // VirtualKeyCode::Paste etc in winit are broken/untrustworthy,
                    // so we detect these things manually:
                    //
                    // TODO: See if this is an issue in baseview as well.
                    if is_cut_command(self.egui_input.modifiers, event.code) {
                        self.egui_input.events.push(egui::Event::Cut);
                    } else if is_copy_command(self.egui_input.modifiers, event.code) {
                        self.egui_input.events.push(egui::Event::Copy);
                    } else if is_paste_command(self.egui_input.modifiers, event.code) {
                        if let Some(clipboard_ctx) = &mut self.clipboard_ctx {
                            match clipboard_ctx.get_contents() {
                                Ok(contents) => {
                                    self.egui_input.events.push(egui::Event::Text(contents))
                                }
                                Err(err) => {
                                    log::error!("Paste error: {}", err);
                                }
                            }
                        }
                    } else if let keyboard_types::Key::Character(written) = &event.key {
                        if !self.egui_input.modifiers.ctrl && !self.egui_input.modifiers.command {
                            self.egui_input
                                .events
                                .push(egui::Event::Text(written.clone()));
                        }
                    }
                }
            }
            baseview::Event::Window(event) => match event {
                baseview::WindowEvent::Resized(window_info) => {
                    self.pixels_per_point = match self.scale_policy {
                        WindowScalePolicy::ScaleFactor(scale) => scale,
                        WindowScalePolicy::SystemScaleFactor => window_info.scale(),
                    } as f32;
                    self.points_per_pixel = self.pixels_per_point.recip();

                    self.physical_size = window_info.physical_size();

                    let screen_rect = calculate_screen_rect(
                        self.physical_size,
                        self.points_per_pixel / self.egui_ctx.zoom_factor(),
                    );

                    self.egui_input.screen_rect = Some(screen_rect);

                    let viewport_info = self
                        .egui_input
                        .viewports
                        .get_mut(&self.viewport_id)
                        .unwrap();
                    viewport_info.native_pixels_per_point = Some(self.pixels_per_point);
                    viewport_info.inner_rect = Some(screen_rect);

                    // Schedule to repaint on the next frame.
                    self.repaint_after = Some(Instant::now());
                }
                baseview::WindowEvent::Focused => {
                    self.egui_input
                        .events
                        .push(egui::Event::WindowFocused(true));
                    self.egui_input
                        .viewports
                        .get_mut(&self.viewport_id)
                        .unwrap()
                        .focused = Some(true);
                }
                baseview::WindowEvent::Unfocused => {
                    self.egui_input
                        .events
                        .push(egui::Event::WindowFocused(false));
                    self.egui_input
                        .viewports
                        .get_mut(&self.viewport_id)
                        .unwrap()
                        .focused = Some(false);
                }
                baseview::WindowEvent::WillClose => {}
            },
        }

        if pass_key { EventStatus::Ignored } else { EventStatus::Captured }
    }
}

fn is_cut_command(modifiers: egui::Modifiers, keycode: keyboard_types::Code) -> bool {
    (modifiers.command && keycode == keyboard_types::Code::KeyX)
        || (cfg!(target_os = "windows")
            && modifiers.shift
            && keycode == keyboard_types::Code::Delete)
}

fn is_copy_command(modifiers: egui::Modifiers, keycode: keyboard_types::Code) -> bool {
    (modifiers.command && keycode == keyboard_types::Code::KeyC)
        || (cfg!(target_os = "windows")
            && modifiers.ctrl
            && keycode == keyboard_types::Code::Insert)
}

fn is_paste_command(modifiers: egui::Modifiers, keycode: keyboard_types::Code) -> bool {
    (modifiers.command && keycode == keyboard_types::Code::KeyV)
        || (cfg!(target_os = "windows")
            && modifiers.shift
            && keycode == keyboard_types::Code::Insert)
}

/// Calculate screen rectangle in logical size.
fn calculate_screen_rect(physical_size: PhySize, points_per_pixel: f32) -> Rect {
    let logical_size = (
        physical_size.width as f32 * points_per_pixel,
        physical_size.height as f32 * points_per_pixel,
    );
    Rect::from_min_size(Pos2::new(0f32, 0f32), vec2(logical_size.0, logical_size.1))
}
