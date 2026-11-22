# Notification Actions & Background Execution

Smart Connect for Linux allows you to interact with notifications received from your phone directly from your desktop.

---

## Supported Notification Actions

| Action Button | Desktop Behavior | Execution Details |
|---|---|---|
| **Direct Reply** | Opens an inline text reply entry (in the desktop notification, GTK app, or quick-reply popup). | The message is sent back to the phone as a direct notification inline reply packet. |
| **Action Buttons** (e.g. *Mark as read*, *Archive*, *Pause*) | Executed immediately on the phone without opening an intrusive window on your desktop. | Handled via an ephemeral background app stream. See below. |
| **Open App on Phone** | Launches the corresponding phone app in a native desktop window. | Initiates a standard `smartconnect-mirror --app` stream. |
| **Mute App** | Stops desktop alerts from that application on this PC. | Stored in local configuration (`~/.config/smartconnect-linux/prefs.json`). The phone continues to notify normally. |

---

## How Remote Action Buttons Work

Android notifications often include interactive buttons defined by the application (for example, marking an email as read in Gmail, archiving a message in WhatsApp, or pausing a timer).

When connected to a PC companion, the phone executes these actions by briefly hosting the app on a virtual display. Smart Connect for Linux handles this automatically behind the scenes:

1. When you click an action button on your desktop, `smartconnectd` requests an app-streaming session for that action from the phone.
2. The headless stream host (`smartconnect-mirror --host`) answers the WebRTC session invisibly without creating a desktop window.
3. The phone renders the app to its virtual display and dispatches the action intent.
4. As soon as the phone confirms completion (typically ~2.5 seconds), the session terminates cleanly.

### Configuration & Debugging

If you wish to view the visual execution on the phone's virtual display for debugging purposes:

```bash
# Set this environment variable before starting the daemon or UI
export SMARTCONNECT_BACKGROUND_ACTIONS=0
```

Log output for action execution is recorded in:
```
$XDG_RUNTIME_DIR/smartconnectd/smartconnect-mirror.log
```
