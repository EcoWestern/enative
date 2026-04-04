use crate::core::{Element, Layout, Length, Rectangle, Size, Widget};
use crate::core::layout;
use crate::core::mouse;
use crate::core::renderer;
use crate::core::widget::Tree;
use std::sync::{Arc, Mutex};

/// A handle that can be used to update the underlying Wayland surfaces
pub trait NativeViewHandle: Send + Sync {
    /// Positions the native view at the given coordinates.
    fn position(&mut self, x: i32, y: i32);
    /// Resizes the native view to the given dimensions.
    fn resize(&mut self, w: u32, h: u32);
}

/// A widget that punches a transparent hole in the UI and synchronizes
/// its bounds with a native Wayland subsurface.
pub struct NativeView {
    handle: Arc<Mutex<dyn NativeViewHandle>>,
    width: Length,
    height: Length,
}

impl NativeView {
    /// Creates a new [`NativeView`] with the given [`NativeViewHandle`].
    pub fn new(handle: Arc<Mutex<dyn NativeViewHandle>>) -> Self {
        Self {
            handle,
            width: Length::Fill,
            height: Length::Fill,
        }
    }

    /// Sets the width of the [`NativeView`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`NativeView`].
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }
}

/// Creates a new [`NativeView`] with the given [`NativeViewHandle`].
pub fn native_view(handle: Arc<Mutex<dyn NativeViewHandle>>) -> NativeView {
    NativeView::new(handle)
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for NativeView
where
    Renderer: crate::core::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        // Layout the widget based on the limits
        layout::atomic(limits, self.width, self.height)
    }

    fn draw(
        &self,
        _tree: &Tree,
        _renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        // We do *not* render anything to WGPU, acting as a transparent hole.
        // We update the underlying Wayland subsurface to match our layout bounds.
        let bounds = layout.bounds();
        let mut handle = self.handle.lock().unwrap();

        // Update to new bounds
        handle.position(bounds.x as i32, bounds.y as i32);
        handle.resize(bounds.width as u32, bounds.height as u32);
    }
}

impl<'a, Message, Theme, Renderer> From<NativeView> for Element<'a, Message, Theme, Renderer>
where
    Renderer: crate::core::Renderer + 'a,
    Message: 'a,
{
    fn from(view: NativeView) -> Element<'a, Message, Theme, Renderer> {
        Element::new(view)
    }
}
