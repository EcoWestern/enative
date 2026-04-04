//! Layer shell integration for Wayland.
//!
//! A layer shell is a Wayland surface that is rendered at a specific layer
//! in the compositor's stack. This is commonly used for shell elements like
//! panels, notifications, and wallpapers.
use crate::application::{BootFn, UpdateFn, ViewFn};

use crate::program::{self, Program};
use crate::graphics::compositor;
use crate::{Element, Result, Settings, Task, theme};

pub use enative_layershell::actions::LayershellCustomActions;
pub use enative_layershell::reexport::{Anchor, Layer};
pub use enative_layershell::settings::LayerShellSettings as LayerSettings;

/// Creates an enative [`Application`] for a layer shell.
pub fn application<State, Message, Theme, Renderer>(
    boot: impl BootFn<State, Message>,
    update: impl UpdateFn<State, Message>,
    view: impl for<'a> ViewFn<'a, State, Message, Theme, Renderer>,
) -> Application<impl Program<State = State, Message = Message, Theme = Theme, Renderer = Renderer>>
where
    State: 'static,
    Message: Send + 'static,
    Theme: theme::Base,
    Renderer: crate::program::Renderer,
{
    use std::marker::PhantomData;

    struct Instance<State, Message, Theme, Renderer, Boot, Update, View> {
        boot: Boot,
        update: Update,
        view: View,
        _marker: PhantomData<(State, Message, Theme, Renderer)>,
    }

    impl<State, Message, Theme, Renderer, Boot, Update, View> Program
        for Instance<State, Message, Theme, Renderer, Boot, Update, View>
    where
        Message: Send + 'static,
        Theme: theme::Base,
        Renderer: crate::program::Renderer,
        Boot: BootFn<State, Message>,
        Update: UpdateFn<State, Message>,
        View: for<'a> ViewFn<'a, State, Message, Theme, Renderer>,
    {
        type State = State;
        type Message = Message;
        type Theme = Theme;
        type Renderer = Renderer;
        type Executor = enative_futures::backend::default::Executor;

        fn name() -> &'static str {
            let name = std::any::type_name::<State>();
            name.split("::").next().unwrap_or("a_cool_application")
        }

        fn boot(&self) -> (State, Task<Message>) {
            self.boot.boot()
        }

        fn update(&self, state: &mut Self::State, message: Self::Message) -> Task<Self::Message> {
            self.update.update(state, message)
        }

        fn view<'a>(
            &self,
            state: &'a Self::State,
            _window: crate::core::window::Id,
        ) -> Element<'a, Self::Message, Self::Theme, Self::Renderer> {
            self.view.view(state)
        }

        fn settings(&self) -> Settings {
            Settings::default()
        }

        fn window(&self) -> Option<crate::core::window::Settings> {
            None
        }
    }

    Application {
        raw: Instance {
            boot,
            update,
            view,
            _marker: PhantomData,
        },
        settings: Settings::default(),
        layer_settings: LayerSettings::default(),
    }
}

/// An enative application running in a layer shell.
pub struct Application<P: Program> {
    raw: P,
    settings: Settings,
    layer_settings: LayerSettings,
}

impl<P: Program> Application<P> {
    /// Sets the [`Layer`] of the [`Application`].
    pub fn layer(mut self, layer: Layer) -> Self {
        self.layer_settings.layer = layer;
        self
    }

    /// Sets the [`Anchor`] of the [`Application`].
    pub fn anchor(mut self, anchor: Anchor) -> Self {
        self.layer_settings.anchor = anchor;
        self
    }

    /// Sets the exclusive zone of the [`Application`].
    pub fn exclusive_zone(mut self, exclusive_zone: i32) -> Self {
        self.layer_settings.exclusive_zone = exclusive_zone;
        self
    }

    /// Sets the subscription logic of the [`Application`].
    pub fn subscription(
        self,
        f: impl Fn(&P::State) -> crate::Subscription<P::Message>,
    ) -> Application<impl Program<State = P::State, Message = P::Message, Theme = P::Theme, Renderer = P::Renderer>> {
        Application {
            raw: program::with_subscription(self.raw, f),
            settings: self.settings,
            layer_settings: self.layer_settings,
        }
    }

    /// Sets the theme logic of the [`Application`].
    pub fn theme(
        self,
        f: impl crate::application::ThemeFn<P::State, P::Theme>,
    ) -> Application<impl Program<State = P::State, Message = P::Message, Theme = P::Theme, Renderer = P::Renderer>> {
        Application {
            raw: program::with_theme(self.raw, move |state, _window| f.theme(state)),
            settings: self.settings,
            layer_settings: self.layer_settings,
        }
    }

    /// Sets the settings of the [`Application`].
    pub fn settings(mut self, settings: Settings) -> Self {
        self.settings = settings;
        self
    }

    /// Runs the [`Application`].
    pub fn run(self) -> Result
    where
        Self: 'static,
        P::Theme: Default,
        P::Message: crate::message::MaybeDebug
            + crate::message::MaybeClone
            + TryInto<
                enative_layershell::actions::LayershellCustomActions,
                Error = P::Message,
            >,
        P::Renderer: compositor::Default,
    {
        use enative_layershell::settings::Settings as LayershellSettings;

        Ok(enative_layershell::run::<
            P,
            <P::Renderer as compositor::Default>::Compositor,
        >(
            self.raw,
            LayershellSettings {
                layer_settings: self.layer_settings,
                ..Default::default()
            },
        )?)
    }
}
