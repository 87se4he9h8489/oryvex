//! GUI module for Oryvex - Entry point

pub mod app;
pub mod widgets;
pub mod styles;

pub use app::OryvexGui;
pub use widgets::{WidgetMessage, StatusIndicator, ConfigRow, ActionButtons, LogViewer};
pub use styles::{AppTheme, AppStyles};

use iced::Application;

/// Initialize and run the GUI application
pub fn run_gui() -> iced::Result {
    OryvexGui::run(iced::Settings {
        window: iced::window::Settings {
            size: iced::Size::new(800.0, 600.0),
            min_size: Some(iced::Size::new(600.0, 400.0)),
            ..Default::default()
        },
        ..Default::default()
    })
}

/// Helper to create a themed container
pub fn themed_container<'a, T: 'a + iced::Widget<Message>>(
    child: T,
    styles: &AppStyles,
) -> iced::widget::Container<'a, T> {
    container(child)
        .padding(20)
        .style(iced::theme::Container::Custom(Box::new(move |_theme| {
            iced::widget::container::Appearance {
                background: Some(styles.background.into()),
                text_color: Some(styles.text),
                border: iced::Border {
                    radius: 8.0.into(),
                    width: 1.0,
                    color: styles.border,
                },
                ..Default::default()
            }
        })))
}