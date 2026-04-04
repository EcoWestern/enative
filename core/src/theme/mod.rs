//! The built-in themes for enative.
pub mod palette;

use crate::Color;
use palette::Palette;
use std::sync::Arc;

/// The intensity level of the [MateFluency] theme.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Intensity {
    /// Minimal effects, optimized for efficiency.
    Light,
    /// Balanced visual depth with moderate effects.
    #[default]
    Casual,
    /// Maximum layering, texture, and translucency.
    Deep,
}

/// A built-in theme.
#[derive(Debug, Clone, PartialEq)]
pub enum Theme {
    /// The built-in light variant.
    Light,
    /// The built-in dark variant.
    Dark,
    /// The built-in Dracula variant.
    ///
    /// [Dracula]: https://draculatheme.com
    Dracula,
    /// The built-in Nord variant.
    ///
    /// [Nord]: https://www.nordtheme.com/docs/colors-and-palettes
    Nord,
    /// The built-in Solarized Light variant.
    ///
    /// [Solarized]: https://ethanschoonover.com/solarized
    SolarizedLight,
    /// The built-in Solarized Dark variant.
    ///
    /// [Solarized]: https://ethanschoonover.com/solarized
    SolarizedDark,
    /// The built-in Gruvbox Light variant.
    ///
    /// [Gruvbox]: https://github.com/morhetz/gruvbox
    GruvboxLight,
    /// The built-in Gruvbox Dark variant.
    ///
    /// [Gruvbox]: https://github.com/morhetz/gruvbox
    GruvboxDark,
    /// The built-in Catppuccin Latte variant.
    ///
    /// [Catppuccin]: https://github.com/catppuccin/catppuccin
    CatppuccinLatte,
    /// The built-in Catppuccin Frappé variant.
    ///
    /// [Catppuccin]: https://github.com/catppuccin/catppuccin
    CatppuccinFrappe,
    /// The built-in Catppuccin Macchiato variant.
    ///
    /// [Catppuccin]: https://github.com/catppuccin/catppuccin
    CatppuccinMacchiato,
    /// The built-in Catppuccin Mocha variant.
    ///
    /// [Catppuccin]: https://github.com/catppuccin/catppuccin
    CatppuccinMocha,
    /// The built-in Tokyo Night variant.
    ///
    /// [Tokyo Night]: https://github.com/enkia/tokyo-night-vscode-theme
    TokyoNight,
    /// The built-in Tokyo Night Storm variant.
    ///
    /// [Tokyo Night]: https://github.com/enkia/tokyo-night-vscode-theme
    TokyoNightStorm,
    /// The built-in Tokyo Night Light variant.
    ///
    /// [Tokyo Night]: https://github.com/enkia/tokyo-night-vscode-theme
    TokyoNightLight,
    /// The built-in Kanagawa Wave variant.
    ///
    /// [Kanagawa]: https://github.com/rebelot/kanagawa.nvim
    KanagawaWave,
    /// The built-in Kanagawa Dragon variant.
    ///
    /// [Kanagawa]: https://github.com/rebelot/kanagawa.nvim
    KanagawaDragon,
    /// The built-in Kanagawa Lotus variant.
    ///
    /// [Kanagawa]: https://github.com/rebelot/kanagawa.nvim
    KanagawaLotus,
    /// The built-in Moonfly variant.
    ///
    /// [Moonfly]: https://github.com/bluz71/vim-moonfly-colors
    Moonfly,
    /// The built-in Nightfly variant.
    ///
    /// [Nightfly]: https://github.com/bluz71/vim-nightfly-colors
    Nightfly,
    /// The built-in Oxocarbon variant.
    Oxocarbon,
    /// The built-in Ferra variant.
    ///
    /// [Ferra]: https://github.com/casperstorm/ferra
    Ferra,
    /// The built-in MateFluency variant.
    MateFluency(Intensity),
    /// The built-in MateFluency Dark variant.
    MateFluencyDark(Intensity),
    /// A [`Theme`] that uses a [`Custom`] palette.
    Custom(Arc<Custom>),
}

impl Theme {
    /// A slice containing all the built-in [`Theme`] variants.
    pub const ALL: &'static [Self] = &[
        Self::Light,
        Self::Dark,
        Self::Dracula,
        Self::Nord,
        Self::SolarizedLight,
        Self::SolarizedDark,
        Self::GruvboxLight,
        Self::GruvboxDark,
        Self::CatppuccinLatte,
        Self::CatppuccinFrappe,
        Self::CatppuccinMacchiato,
        Self::CatppuccinMocha,
        Self::TokyoNight,
        Self::TokyoNightStorm,
        Self::TokyoNightLight,
        Self::KanagawaWave,
        Self::KanagawaDragon,
        Self::KanagawaLotus,
        Self::Moonfly,
        Self::Nightfly,
        Self::Oxocarbon,
        Self::Ferra,
        Self::MateFluency(Intensity::Light),
        Self::MateFluency(Intensity::Casual),
        Self::MateFluency(Intensity::Deep),
        Self::MateFluencyDark(Intensity::Light),
        Self::MateFluencyDark(Intensity::Casual),
        Self::MateFluencyDark(Intensity::Deep),
    ];

    /// Creates a new custom [`Theme`] from the given [`Seed`](palette::Seed).
    pub fn custom(name: impl Into<String>, seed: palette::Seed) -> Self {
        Self::Custom(Arc::new(Custom {
            name: name.into(),
            seed,
            palette: Palette::generate(seed),
        }))
    }

    /// Creates a new custom [`Theme`] from the given [`Palette`].
    pub fn custom_with_fn(
        name: impl Into<String>,
        seed: palette::Seed,
        generate: impl FnOnce(palette::Seed) -> Palette,
    ) -> Self {
        Self::Custom(Arc::new(Custom {
            name: name.into(),
            palette: generate(seed),
            seed,
        }))
    }

    /// Returns the [`Palette`] of the [`Theme`].
    pub fn palette(&self) -> &Palette {
        match self {
            Self::Light => &palette::LIGHT,
            Self::Dark => &palette::DARK,
            Self::Dracula => &palette::DRACULA,
            Self::Nord => &palette::NORD,
            Self::SolarizedLight => &palette::SOLARIZED_LIGHT,
            Self::SolarizedDark => &palette::SOLARIZED_DARK,
            Self::GruvboxLight => &palette::GRUVBOX_LIGHT,
            Self::GruvboxDark => &palette::GRUVBOX_DARK,
            Self::CatppuccinLatte => &palette::CATPPUCCIN_LATTE,
            Self::CatppuccinFrappe => &palette::CATPPUCCIN_FRAPPE,
            Self::CatppuccinMacchiato => &palette::CATPPUCCIN_MACCHIATO,
            Self::CatppuccinMocha => &palette::CATPPUCCIN_MOCHA,
            Self::TokyoNight => &palette::TOKYO_NIGHT,
            Self::TokyoNightStorm => &palette::TOKYO_NIGHT_STORM,
            Self::TokyoNightLight => &palette::TOKYO_NIGHT_LIGHT,
            Self::KanagawaWave => &palette::KANAGAWA_WAVE,
            Self::KanagawaDragon => &palette::KANAGAWA_DRAGON,
            Self::KanagawaLotus => &palette::KANAGAWA_LOTUS,
            Self::Moonfly => &palette::MOONFLY,
            Self::Nightfly => &palette::NIGHTFLY,
            Self::Oxocarbon => &palette::OXOCARBON,
            Self::Ferra => &palette::FERRA,
            Self::MateFluency(_) => &palette::MATE_FLUENCY,
            Self::MateFluencyDark(_) => &palette::MATE_FLUENCY_DARK,
            Self::Custom(custom) => &custom.palette,
        }
    }

    /// Returns the [`Seed`](palette::Seed) of the [`Theme`].
    pub fn seed(&self) -> palette::Seed {
        match self {
            Self::Light => palette::Seed::LIGHT,
            Self::Dark => palette::Seed::DARK,
            Self::Dracula => palette::Seed::DRACULA,
            Self::Nord => palette::Seed::NORD,
            Self::SolarizedLight => palette::Seed::SOLARIZED_LIGHT,
            Self::SolarizedDark => palette::Seed::SOLARIZED_DARK,
            Self::GruvboxLight => palette::Seed::GRUVBOX_LIGHT,
            Self::GruvboxDark => palette::Seed::GRUVBOX_DARK,
            Self::CatppuccinLatte => palette::Seed::CATPPUCCIN_LATTE,
            Self::CatppuccinFrappe => palette::Seed::CATPPUCCIN_FRAPPE,
            Self::CatppuccinMacchiato => palette::Seed::CATPPUCCIN_MACCHIATO,
            Self::CatppuccinMocha => palette::Seed::CATPPUCCIN_MOCHA,
            Self::TokyoNight => palette::Seed::TOKYO_NIGHT,
            Self::TokyoNightStorm => palette::Seed::TOKYO_NIGHT_STORM,
            Self::TokyoNightLight => palette::Seed::TOKYO_NIGHT_LIGHT,
            Self::KanagawaWave => palette::Seed::KANAGAWA_WAVE,
            Self::KanagawaDragon => palette::Seed::KANAGAWA_DRAGON,
            Self::KanagawaLotus => palette::Seed::KANAGAWA_LOTUS,
            Self::Moonfly => palette::Seed::MOONFLY,
            Self::Nightfly => palette::Seed::NIGHTFLY,
            Self::Oxocarbon => palette::Seed::OXOCARBON,
            Self::Ferra => palette::Seed::FERRA,
            Self::MateFluency(_) => palette::Seed::MATE_FLUENCY,
            Self::MateFluencyDark(_) => palette::Seed::MATE_FLUENCY_DARK,
            Self::Custom(custom) => custom.seed,
        }
    }

    /// Returns the default [`Theme`] for the given system preference.
    pub fn default_for(preference: Mode) -> Self {
        match preference {
            Mode::None | Mode::Light => Self::MateFluency(Intensity::default()),
            Mode::Dark => Self::MateFluencyDark(Intensity::default()),
        }
    }

    /// Returns the name of the [`Theme`].
    pub fn name(&self) -> &str {
        match self {
            Self::Light => "Light",
            Self::Dark => "Dark",
            Self::Dracula => "Dracula",
            Self::Nord => "Nord",
            Self::SolarizedLight => "Solarized Light",
            Self::SolarizedDark => "Solarized Dark",
            Self::GruvboxLight => "Gruvbox Light",
            Self::GruvboxDark => "Gruvbox Dark",
            Self::CatppuccinLatte => "Catppuccin Latte",
            Self::CatppuccinFrappe => "Catppuccin Frappé",
            Self::CatppuccinMacchiato => "Catppuccin Macchiato",
            Self::CatppuccinMocha => "Catppuccin Mocha",
            Self::TokyoNight => "Tokyo Night",
            Self::TokyoNightStorm => "Tokyo Night Storm",
            Self::TokyoNightLight => "Tokyo Night Light",
            Self::KanagawaWave => "Kanagawa Wave",
            Self::KanagawaDragon => "Kanagawa Dragon",
            Self::KanagawaLotus => "Kanagawa Lotus",
            Self::Moonfly => "Moonfly",
            Self::Nightfly => "Nightfly",
            Self::Oxocarbon => "Oxocarbon",
            Self::Ferra => "Ferra",
            Self::MateFluency(intensity) => match intensity {
                Intensity::Light => "MateFluency Light",
                Intensity::Casual => "MateFluency",
                Intensity::Deep => "MateFluency Deep",
            },
            Self::MateFluencyDark(intensity) => match intensity {
                Intensity::Light => "MateFluency Dark Light",
                Intensity::Casual => "MateFluency Dark",
                Intensity::Deep => "MateFluency Dark Deep",
            },
            Self::Custom(custom) => &custom.name,
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::MateFluency(Intensity::default())
    }
}

impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// The system color preference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Mode {
    /// No preference expressed.
    #[default]
    None,
    /// Light mode is preferred.
    Light,
    /// Dark mode is preferred.
    Dark,
}

/// Alias for [`Mode`]; kept for ergonomic use.
pub type SystemPreference = Mode;

/// A custom theme.
#[derive(Debug, Clone, PartialEq)]
pub struct Custom {
    name: String,
    palette: Palette,
    seed: palette::Seed,
}

impl Custom {
    /// Creates a new custom [`Theme`] with the given name and [`Seed`](palette::Seed).
    pub fn new(name: impl Into<String>, seed: palette::Seed) -> Self {
        Self {
            name: name.into(),
            palette: Palette::generate(seed),
            seed,
        }
    }

    /// Creates a new custom [`Theme`] with the given name and [`Palette`].
    pub fn with_fn(
        name: impl Into<String>,
        seed: palette::Seed,
        generate: impl FnOnce(palette::Seed) -> Palette,
    ) -> Self {
        Self {
            name: name.into(),
            palette: generate(seed),
            seed,
        }
    }
}

/// The base appearance of a theme.
pub trait Base {
    /// Returns the [`Mode`] of the theme.
    fn mode(&self) -> Mode;

    /// Returns the name of the theme.
    fn name(&self) -> &str;

    /// Returns the default theme for the given [`Mode`].
    fn default(mode: Mode) -> Self;

    /// Returns the standard animation duration for this theme.
    fn animation_duration(&self) -> crate::time::Duration;

    /// Returns the [`palette::Seed`] of the theme, if any.
    fn seed(&self) -> Option<palette::Seed>;

    /// Returns the base [`Style`] of a theme.
    fn base(&self) -> Style;
}

/// The base style of a theme.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// The background [`Color`] of the theme.
    pub background_color: Color,
    /// The text [`Color`] of the theme.
    pub text_color: Color,
}

impl Base for Theme {
    fn mode(&self) -> Mode {
        match self {
            Self::Light
            | Self::SolarizedLight
            | Self::GruvboxLight
            | Self::CatppuccinLatte
            | Self::TokyoNightLight
            | Self::KanagawaLotus
            | Self::MateFluency(_) => Mode::Light,
            Self::Dark
            | Self::Dracula
            | Self::Nord
            | Self::SolarizedDark
            | Self::GruvboxDark
            | Self::CatppuccinFrappe
            | Self::CatppuccinMacchiato
            | Self::CatppuccinMocha
            | Self::TokyoNight
            | Self::TokyoNightStorm
            | Self::KanagawaWave
            | Self::KanagawaDragon
            | Self::Moonfly
            | Self::Nightfly
            | Self::Oxocarbon
            | Self::Ferra
            | Self::MateFluencyDark(_) => Mode::Dark,
            Self::Custom(custom) => {
                if custom.palette.is_dark {
                    Mode::Dark
                } else {
                    Mode::Light
                }
            }
        }
    }

    fn name(&self) -> &str {
        self.name()
    }

    fn default(mode: Mode) -> Self {
        Self::default_for(mode)
    }

    fn animation_duration(&self) -> crate::time::Duration {
        use crate::animation::duration;

        let intensity = match self {
            Self::MateFluency(intensity) => *intensity,
            Self::MateFluencyDark(intensity) => *intensity,
            _ => Intensity::default(),
        };

        duration::standard(intensity)
    }

    fn seed(&self) -> Option<palette::Seed> {
        match self {
            Self::Light => Some(palette::Seed::LIGHT),
            Self::Dark => Some(palette::Seed::DARK),
            Self::Dracula => Some(palette::Seed::DRACULA),
            Self::Nord => Some(palette::Seed::NORD),
            Self::SolarizedLight => Some(palette::Seed::SOLARIZED_LIGHT),
            Self::SolarizedDark => Some(palette::Seed::SOLARIZED_DARK),
            Self::GruvboxLight => Some(palette::Seed::GRUVBOX_LIGHT),
            Self::GruvboxDark => Some(palette::Seed::GRUVBOX_DARK),
            Self::CatppuccinLatte => Some(palette::Seed::CATPPUCCIN_LATTE),
            Self::CatppuccinFrappe => Some(palette::Seed::CATPPUCCIN_FRAPPE),
            Self::CatppuccinMacchiato => Some(palette::Seed::CATPPUCCIN_MACCHIATO),
            Self::CatppuccinMocha => Some(palette::Seed::CATPPUCCIN_MOCHA),
            Self::TokyoNight => Some(palette::Seed::TOKYO_NIGHT),
            Self::TokyoNightStorm => Some(palette::Seed::TOKYO_NIGHT_STORM),
            Self::TokyoNightLight => Some(palette::Seed::TOKYO_NIGHT_LIGHT),
            Self::KanagawaWave => Some(palette::Seed::KANAGAWA_WAVE),
            Self::KanagawaDragon => Some(palette::Seed::KANAGAWA_DRAGON),
            Self::KanagawaLotus => Some(palette::Seed::KANAGAWA_LOTUS),
            Self::Moonfly => Some(palette::Seed::MOONFLY),
            Self::Nightfly => Some(palette::Seed::NIGHTFLY),
            Self::Oxocarbon => Some(palette::Seed::OXOCARBON),
            Self::Ferra => Some(palette::Seed::FERRA),
            Self::MateFluency(_) => Some(palette::Seed::MATE_FLUENCY),
            Self::MateFluencyDark(_) => Some(palette::Seed::MATE_FLUENCY_DARK),
            Self::Custom(_) => None,
        }
    }

    fn base(&self) -> Style {
        let palette = self.palette();

        Style {
            background_color: palette.background.base.color,
            text_color: palette.background.base.text,
        }
    }
}
