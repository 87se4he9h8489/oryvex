//! Main application state and logic for Oryvex GUI

use iced::{
    widget::{button, column, container, row, scrollable, text, text_input, Column, Row, Space},
    Application, Command, Element, Length, Theme,
};
use std::sync::Arc;
use tokio::sync::Mutex;

use super::widgets::{WidgetMessage, StatusIndicator, ConfigRow, ActionButtons, LogViewer};
use super::styles::{AppTheme, AppStyles};
use super::themed_container;

/// Main GUI application state
#[derive(Default)]
pub struct OryvexGui {
    // Connection state
    pub status: String,
    pub is_running: bool,
    pub logs: Vec<String>,
    
    // Configuration
    pub protocol: String,
    pub socks_port: String,
    pub noize_profile: String,
    pub scan_mode: String,
    pub ip_version: String,
    
    // UI state
    pub theme: AppTheme,
    pub styles: AppStyles,
    pub show_advanced: bool,
}

/// Messages that can be sent to the GUI
#[derive(Debug, Clone)]
pub enum Message {
    // Actions
    Start,
    Stop,
    ToggleAdvanced,
    
    // Configuration changes
    ProtocolChanged(String),
    PortChanged(String),
    NoizeChanged(String),
    ScanModeChanged(String),
    IpVersionChanged(String),
    
    // UI updates
    ThemeChanged(AppTheme),
    LogUpdated(String),
    Tick,
}

impl Application for OryvexGui {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let gui = OryvexGui {
            status: "Stopped".to_string(),
            is_running: false,
            logs: vec![
                "Oryvex GUI v1.0.0".to_string(),
                "Ready to connect".to_string(),
            ],
            protocol: "masque".to_string(),
            socks_port: "1819".to_string(),
            noize_profile: "firewall".to_string(),
            scan_mode: "balanced".to_string(),
            ip_version: "ipv4".to_string(),
            theme: AppTheme::Dark,
            styles: AppStyles::new(AppTheme::Dark),
            show_advanced: false,
        };
        (gui, Command::none())
    }

    fn title(&self) -> String {
        format!("Oryvex - {}", self.status)
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Start => {
                self.status = "Connecting...".to_string();
                self.is_running = true;
                self.add_log(format!("Starting tunnel with protocol: {}", self.protocol));
                self.add_log(format!("SOCKS5 proxy listening on 127.0.0.1:{}", self.socks_port));
                self.add_log(format!("Noize profile: {}", self.noize_profile));
                
                // TODO: Actually start the tunnel here
                // This would spawn a tokio task with the tunnel logic
                // For now, simulate success
                self.status = "Running".to_string();
                self.add_log("Tunnel established successfully ✅".to_string());
            }
            
            Message::Stop => {
                self.status = "Stopping...".to_string();
                self.is_running = false;
                self.add_log("Stopping tunnel...".to_string());
                // TODO: Actually stop the tunnel here
                self.status = "Stopped".to_string();
                self.add_log("Tunnel stopped".to_string());
            }
            
            Message::ProtocolChanged(p) => {
                self.protocol = p;
                self.add_log(format!("Protocol changed to: {}", self.protocol));
            }
            
            Message::PortChanged(p) => {
                if let Ok(port) = p.parse::<u16>() {
                    self.socks_port = p;
                    self.add_log(format!("SOCKS port changed to: {}", port));
                } else if p.is_empty() {
                    self.socks_port = p;
                }
            }
            
            Message::NoizeChanged(n) => {
                self.noize_profile = n;
                self.add_log(format!("Noize profile changed to: {}", n));
            }
            
            Message::ScanModeChanged(s) => {
                self.scan_mode = s;
                self.add_log(format!("Scan mode changed to: {}", s));
            }
            
            Message::IpVersionChanged(ip) => {
                self.ip_version = ip;
                self.add_log(format!("IP version changed to: {}", ip));
            }
            
            Message::ThemeChanged(theme) => {
                self.theme = theme;
                self.styles = AppStyles::new(theme);
                self.add_log(format!("Theme changed to: {:?}", theme));
            }
            
            Message::ToggleAdvanced => {
                self.show_advanced = !self.show_advanced;
                self.add_log(if self.show_advanced {
                    "Showing advanced settings".to_string()
                } else {
                    "Hiding advanced settings".to_string()
                });
            }
            
            Message::LogUpdated(msg) => {
                self.add_log(msg);
            }
            
            Message::Tick => {
                // Periodic updates
                // Could check tunnel status here
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let styles = &self.styles;
        
        // Status section
        let status_indicator = StatusIndicator::new(&self.status, self.is_running);
        let status_view = status_indicator.view().map(|_| Message::Tick);
        
        // Controls section
        let start_button = button("▶ Start")
            .on_press(Message::Start)
            .style(if self.is_running {
                iced::theme::Button::Secondary
            } else {
                iced::theme::Button::Primary
            });
            
        let stop_button = button("■ Stop")
            .on_press(Message::Stop)
            .style(if self.is_running {
                iced::theme::Button::Danger
            } else {
                iced::theme::Button::Secondary
            });

        // Configuration section
        let protocol_input = text_input("Protocol", &self.protocol)
            .on_input(Message::ProtocolChanged)
            .placeholder("masque / wg / gool")
            .style(iced::theme::TextInput::default());
            
        let port_input = text_input("SOCKS Port", &self.socks_port)
            .on_input(Message::PortChanged)
            .placeholder("1819")
            .style(iced::theme::TextInput::default());
            
        let noize_input = text_input("Noize Profile", &self.noize_profile)
            .on_input(Message::NoizeChanged)
            .placeholder("firewall / gfw / off")
            .style(iced::theme::TextInput::default());

        let controls = row![
            start_button,
            stop_button,
            Space::with_width(Length::Units(20)),
            protocol_input,
            port_input,
            noize_input,
        ]
        .spacing(10)
        .padding(10);

        // Advanced settings (togglable)
        let advanced_settings = if self.show_advanced {
            let scan_input = text_input("Scan Mode", &self.scan_mode)
                .on_input(Message::ScanModeChanged)
                .placeholder("turbo / balanced / thorough / stealth");
                
            let ip_input = text_input("IP Version", &self.ip_version)
                .on_input(Message::IpVersionChanged)
                .placeholder("ipv4 / ipv6 / both");

            let theme_button = button("Toggle Theme")
                .on_press(Message::ThemeChanged(match self.theme {
                    AppTheme::Dark => AppTheme::Light,
                    AppTheme::Light => AppTheme::Dark,
                }));

            Some(
                row![scan_input, ip_input, theme_button]
                    .spacing(10)
                    .padding(10)
            )
        } else {
            None
        };

        let toggle_button = button(if self.show_advanced {
            "▲ Hide Advanced"
        } else {
            "▼ Show Advanced"
        })
        .on_press(Message::ToggleAdvanced)
        .style(iced::theme::Button::Text);

        // Log section
        let log_viewer = LogViewer::new(self.logs.clone(), 100);
        let log_view = log_viewer.view().map(Message::LogUpdated);

        // Assemble the main layout
        let mut content = Column::new()
            .push(status_view)
            .push(controls)
            .push(toggle_button)
            .padding(10)
            .spacing(10);

        if let Some(advanced) = advanced_settings {
            content = content.push(advanced);
        }

        content = content.push(log_view);

        // Apply styling
        let themed = themed_container(content, styles)
            .width(Length::Fill)
            .height(Length::Fill);

        themed.into()
    }

    fn theme(&self) -> Theme {
        match self.theme {
            AppTheme::Dark => Theme::Dark,
            AppTheme::Light => Theme::Light,
        }
    }
}

impl OryvexGui {
    /// Add a log message
    pub fn add_log(&mut self, msg: String) {
        self.logs.push(msg);
        if self.logs.len() > 1000 {
            self.logs.drain(0..self.logs.len() - 1000);
        }
    }

    /// Get the current status
    pub fn get_status(&self) -> &str {
        &self.status
    }

    /// Check if the tunnel is running
    pub fn is_running(&self) -> bool {
        self.is_running
    }

    /// Get the SOCKS port
    pub fn get_port(&self) -> u16 {
        self.socks_port.parse().unwrap_or(1819)
    }

    /// Get the protocol
    pub fn get_protocol(&self) -> &str {
        &self.protocol
    }

    /// Get the noize profile
    pub fn get_noize_profile(&self) -> &str {
        &self.noize_profile
    }
}