//! Reusable GUI widgets for Oryvex

use iced::{
    widget::{button, column, container, row, scrollable, text, text_input, Column, Row},
    Element, Length,
};

/// Messages that widgets can emit
#[derive(Debug, Clone)]
pub enum WidgetMessage {
    Click,
    Input(String),
    Toggle,
    Update,
}

/// A status indicator widget (red/green dot + label)
pub struct StatusIndicator {
    pub label: String,
    pub active: bool,
}

impl StatusIndicator {
    pub fn new(label: impl Into<String>, active: bool) -> Self {
        Self {
            label: label.into(),
            active,
        }
    }

    pub fn view(&self) -> Element<WidgetMessage> {
        let dot = if self.active { "🟢" } else { "🔴" };
        let color = if self.active {
            iced::Color::from_rgb(0.0, 1.0, 0.0)
        } else {
            iced::Color::from_rgb(1.0, 0.0, 0.0)
        };
        
        row![
            text(dot),
            text(&self.label).style(color),
        ]
        .spacing(5)
        .into()
    }
}

/// A configuration row with label and input
pub struct ConfigRow {
    pub label: String,
    pub value: String,
    pub placeholder: String,
    pub width: u16,
}

impl ConfigRow {
    pub fn new(label: impl Into<String>, value: impl Into<String>, placeholder: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            placeholder: placeholder.into(),
            width: 200,
        }
    }

    pub fn with_width(mut self, width: u16) -> Self {
        self.width = width;
        self
    }

    pub fn view(&self) -> Element<WidgetMessage> {
        row![
            text(&self.label).width(Length::Units(120)),
            text_input(&self.placeholder, &self.value)
                .on_input(WidgetMessage::Input)
                .width(Length::Units(self.width)),
        ]
        .spacing(10)
        .align_items(iced::Alignment::Center)
        .into()
    }
}

/// Action buttons (Start/Stop)
pub struct ActionButtons {
    pub start_label: String,
    pub stop_label: String,
    pub is_running: bool,
    pub start_enabled: bool,
    pub stop_enabled: bool,
}

impl ActionButtons {
    pub fn new(start_label: impl Into<String>, stop_label: impl Into<String>, is_running: bool) -> Self {
        Self {
            start_label: start_label.into(),
            stop_label: stop_label.into(),
            is_running,
            start_enabled: true,
            stop_enabled: true,
        }
    }

    pub fn with_enabled(mut self, start_enabled: bool, stop_enabled: bool) -> Self {
        self.start_enabled = start_enabled;
        self.stop_enabled = stop_enabled;
        self
    }

    pub fn view(&self) -> Element<WidgetMessage> {
        let start = button(&self.start_label)
            .on_press(WidgetMessage::Click)
            .style(if self.is_running || !self.start_enabled {
                iced::theme::Button::Secondary
            } else {
                iced::theme::Button::Primary
            });

        let stop = button(&self.stop_label)
            .on_press(WidgetMessage::Click)
            .style(if self.is_running && self.stop_enabled {
                iced::theme::Button::Danger
            } else {
                iced::theme::Button::Secondary
            });

        row![start, stop].spacing(10).into()
    }
}

/// A scrollable log viewer
pub struct LogViewer {
    pub entries: Vec<String>,
    pub max_entries: usize,
    pub auto_scroll: bool,
}

impl LogViewer {
    pub fn new(entries: Vec<String>, max_entries: usize) -> Self {
        Self {
            entries,
            max_entries,
            auto_scroll: true,
        }
    }

    pub fn with_auto_scroll(mut self, auto_scroll: bool) -> Self {
        self.auto_scroll = auto_scroll;
        self
    }

    pub fn add_entry(&mut self, entry: impl Into<String>) {
        self.entries.push(entry.into());
        if self.entries.len() > self.max_entries {
            self.entries.drain(0..self.entries.len() - self.max_entries);
        }
    }

    pub fn view(&self) -> Element<WidgetMessage> {
        let content = self.entries.join("\n");
        
        let log_text = text(content)
            .size(12)
            .style(iced::Color::from_rgb(0.6, 0.6, 0.6));

        // Log viewer with timestamp-like styling
        let log_container = container(
            scrollable(log_text)
                .height(Length::Units(300))
        )
        .padding(10)
        .style(iced::theme::Container::Custom(Box::new(|_theme| {
            iced::widget::container::Appearance {
                background: Some(iced::Color::from_rgb(0.05, 0.05, 0.05).into()),
                text_color: Some(iced::Color::from_rgb(0.8, 0.8, 0.8)),
                border: iced::Border {
                    radius: 4.0.into(),
                    width: 1.0,
                    color: iced::Color::from_rgb(0.2, 0.2, 0.2),
                },
                ..Default::default()
            }
        })));

        log_container.into()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

/// A dropdown selector widget
pub struct Selector {
    pub label: String,
    pub options: Vec<String>,
    pub selected: String,
}

impl Selector {
    pub fn new(label: impl Into<String>, options: Vec<String>, selected: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            options,
            selected: selected.into(),
        }
    }

    pub fn view(&self) -> Element<WidgetMessage> {
        // For now, use a text input as a simple selector
        // In a real implementation, you'd use a proper dropdown
        row![
            text(&self.label).width(Length::Units(100)),
            text_input("Select option", &self.selected)
                .on_input(WidgetMessage::Input)
                .width(Length::Units(150)),
        ]
        .spacing(10)
        .into()
    }
}

/// A toggle switch widget
pub struct Toggle {
    pub label: String,
    pub enabled: bool,
}

impl Toggle {
    pub fn new(label: impl Into<String>, enabled: bool) -> Self {
        Self {
            label: label.into(),
            enabled,
        }
    }

    pub fn view(&self) -> Element<WidgetMessage> {
        let toggle_text = if self.enabled { "✅" } else { "❌" };
        row![
            text(&self.label),
            button(toggle_text)
                .on_press(WidgetMessage::Toggle)
                .style(if self.enabled {
                    iced::theme::Button::Primary
                } else {
                    iced::theme::Button::Secondary
                }),
        ]
        .spacing(10)
        .into()
    }
}

/// A section header with optional toggle
pub struct SectionHeader {
    pub title: String,
    pub subtitle: Option<String>,
    pub collapsible: bool,
    pub collapsed: bool,
}

impl SectionHeader {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            subtitle: None,
            collapsible: false,
            collapsed: false,
        }
    }

    pub fn with_subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    pub fn collapsible(mut self, collapsed: bool) -> Self {
        self.collapsible = true;
        self.collapsed = collapsed;
        self
    }

    pub fn view(&self) -> Element<WidgetMessage> {
        let mut row = Row::new().push(text(&self.title).size(16).bold());
        
        if let Some(sub) = &self.subtitle {
            row = row.push(text(sub).size(12).style(iced::Color::from_rgb(0.5, 0.5, 0.5)));
        }

        if self.collapsible {
            let icon = if self.collapsed { "▶" } else { "▼" };
            row = row.push(button(icon).on_press(WidgetMessage::Toggle));
        }

        row.spacing(10).into()
    }
}

/// A progress indicator
pub struct ProgressIndicator {
    pub label: String,
    pub progress: f32, // 0.0 to 1.0
    pub show_percentage: bool,
}

impl ProgressIndicator {
    pub fn new(label: impl Into<String>, progress: f32) -> Self {
        Self {
            label: label.into(),
            progress: progress.clamp(0.0, 1.0),
            show_percentage: true,
        }
    }

    pub fn view(&self) -> Element<WidgetMessage> {
        let bar_width = 200;
        let filled = (self.progress * bar_width as f32) as usize;
        let empty = bar_width - filled;

        let bar = format!(
            "[{}{}]",
            "=".repeat(filled),
            " ".repeat(empty)
        );

        let percentage = if self.show_percentage {
            format!(" {:.0}%", self.progress * 100.0)
        } else {
            String::new()
        };

        row![
            text(&self.label).width(Length::Units(100)),
            text(bar),
            text(percentage),
        ]
        .spacing(5)
        .into()
    }
}