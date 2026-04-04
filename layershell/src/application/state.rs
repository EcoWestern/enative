use enative_core::{mouse, Color, Point, Size, window};
use enative_graphics::Viewport;
use enative_program::Program;
use layershellev::keyboard::ModifiersState;

use crate::event::WindowEvent;

pub struct State<P: Program> {
    window_id: window::Id,
    viewport: Viewport,
    viewport_version: usize,
    theme: P::Theme,
    background_color: Color,
    text_color: Color,
    mouse_position: Option<Point>,
    modifiers: ModifiersState,
}

impl<P: Program> State<P>
where
    P::Theme: Default,
{
    pub fn new(
        program: &P,
        program_state: &P::State,
        window: &layershellev::WindowStateSimple,
        window_id: window::Id,
    ) -> Self {
        let scale_factor = program.scale_factor(program_state, window_id);
        let theme = program
            .theme(program_state, window_id)
            .unwrap_or_default();
        let style = program.style(program_state, &theme);

        let viewport = {
            let (width, height) = window.main_window().get_size();
            Viewport::with_physical_size(Size::new(width, height), scale_factor)
        };

        Self {
            window_id,
            viewport,
            viewport_version: 0,
            background_color: style.background_color,
            text_color: style.text_color,
            theme,
            mouse_position: None,
            modifiers: ModifiersState::default(),
        }
    }

    pub fn window_id(&self) -> window::Id {
        self.window_id
    }

    pub fn modifiers(&self) -> ModifiersState {
        self.modifiers
    }

    pub fn update_view_port(&mut self, width: u32, height: u32) {
        self.viewport = Viewport::with_physical_size(
            Size::new(width, height),
            self.viewport.scale_factor(),
        );
    }

    pub fn viewport(&self) -> &Viewport {
        &self.viewport
    }

    pub fn physical_size(&self) -> Size<u32> {
        self.viewport.physical_size()
    }

    pub fn logical_size(&self) -> Size<f32> {
        self.viewport.logical_size()
    }

    pub fn text_color(&self) -> Color {
        self.text_color
    }

    pub fn background_color(&self) -> Color {
        self.background_color
    }

    pub fn theme(&self) -> &P::Theme {
        &self.theme
    }

    pub fn cursor(&self) -> mouse::Cursor {
        self.mouse_position
            .map(mouse::Cursor::Available)
            .unwrap_or(mouse::Cursor::Unavailable)
    }

    pub fn update(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::CursorLeft => {
                self.mouse_position = None;
            }
            WindowEvent::CursorMoved { x, y } => {
                self.mouse_position = Some(Point::new(*x as f32, *y as f32));
            }
            WindowEvent::ModifiersChanged(modifiers) => {
                self.modifiers = *modifiers;
            }
            _ => {}
        }
    }

    pub fn synchronize(&mut self, program: &P, program_state: &P::State) {
        self.theme = program
            .theme(program_state, self.window_id)
            .unwrap_or_default();
        let style = program.style(program_state, &self.theme);
        self.background_color = style.background_color;
        self.text_color = style.text_color;
    }
}
