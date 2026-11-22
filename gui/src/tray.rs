//! System-tray item (freedesktop/KDE StatusNotifierItem, via the `ksni` crate).
//!
//! The tray runs on its own thread (ksni's blocking API spawns one). Its menu items and a left
//! click can't touch GTK directly from that thread, so they only drop a [`Command`] into an
//! `async_channel`; the GTK main loop drains it (see `window::Ui::listen_tray`). The icon and the
//! Connect/Disconnect menu entries are refreshed from the GTK side through [`Tray::set`].

use ksni::blocking::{Handle, TrayMethods};

/// What a tray menu entry (or a left click) asks the window to do, handled on the GTK main loop.
#[derive(Clone, Copy, Debug)]
pub enum Command {
    /// Show the window if hidden, hide it if shown (also the left-click action).
    ToggleWindow,
    Connect,
    Disconnect,
    OpenPhone,
    OpenApps,
    OpenDisplay,
    OpenFiles,
    OpenNotifications,
    Pair,
    /// Quit the app *and* stop the daemon (the only thing that does).
    Quit,
}

/// The coarse state the icon reflects.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    /// `smartconnectd` is not answering its control socket.
    DaemonDown,
    /// The daemon was just launched and the socket hasn't answered yet.
    Starting,
    Disconnected,
    Connected,
}

/// Rich phone telemetry displayed on tray hover tooltip.
#[derive(Clone, Debug, Default)]
pub struct PhoneTelemetry {
    pub name: String,
    pub status: String,
    pub battery: Option<i64>,
    pub charging: bool,
    pub wifi_word: String,
    pub cell_word: String,
    pub bluetooth: bool,
}

/// The actual StatusNotifierItem. Lives on the ksni thread; the GTK side only reaches it through
/// the [`Handle`] in [`Tray`].
struct Item {
    state: State,
    /// Drives the Show/Hide label.
    window_visible: bool,
    telemetry: Option<PhoneTelemetry>,
    tx: async_channel::Sender<Command>,
}

impl Item {
    fn send(&self, cmd: Command) {
        // Unbounded channel: `try_send` never blocks (only fails if the UI is gone), which is what
        // we want from the tray thread.
        let _ = self.tx.try_send(cmd);
    }
}

impl ksni::Tray for Item {
    fn id(&self) -> String {
        "dev.smartconnect.Linux".into()
    }

    fn title(&self) -> String {
        "Smart Connect".into()
    }

    fn icon_name(&self) -> String {
        match self.state {
            State::DaemonDown => "network-offline-symbolic",
            State::Starting => "content-loading-symbolic",
            State::Disconnected | State::Connected => "phone-symbolic",
        }
        .into()
    }

    /// A small check badge on the icon when the phone is connected (hosts that don't draw overlays
    /// just ignore it; the tooltip still says the state).
    fn overlay_icon_name(&self) -> String {
        match self.state {
            State::Connected => "emblem-ok-symbolic".into(),
            _ => String::new(),
        }
    }

    fn tool_tip(&self) -> ksni::ToolTip {
        let title = match &self.telemetry {
            Some(t) if !t.name.is_empty() => format!("Smart Connect · {}", t.name),
            _ => "Smart Connect for Linux".to_string(),
        };

        let description = if let Some(t) = &self.telemetry {
            let mut lines = Vec::new();
            lines.push(format!("Status: {}", t.status));
            if let Some(b) = t.battery {
                if t.charging {
                    lines.push(format!("Battery: {}% (Charging)", b));
                } else {
                    lines.push(format!("Battery: {}%", b));
                }
            }
            lines.push(format!("Wi-Fi: {} · Cellular: {}", t.wifi_word, t.cell_word));
            lines.push(format!("Bluetooth: {}", if t.bluetooth { "Active" } else { "Off" }));
            lines.join("\n")
        } else {
            match self.state {
                State::DaemonDown => "Daemon not running".into(),
                State::Starting => "Starting…".into(),
                State::Disconnected => "Phone disconnected · Ready to connect".into(),
                State::Connected => "Phone connected".into(),
            }
        };

        ksni::ToolTip {
            icon_name: self.icon_name(),
            icon_pixmap: Vec::new(),
            title,
            description,
        }
    }

    /// Left click: toggle the window.
    fn activate(&mut self, _x: i32, _y: i32) {
        self.send(Command::ToggleWindow);
    }

    fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
        use ksni::menu::{MenuItem, StandardItem};
        let mut items = Vec::new();

        items.push(
            StandardItem {
                label: if self.window_visible { "Hide Smart Connect".into() } else { "Open Smart Connect".into() },
                activate: Box::new(|this: &mut Self| this.send(Command::ToggleWindow)),
                ..Default::default()
            }
            .into(),
        );

        items.push(MenuItem::Separator);

        items.push(
            StandardItem {
                label: "Phone Overview".into(),
                icon_name: "phone-symbolic".into(),
                activate: Box::new(|this: &mut Self| this.send(Command::OpenPhone)),
                ..Default::default()
            }
            .into(),
        );
        items.push(
            StandardItem {
                label: "Apps Launchpad".into(),
                icon_name: "view-app-grid-symbolic".into(),
                activate: Box::new(|this: &mut Self| this.send(Command::OpenApps)),
                ..Default::default()
            }
            .into(),
        );
        items.push(
            StandardItem {
                label: "Display Studio".into(),
                icon_name: "video-display-symbolic".into(),
                activate: Box::new(|this: &mut Self| this.send(Command::OpenDisplay)),
                ..Default::default()
            }
            .into(),
        );
        items.push(
            StandardItem {
                label: "Files & Storage".into(),
                icon_name: "folder-symbolic".into(),
                activate: Box::new(|this: &mut Self| this.send(Command::OpenFiles)),
                ..Default::default()
            }
            .into(),
        );
        items.push(
            StandardItem {
                label: "Notifications".into(),
                icon_name: "preferences-system-notifications-symbolic".into(),
                activate: Box::new(|this: &mut Self| this.send(Command::OpenNotifications)),
                ..Default::default()
            }
            .into(),
        );

        items.push(MenuItem::Separator);

        if matches!(self.state, State::Connected) {
            items.push(
                StandardItem {
                    label: "Disconnect".into(),
                    icon_name: "network-offline-symbolic".into(),
                    activate: Box::new(|this: &mut Self| this.send(Command::Disconnect)),
                    ..Default::default()
                }
                .into(),
            );
        } else {
            items.push(
                StandardItem {
                    label: "Connect".into(),
                    icon_name: "network-wireless-symbolic".into(),
                    enabled: matches!(self.state, State::Disconnected),
                    activate: Box::new(|this: &mut Self| this.send(Command::Connect)),
                    ..Default::default()
                }
                .into(),
            );
        }

        items.push(
            StandardItem {
                label: "Pair New Phone…".into(),
                icon_name: "list-add-symbolic".into(),
                activate: Box::new(|this: &mut Self| this.send(Command::Pair)),
                ..Default::default()
            }
            .into(),
        );

        items.push(MenuItem::Separator);

        items.push(
            StandardItem {
                label: "Quit Smart Connect".into(),
                icon_name: "application-exit-symbolic".into(),
                activate: Box::new(|this: &mut Self| this.send(Command::Quit)),
                ..Default::default()
            }
            .into(),
        );

        items
    }
}

/// A spawned tray item. Dropping it tears the tray down.
pub struct Tray {
    handle: Handle<Item>,
}

impl Tray {
    /// Spawns the tray on its own thread. Returns the handle and the menu-command receiver, or
    /// `None` when the desktop has no StatusNotifierHost (then the app just runs without a tray).
    pub fn spawn() -> Option<(Tray, async_channel::Receiver<Command>)> {
        let (tx, rx) = async_channel::unbounded();
        let item = Item {
            state: State::Starting,
            window_visible: true,
            telemetry: None,
            tx,
        };
        match item.spawn() {
            Ok(handle) => Some((Tray { handle }, rx)),
            Err(e) => {
                eprintln!("smartconnect-gui: system tray unavailable: {e}");
                None
            }
        }
    }

    /// Pushes the current connection state, window visibility, and phone telemetry to the tray.
    pub fn set(&self, state: State, window_visible: bool, telemetry: Option<PhoneTelemetry>) {
        self.handle.update(|item| {
            item.state = state;
            item.window_visible = window_visible;
            item.telemetry = telemetry;
        });
    }
}
