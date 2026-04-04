use std::ffi::c_void;
use wayland_client::{
    protocol::{
        wl_compositor::WlCompositor, wl_registry, wl_subcompositor::WlSubcompositor,
        wl_subsurface::WlSubsurface, wl_surface::WlSurface,
    },
    Connection, Dispatch, Proxy, QueueHandle,
};

/// A handle to a native view.
///
/// This is used to embed external surfaces into an eNative application.
pub struct NativeView {
    _connection: Connection,
    surface: WlSurface,
    subsurface: WlSubsurface,
    position: (i32, i32),
    size: (u32, u32),
}

impl NativeView {
    /// Creates a new [`NativeView`].
    #[allow(unsafe_code)]
    pub fn new(display_ptr: *mut c_void, parent_ptr: *mut c_void) -> Self {
        let backend = unsafe {
            wayland_client::backend::Backend::from_foreign_display(display_ptr as *mut _)
        };
        let connection = Connection::from_backend(backend);

        let mut event_queue = connection.new_event_queue();
        let qh = event_queue.handle();

        let mut state = NativeViewState::default();
        let _registry = connection.display().get_registry(&qh, ());

        // Roundtrip to get globals
        let _ = event_queue
            .blocking_dispatch(&mut state)
            .expect("Failed to dispatch registry events");

        // Roundtrip to bind compositor and subcompositor
        let _ = event_queue
            .blocking_dispatch(&mut state)
            .expect("Failed to bind Wayland globals");

        let compositor = state.compositor.expect("wl_compositor global not found");
        let subcompositor = state
            .subcompositor
            .expect("wl_subcompositor global not found");

        let surface = compositor.create_surface(&qh, ());

        // We wrap the raw parent pointer.
        // NOTE: This assumes the parent pointer belongs to the same display and
        // that the underlying backend can handle cross-connection proxies if applicable,
        // though typically this should be called with pointers already managed by
        // a shared connection.
        let parent_id = unsafe {
            wayland_client::backend::ObjectId::from_ptr(
                &WlSurface::interface(),
                parent_ptr as *mut _,
            )
            .expect("Failed to create ObjectId from parent pointer")
        };

        let parent =
            WlSurface::from_id(&connection, parent_id).expect("Failed to create WlSurface from ID");

        let subsurface = subcompositor.get_subsurface(&surface, &parent, &qh, ());

        Self {
            _connection: connection,
            surface,
            subsurface,
            position: (0, 0),
            size: (0, 0),
        }
    }

    /// Sets the position of the [`NativeView`].
    pub fn position(&mut self, x: i32, y: i32) {
        if self.position != (x, y) {
            self.position = (x, y);
            self.subsurface.set_position(x, y);
        }
    }

    /// Resizes the [`NativeView`].
    pub fn resize(&mut self, w: u32, h: u32) {
        if self.size != (w, h) {
            self.size = (w, h);
            // In Wayland, the subsurface size is determined by the buffer attached to the surface.
        }
    }

    /// Returns the raw surface handle of the [`NativeView`].
    #[allow(unsafe_code)]
    pub fn surface_handle(&self) -> *mut c_void {
        self.surface.id().as_ptr() as *mut c_void
    }

    /// Commits the [`NativeView`].
    pub fn commit(&self) {
        self.surface.commit();
    }
}

impl crate::widget::native_view::NativeViewHandle for NativeView {
    fn position(&mut self, x: i32, y: i32) {
        self.position(x, y);
    }

    fn resize(&mut self, w: u32, h: u32) {
        self.resize(w, h);
    }
}

impl Drop for NativeView {
    fn drop(&mut self) {
        self.subsurface.destroy();
        self.surface.destroy();
    }
}

#[derive(Default)]
struct NativeViewState {
    compositor: Option<WlCompositor>,
    subcompositor: Option<WlSubcompositor>,
}

impl Dispatch<wl_registry::WlRegistry, ()> for NativeViewState {
    fn event(
        state: &mut Self,
        registry: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        _data: &(),
        _conn: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        if let wl_registry::Event::Global {
            name,
            interface,
            version,
        } = event
        {
            match &interface[..] {
                "wl_compositor" => {
                    state.compositor = Some(registry.bind::<WlCompositor, _, _>(
                        name,
                        version,
                        qh,
                        (),
                    ));
                }
                "wl_subcompositor" => {
                    state.subcompositor = Some(registry.bind::<WlSubcompositor, _, _>(
                        name,
                        version,
                        qh,
                        (),
                    ));
                }
                _ => {}
            }
        }
    }
}

impl Dispatch<WlCompositor, ()> for NativeViewState {
    fn event(
        _state: &mut Self,
        _proxy: &WlCompositor,
        _event: <WlCompositor as Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<WlSubcompositor, ()> for NativeViewState {
    fn event(
        _state: &mut Self,
        _proxy: &WlSubcompositor,
        _event: <WlSubcompositor as Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<WlSurface, ()> for NativeViewState {
    fn event(
        _state: &mut Self,
        _proxy: &WlSurface,
        _event: <WlSurface as Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<WlSubsurface, ()> for NativeViewState {
    fn event(
        _state: &mut Self,
        _proxy: &WlSubsurface,
        _event: <WlSubsurface as Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
    }
}
