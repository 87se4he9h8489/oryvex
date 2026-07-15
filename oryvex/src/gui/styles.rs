//! GUI styling and theming

use iced::{Color, Theme};
use iced::widget::button;

/// Available themes for the GUI
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum AppTheme {
    #[default]
    Dark,
    Light,
}

/// Application styles
pub struct AppStyles {
    pub theme: AppTheme,
    pub background: Color,
    pub text: Color,
    pub accent: Color,
    pub accent_hover: Color,
    pub border: Color,
    pub success: Color,
    pub warning: Color,
    pub danger: Color,
    pub muted: Color,
    pub card_background: Color,
    pub input_background: Color,
    pub shadow: Color,
}

impl AppStyles {
    pub fn new(theme: AppTheme) -> Self {
        match theme {
            AppTheme::Dark => Self {
                theme,
                background: Color::from_rgb(0.12, 0.12, 0.12),
                text: Color::from_rgb(0.83, 0.83, 0.83),
                accent: Color::from_rgb(0.0, 0.48, 0.8),
                accent_hover: Color::from_rgb(0.0, 0.58, 0.9),
                border: Color::from_rgb(0.24, 0.24, 0.24),
                success: Color::from_rgb(0.0, 0.8, 0.0),
                warning: Color::from_rgb(0.8, 0.6, 0.0),
                danger: Color::from_rgb(0.8, 0.0, 0.0),
                muted: Color::from_rgb(0.4, 0.4, 0.4),
                card_background: Color::from_rgb(0.16, 0.16, 0.16),
                input_background: Color::from_rgb(0.1, 0.1, 0.1),
                shadow: Color::from_rgb(0.0, 0.0, 0.0),
            },
            AppTheme::Light => Self {
                theme,
                background: Color::from_rgb(0.95, 0.95, 0.95),
                text: Color::from_rgb(0.1, 0.1, 0.1),
                accent: Color::from_rgb(0.0, 0.4, 0.8),
                accent_hover: Color::from_rgb(0.0, 0.5, 0.9),
                border: Color::from_rgb(0.7, 0.7, 0.7),
                success: Color::from_rgb(0.0, 0.6, 0.0),
                warning: Color::from_rgb(0.6, 0.4, 0.0),
                danger: Color::from_rgb(0.6, 0.0, 0.0),
                muted: Color::from_rgb(0.5, 0.5, 0.5),
                card_background: Color::from_rgb(1.0, 1.0, 1.0),
                input_background: Color::from_rgb(0.98, 0.98, 0.98),
                shadow: Color::from_rgb(0.0, 0.0, 0.0).with_alpha(0.1),
            },
        }
    }

    /// Get status color based on state
    pub fn status_color(&self, is_running: bool) -> Color {
        if is_running { self.success } else { self.danger }
    }

    /// Get button style
    pub fn button_style(&self, variant: ButtonVariant) -> iced::theme::Button {
        match variant {
            ButtonVariant::Primary => {
                iced::theme::Button::Custom(Box::new(move |_theme, status| {
                    let base = iced::widget::button::primary(_theme, status);
                    button::Appearance {
                        background: Some(self.accent.into()),
                        text_color: Color::WHITE,
                        ..base
                    }
                }))
            }
            ButtonVariant::Success => {
                iced::theme::Button::Custom(Box::new(move |_theme, status| {
                    let base = iced::widget::button::primary(_theme, status);
                    button::Appearance {
                        background: Some(self.success.into()),
                        text_color: Color::WHITE,
                        ..base
                    }
                }))
            }
            ButtonVariant::Danger => {
                iced::theme::Button::Custom(Box::new(move |_theme, status| {
                    let base = iced::widget::button::primary(_theme, status);
                    button::Appearance {
                        background: Some(self.danger.into()),
                        text_color: Color::WHITE,
                        ..base
                    }
                }))
            }
            ButtonVariant::Secondary => {
                iced::theme::Button::Custom(Box::new(move |_theme, status| {
                    let base = iced::widget::button::secondary(_theme, status);
                    button::Appearance {
                        background: Some(self.card_background.into()),
                        text_color: self.text,
                        ..base
                    }
                }))
            }
            ButtonVariant::Text => {
                iced::theme::Button::Custom(Box::new(move |_theme, status| {
                    let base = iced::widget::button::text(_theme, status);
                    button::Appearance {
                        text_color: self.accent,
                        ..base
                    }
                }))
            }
        }
    }

    /// Get text input style
    pub fn text_input_style(&self) -> iced::theme::TextInput {
        iced::theme::TextInput::Custom(Box::new(move |_theme, status| {
            iced::widget::text_input::Appearance {
                background: self.input_background.into(),
                border: iced::Border {
                    radius: 4.0.into(),
                    width: 1.0,
                    color: self.border,
                },
                icon_color: self.muted,
                placeholder_color: self.muted,
                value_color: self.text,
                selection_color: self.accent.with_alpha(0.3),
            }
        }))
    }

    /// Get container style for cards
    pub fn card_style(&self) -> iced::theme::Container {
        iced::theme::Container::Custom(Box::new(move |_theme| {
            iced::widget::container::Appearance {
                background: Some(self.card_background.into()),
                text_color: Some(self.text),
                border: iced::Border {
                    radius: 8.0.into(),
                    width: 1.0,
                    color: self.border,
                },
                shadow: iced::Shadow {
                    color: self.shadow,
                    offset: iced::Vector::new(0.0, 2.0),
                    blur_radius: 4.0,
                },
                ..Default::default()
            }
        }))
    }

    /// Get container style for the log area
    pub fn log_style(&self) -> iced::theme::Container {
        iced::theme::Container::Custom(Box::new(move |_theme| {
            iced::widget::container::Appearance {
                background: Some(self.background.into()),
                text_color: Some(self.muted),
                border: iced::Border {
                    radius: 4.0.into(),
                    width: 1.0,
                    color: self.border,
                },
                ..Default::default()
            }
        }))
    }

    /// Combine two colors with alpha blending
    pub fn blend(&self, foreground: Color, background: Color, alpha: f32) -> Color {
        let alpha = alpha.clamp(0.0, 1.0);
        Color {
            r: foreground.r * alpha + background.r * (1.0 - alpha),
            g: foreground.g * alpha + background.g * (1.0 - alpha),
            b: foreground.b * alpha + background.b * (1.0 - alpha),
            a: 1.0,
        }
    }
}

/// Button variants
#[derive(Debug, Clone, Copy)]
pub enum ButtonVariant {
    Primary,
    Success,
    Danger,
    Secondary,
    Text,
}

/// Helper to create a styled container
pub fn styled_container<'a, T: 'a + iced::Widget<Message>>(
    child: T,
    styles: &AppStyles,
) -> iced::widget::Container<'a, T> {
    iced::widget::container(child)
        .padding(20)
        .style(styles.card_style())
}

/// Helper to create a styled header
pub fn header_text<'a>(text: &'a str, styles: &AppStyles) -> iced::widget::Text<'a> {
    iced::widget::Text::new(text)
        .size(20)
        .bold()
        .style(styles.accent)
}

/// Helper to create a styled log entry
pub fn log_entry<'a>(text: &'a str, styles: &AppStyles) -> iced::widget::Text<'a> {
    iced::widget::Text::new(text)
        .size(12)
        .style(if text.contains("✅") || text.contains("success") {
            styles.success
        } else if text.contains("❌") || text.contains("error") || text.contains("failed") {
            styles.danger
        } else if text.contains("⚠️") || text.contains("warning") {
            styles.warning
        } else {
            styles.muted
        })
}

/// Predefined color palettes
pub mod palettes {
    use iced::Color;

    pub const BLUE: Color = Color::from_rgb(0.0, 0.48, 0.8);
    pub const GREEN: Color = Color::from_rgb(0.0, 0.8, 0.0);
    pub const RED: Color = Color::from_rgb(0.8, 0.0, 0.0);
    pub const YELLOW: Color = Color::from_rgb(0.8, 0.6, 0.0);
    pub const PURPLE: Color = Color::from_rgb(0.6, 0.0, 0.8);
    pub const CYAN: Color = Color::from_rgb(0.0, 0.8, 0.8);
    pub const ORANGE: Color = Color::from_rgb(0.8, 0.4, 0.0);
    pub const PINK: Color = Color::from_rgb(0.8, 0.0, 0.4);
}

/// Dark theme specific colors
pub mod dark {
    use iced::Color;

    pub const BACKGROUND: Color = Color::from_rgb(0.12, 0.12, 0.12);
    pub const CARD: Color = Color::from_rgb(0.16, 0.16, 0.16);
    pub const TEXT: Color = Color::from_rgb(0.83, 0.83, 0.83);
    pub const MUTED: Color = Color::from_rgb(0.5, 0.5, 0.5);
    pub const BORDER: Color = Color::from_rgb(0.24, 0.24, 0.24);
    pub const ACCENT: Color = Color::from_rgb(0.0, 0.48, 0.8);
    pub const INPUT: Color = Color::from_rgb(0.1, 0.1, 0.1);
}

/// Light theme specific colors
pub mod light {
    use iced::Color;

    pub const BACKGROUND: Color = Color::from_rgb(0.95, 0.95, 0.95);
    pub const CARD: Color = Color::from_rgb(1.0, 1.0, 1.0);
    pub const TEXT: Color = Color::from_rgb(0.1, 0.1, 0.1);
    pub const MUTED: Color = Color::from_rgb(0.5, 0.5, 0.5);
    pub const BORDER: Color = Color::from_rgb(0.7, 0.7, 0.7);
    pub const ACCENT: Color = Color::from_rgb(0.0, 0.4, 0.8);
    pub const INPUT: Color = Color::from_rgb(0.98, 0.98, 0.98);
}