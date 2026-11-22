//! Design System Tokens for Smart Connect Desktop Redesign (2026 Premium Dark Standard).
//!
//! Strict Palette:
//!   --bg:        #022B3A (App background)
//!   --text:      #FFFFFF (Primary text)
//!   --accent:    #1F7A8C (Primary action fill, selected states, toggle-on, progress)
//!   --highlight: #BFDBF7 (Links, key values, active icons, focus ring, connected indicator)
//!   --soft:      #E1E5F2 (Secondary text and hairlines)
//!
//! Derived Tokens:
//!   surface-1:      white 4% over bg (grouped rows container)
//!   surface-2:      white 7% (hover, inputs, secondary buttons)
//!   surface-3:      white 11% (pressed, active nav item)
//!   border:         white 8% (decorative dividers)
//!   border-control: soft 50% (input, toggle-off, checkbox boundaries; >= 3:1)
//!   text-muted:     soft 75% (secondary and helper text)
//!   text-subtle:    soft 55% (placeholders and disabled hints)
//!   accent-tint:    accent 18% (selected row or card background)
//!   accent-hover:   accent mixed with 12% bg (darker on hover to maintain >= 4.5:1 contrast)
//!   accent-active:  accent mixed with 20% bg (darker on active/pressed)
//!   danger:         #E06C75 (desaturated red, error & destructive confirmation only)

use gtk::gdk::RGBA;

// ── Colors (Hex strings & RGBA values) ──────────────────────────────────────

pub const COLOR_BG_HEX: &str = "#022B3A";
pub const COLOR_TEXT_HEX: &str = "#FFFFFF";
pub const COLOR_ACCENT_HEX: &str = "#1F7A8C";
pub const COLOR_HIGHLIGHT_HEX: &str = "#BFDBF7";
pub const COLOR_SOFT_HEX: &str = "#E1E5F2";
pub const COLOR_DANGER_HEX: &str = "#E06C75";

pub const COLOR_ACCENT_HOVER_HEX: &str = "#1B7082";
pub const COLOR_ACCENT_ACTIVE_HEX: &str = "#196A7C";

// Flat derived surfaces (for Cairo painting when transparency is not desired)
pub const COLOR_SURFACE_1_HEX: &str = "#0C3342";
pub const COLOR_SURFACE_2_HEX: &str = "#143A48";
pub const COLOR_SURFACE_3_HEX: &str = "#1E4250";
/// surface-overlay = neutral dark surface (#12181F)
pub const COLOR_SURFACE_OVERLAY_HEX: &str = "#12181F";

// RGBA float representations (0.0 .. 1.0) for Cairo and custom drawing routines
pub const RGBA_BG: (f64, f64, f64, f64) = (2.0 / 255.0, 43.0 / 255.0, 58.0 / 255.0, 1.0);
pub const RGBA_TEXT: (f64, f64, f64, f64) = (1.0, 1.0, 1.0, 1.0);
pub const RGBA_ACCENT: (f64, f64, f64, f64) = (31.0 / 255.0, 122.0 / 255.0, 140.0 / 255.0, 1.0);
pub const RGBA_HIGHLIGHT: (f64, f64, f64, f64) = (191.0 / 255.0, 219.0 / 255.0, 247.0 / 255.0, 1.0);
pub const RGBA_SOFT: (f64, f64, f64, f64) = (225.0 / 255.0, 229.0 / 255.0, 242.0 / 255.0, 1.0);
pub const RGBA_DANGER: (f64, f64, f64, f64) = (224.0 / 255.0, 108.0 / 255.0, 117.0 / 255.0, 1.0);

pub const RGBA_SURFACE_1: (f64, f64, f64, f64) = (1.0, 1.0, 1.0, 0.04);
pub const RGBA_SURFACE_2: (f64, f64, f64, f64) = (1.0, 1.0, 1.0, 0.07);
pub const RGBA_SURFACE_3: (f64, f64, f64, f64) = (1.0, 1.0, 1.0, 0.11);
pub const RGBA_SURFACE_OVERLAY: (f64, f64, f64, f64) = (18.0 / 255.0, 24.0 / 255.0, 31.0 / 255.0, 1.0);
pub const RGBA_BORDER: (f64, f64, f64, f64) = (1.0, 1.0, 1.0, 0.08);
pub const RGBA_BORDER_OVERLAY: (f64, f64, f64, f64) = (1.0, 1.0, 1.0, 0.12);
pub const RGBA_BORDER_CONTROL: (f64, f64, f64, f64) = (225.0 / 255.0, 229.0 / 255.0, 242.0 / 255.0, 0.50);
pub const RGBA_TEXT_MUTED: (f64, f64, f64, f64) = (225.0 / 255.0, 229.0 / 255.0, 242.0 / 255.0, 0.75);
pub const RGBA_TEXT_SUBTLE: (f64, f64, f64, f64) = (225.0 / 255.0, 229.0 / 255.0, 242.0 / 255.0, 0.55);
pub const RGBA_ACCENT_TINT: (f64, f64, f64, f64) = (31.0 / 255.0, 122.0 / 255.0, 140.0 / 255.0, 0.18);
pub const RGBA_SCRIM: (f64, f64, f64, f64) = (0.0, 0.0, 0.0, 0.50);

// ── Dialog Sizing Tokens ────────────────────────────────────────────────────

pub const DIALOG_WIDTH_SM: i32 = 440;
pub const DIALOG_WIDTH_MD: i32 = 480;
pub const DIALOG_WIDTH_LG: i32 = 560;
pub const DIALOG_PAD: i32 = 24;
pub const RADIUS_DIALOG: i32 = 16;

pub fn rgba_accent() -> RGBA {
    RGBA::new(31.0 / 255.0, 122.0 / 255.0, 140.0 / 255.0, 1.0)
}
pub fn rgba_highlight() -> RGBA {
    RGBA::new(191.0 / 255.0, 219.0 / 255.0, 247.0 / 255.0, 1.0)
}
pub fn rgba_text() -> RGBA {
    RGBA::new(1.0, 1.0, 1.0, 1.0)
}
pub fn rgba_text_muted() -> RGBA {
    RGBA::new(225.0 / 255.0, 229.0 / 255.0, 242.0 / 255.0, 0.75)
}

// ── Spacing Tokens (4px base scale) ─────────────────────────────────────────

pub const SPACE_1: i32 = 4;
pub const SPACE_2: i32 = 8;
pub const SPACE_3: i32 = 12;
pub const SPACE_4: i32 = 16;
pub const SPACE_5: i32 = 20;
pub const SPACE_6: i32 = 24;
pub const SPACE_8: i32 = 32;
pub const SPACE_10: i32 = 40;
pub const SPACE_12: i32 = 48;

pub const SPACE_1_U: u32 = 4;
pub const SPACE_2_U: u32 = 8;
pub const SPACE_3_U: u32 = 12;
pub const SPACE_4_U: u32 = 16;
pub const SPACE_5_U: u32 = 20;
pub const SPACE_6_U: u32 = 24;
pub const SPACE_8_U: u32 = 32;
pub const SPACE_10_U: u32 = 40;
pub const SPACE_12_U: u32 = 48;

// ── Radii ───────────────────────────────────────────────────────────────────

pub const RADIUS_CONTROL: i32 = 8;
pub const RADIUS_SURFACE: i32 = 12;
pub const RADIUS_AVATAR: i32 = 16;
pub const RADIUS_PILL: i32 = 999;

// ── Sizing Tokens ───────────────────────────────────────────────────────────

pub const HEIGHT_HEADER: i32 = 56;
pub const WIDTH_SIDEBAR: i32 = 240;
pub const WIDTH_SIDEBAR_COLLAPSED: i32 = 64;

pub const HEIGHT_NAV_ITEM: i32 = 40;
pub const HEIGHT_ROW_DEFAULT: i32 = 56;
pub const HEIGHT_ROW_HELPER: i32 = 64;

pub const HEIGHT_BTN_SM: i32 = 32;
pub const HEIGHT_BTN_MD: i32 = 36;
pub const HEIGHT_BTN_DEFAULT: i32 = 40;
pub const MIN_WIDTH_ROW_BTN: i32 = 88;

pub const ICON_TILE_SIZE: i32 = 40;
pub const ICON_SIZE_NAV: i32 = 20;
pub const ICON_SIZE_ACTION: i32 = 16;
pub const ICON_STROKE_WIDTH: f64 = 1.5;

pub const CONTENT_MAX_WIDTH: i32 = 880;
pub const CONTENT_PAD_HORIZ: i32 = 40;
pub const CONTENT_PAD_TOP: i32 = 32;
pub const CONTENT_PAD_BOTTOM: i32 = 48;

pub const WINDOW_MIN_WIDTH: i32 = 960;
pub const WINDOW_MIN_HEIGHT: i32 = 600;

// ── GTK4 CSS Definitions ────────────────────────────────────────────────────

pub const CSS_TOKENS: &str = r#"
:root {
  /* Semantic Brand & Background Tokens */
  --bg: #022B3A;
  --text: #FFFFFF;
  --accent: #1F7A8C;
  --highlight: #BFDBF7;
  --soft: #E1E5F2;
  --danger: #E06C75;

  /* Derived Surface Tokens */
  --surface-1: rgba(255, 255, 255, 0.04);
  --surface-2: rgba(255, 255, 255, 0.07);
  --surface-3: rgba(255, 255, 255, 0.11);
  --surface-overlay: #113846;
  --surface-hover: rgba(255, 255, 255, 0.07);
  --surface-active: rgba(255, 255, 255, 0.11);

  /* Derived Border Tokens */
  --border: rgba(255, 255, 255, 0.08);
  --border-control: rgba(225, 229, 242, 0.50);
  --border-overlay: rgba(255, 255, 255, 0.12);
  --border-subtle: rgba(255, 255, 255, 0.06);

  /* Derived Text Tokens */
  --text-muted: rgba(225, 229, 242, 0.75);
  --text-subtle: rgba(225, 229, 242, 0.55);

  /* Derived Accent States (Hover/Pressed go darker to preserve >= 4.5:1 text contrast) */
  --accent-tint: rgba(31, 122, 140, 0.18);
  --accent-hover: #1B7082;
  --accent-active: #196A7C;

  /* Scrim Token */
  --scrim-bg: rgba(2, 43, 58, 0.72);

  /* Sizing Tokens */
  --header-height: 56px;
  --sidebar-width: 240px;
  --sidebar-collapsed-width: 64px;
  --content-max-width: 880px;

  /* Base Radii */
  --radius-control: 8px;
  --radius-surface: 12px;
  --radius-dialog: 16px;
  --radius-pill: 9999px;

  /* Standard GTK/Libadwaita System Mapping */
  --window-bg-color: #022B3A;
  --window-fg-color: #FFFFFF;
  --view-bg-color: #022B3A;
  --view-fg-color: #FFFFFF;
  --headerbar-bg-color: #022B3A;
  --headerbar-fg-color: #FFFFFF;
  --headerbar-border-color: transparent;
  --card-bg-color: rgba(255, 255, 255, 0.04);
  --card-fg-color: #FFFFFF;
  --dialog-bg-color: #113846;
  --dialog-fg-color: #FFFFFF;
  --popover-bg-color: #113846;
  --popover-fg-color: #FFFFFF;
  --accent-bg-color: #1F7A8C;
  --accent-fg-color: #FFFFFF;
  --accent-color: #1F7A8C;
}
"#;

#[cfg(test)]
mod tests {


    #[test]
    fn verify_contrast_ratios() {
        // Luminance calculation
        fn rel_luminance(r: f64, g: f64, b: f64) -> f64 {
            let adj = |c: f64| if c <= 0.03928 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) };
            0.2126 * adj(r) + 0.7152 * adj(g) + 0.0722 * adj(b)
        }

        fn contrast(l1: f64, l2: f64) -> f64 {
            let lighter = l1.max(l2);
            let darker = l1.min(l2);
            (lighter + 0.05) / (darker + 0.05)
        }

        let l_bg = rel_luminance(2.0 / 255.0, 43.0 / 255.0, 58.0 / 255.0);
        let l_white = rel_luminance(1.0, 1.0, 1.0);
        let l_highlight = rel_luminance(191.0 / 255.0, 219.0 / 255.0, 247.0 / 255.0);
        let l_soft = rel_luminance(225.0 / 255.0, 229.0 / 255.0, 242.0 / 255.0);
        let l_accent = rel_luminance(31.0 / 255.0, 122.0 / 255.0, 140.0 / 255.0);
        let l_accent_hover = rel_luminance(27.0 / 255.0, 112.0 / 255.0, 130.0 / 255.0);

        // Primary text on bg >= 14:1
        assert!(contrast(l_white, l_bg) >= 14.0);
        // Highlight on bg >= 9:1
        assert!(contrast(l_highlight, l_bg) >= 9.0);
        // Soft on bg >= 10:1
        assert!(contrast(l_soft, l_bg) >= 10.0);
        // White text on accent >= 4.5:1 (WCAG AA)
        assert!(contrast(l_white, l_accent) >= 4.5);
        // White text on darker accent hover is even better (>= 4.5:1)
        assert!(contrast(l_white, l_accent_hover) >= 4.8);
    }
}
