//! Draw and interact with text.
mod rich;

pub use crate::core::text::{Fragment, Highlighter, IntoFragment, Span};
pub use crate::core::widget::text::*;
pub use rich::Rich;

/// A bunch of text.
///
/// # Example
/// ```no_run
/// # mod enative { pub mod widget { pub use enative_widget::*; } pub use enative_widget::Renderer; pub use enative_widget::core::*; }
/// # pub type State = ();
/// # pub type Element<'a, Message> = enative_widget::core::Element<'a, Message, enative_widget::Theme, enative_widget::Renderer>;
/// use enative::widget::text;
/// use enative::color;
///
/// enum Message {
///     // ...
/// }
///
/// fn view(state: &State) -> Element<'_, Message> {
///     text("Hello, this is enative!")
///         .size(20)
///         .color(color!(0x0000ff))
///         .into()
/// }
/// ```
pub type Text<'a, Theme = crate::Theme, Renderer = crate::Renderer> =
    crate::core::widget::Text<'a, Theme, Renderer>;

/// Creates a new [`Text`] widget with the provided content.
pub fn text<'a, Theme, Renderer>(
    content: impl crate::core::text::IntoFragment<'a>,
) -> Text<'a, Theme, Renderer>
where
    Theme: crate::core::widget::text::Catalog,
    Renderer: crate::core::text::Renderer,
{
    Text::new(content)
}

/// Creates a new [`Text`] widget with the [`Preset::H1`] size.
pub fn h1<'a, Theme, Renderer>(
    content: impl crate::core::text::IntoFragment<'a>,
) -> Text<'a, Theme, Renderer>
where
    Theme: crate::core::widget::text::Catalog + crate::core::theme::Base + 'a,
    Renderer: crate::core::text::Renderer,
{
    text(content).size(crate::core::text::typography::Preset::H1)
}

/// Creates a new [`Text`] widget with the [`Preset::H2`] size.
pub fn h2<'a, Theme, Renderer>(
    content: impl crate::core::text::IntoFragment<'a>,
) -> Text<'a, Theme, Renderer>
where
    Theme: crate::core::widget::text::Catalog + crate::core::theme::Base + 'a,
    Renderer: crate::core::text::Renderer,
{
    text(content).size(crate::core::text::typography::Preset::H2)
}

/// Creates a new [`Text`] widget with the [`Preset::H3`] size.
pub fn h3<'a, Theme, Renderer>(
    content: impl crate::core::text::IntoFragment<'a>,
) -> Text<'a, Theme, Renderer>
where
    Theme: crate::core::widget::text::Catalog + crate::core::theme::Base + 'a,
    Renderer: crate::core::text::Renderer,
{
    text(content).size(crate::core::text::typography::Preset::H3)
}

/// Creates a new [`Text`] widget with the [`Preset::Body`] size.
pub fn body<'a, Theme, Renderer>(
    content: impl crate::core::text::IntoFragment<'a>,
) -> Text<'a, Theme, Renderer>
where
    Theme: crate::core::widget::text::Catalog + crate::core::theme::Base + 'a,
    Renderer: crate::core::text::Renderer,
{
    text(content).size(crate::core::text::typography::Preset::Body)
}

/// Creates a new [`Text`] widget with the [`Preset::Label`] size.
pub fn label<'a, Theme, Renderer>(
    content: impl crate::core::text::IntoFragment<'a>,
) -> Text<'a, Theme, Renderer>
where
    Theme: crate::core::widget::text::Catalog + crate::core::theme::Base + 'a,
    Renderer: crate::core::text::Renderer,
{
    text(content).size(crate::core::text::typography::Preset::Label)
}
