use gpui::*;
use gpui_component::{ThemeConfig, ThemeConfigColors};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Theme {
    pub background: Rgba,
    pub background_weak: Rgba,
    pub surface: Rgba,
    pub surface_strong: Rgba,
    pub border: Rgba,
    pub text_disabled: Rgba,
    pub text_muted: Rgba,
    pub text: Rgba,
    pub text_strong: Rgba,
    pub error: Rgba,
    pub warning: Rgba,
    pub info: Rgba,
    pub success: Rgba,
    pub accent: Rgba,
    pub accent_alt: Rgba,
    pub accent_muted: Rgba,
    pub special: Rgba,
}

impl Global for Theme {}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
pub enum ThemeType {
    #[default]
    Nord,
}

impl ThemeType {
    pub fn get_theme(&self) -> Theme {
        match self {
            Self::Nord => Theme {
                background: rgba(0x2E_34_40_FF),
                background_weak: rgba(0x3B_42_52_FF),
                surface: rgba(0x43_4C_5E_FF),
                surface_strong: rgba(0x4C_56_6A_FF),
                border: rgba(0x4C_56_6A_FF),
                text_disabled: rgba(0x4C_56_6A_FF),
                text_muted: rgba(0xD8_DE_E9_FF),
                text: rgba(0xE5_E9_F0_FF),
                text_strong: rgba(0xEC_EF_F4_FF),
                error: rgba(0xBF_61_6A_FF),
                warning: rgba(0xD0_87_70_FF),
                info: rgba(0xEB_CB_8B_FF),
                success: rgba(0xA3_BE_8C_FF),
                accent: rgba(0x81_A1_C1_FF),
                accent_alt: rgba(0x88_C0_D0_FF),
                accent_muted: rgba(0xB4_8E_AD_FF),
                special: rgba(0x5E_81_AC_FF),
            },
        }
    }
}

impl From<Theme> for ThemeConfig {
    fn from(value: Theme) -> Self {
        let to_hex = |c: gpui::Rgba| -> SharedString {
            format!(
                "#{:02x}{:02x}{:02x}{:02x}",
                (c.r * 255.0) as u8,
                (c.g * 255.0) as u8,
                (c.b * 255.0) as u8,
                (c.a * 255.0) as u8
            )
            .into()
        };

        let mut colors = ThemeConfigColors::default();

        colors.background = Some(to_hex(value.background));
        colors.foreground = Some(to_hex(value.text));
        colors.border = Some(to_hex(value.border));
        colors.ring = Some(to_hex(value.accent));
        colors.caret = Some(to_hex(value.accent));

        colors.accent = Some(to_hex(value.background_weak));
        colors.accent_foreground = Some(to_hex(value.text_strong));
        colors.selection = Some(to_hex(value.surface));

        colors.input = Some(to_hex(value.border));
        colors.muted = Some(to_hex(value.background_weak));
        colors.muted_foreground = Some(to_hex(value.text_disabled));

        colors.primary = Some(to_hex(value.accent));
        colors.primary_hover = Some(to_hex(value.accent_alt));
        colors.primary_active = Some(to_hex(value.special));
        colors.primary_foreground = Some(to_hex(value.background));

        colors.secondary = Some(to_hex(value.surface));
        colors.secondary_hover = Some(to_hex(value.surface_strong));
        colors.secondary_active = Some(to_hex(value.background_weak));
        colors.secondary_foreground = Some(to_hex(value.text_strong));

        colors.accordion = Some(to_hex(value.background));
        colors.accordion_hover = Some(to_hex(value.background_weak));
        colors.group_box = Some(to_hex(value.background_weak));
        colors.group_box_foreground = Some(to_hex(value.text));
        colors.group_box_title_foreground = Some(to_hex(value.text_strong));

        colors.popover = Some(to_hex(value.background_weak));
        colors.popover_foreground = Some(to_hex(value.text));
        colors.overlay = Some(to_hex(value.background));

        colors.list = Some(to_hex(value.background));
        colors.list_even = Some(to_hex(value.background_weak));
        colors.list_head = Some(to_hex(value.surface));
        colors.list_hover = Some(to_hex(value.surface));
        colors.list_active = Some(to_hex(value.surface_strong));
        colors.list_active_border = Some(to_hex(value.accent));

        colors.table = Some(to_hex(value.background));
        colors.table_even = Some(to_hex(value.background_weak));
        colors.table_head = Some(to_hex(value.surface));
        colors.table_head_foreground = Some(to_hex(value.text_strong));
        colors.table_hover = Some(to_hex(value.surface));
        colors.table_active = Some(to_hex(value.surface_strong));
        colors.table_active_border = Some(to_hex(value.accent));
        colors.table_row_border = Some(to_hex(value.border));

        colors.sidebar = Some(to_hex(value.background_weak));
        colors.sidebar_border = Some(to_hex(value.border));
        colors.sidebar_foreground = Some(to_hex(value.text_muted));
        colors.sidebar_accent = Some(to_hex(value.surface));
        colors.sidebar_accent_foreground = Some(to_hex(value.text_strong));
        colors.sidebar_primary = Some(to_hex(value.accent));
        colors.sidebar_primary_foreground = Some(to_hex(value.background));

        colors.tab = Some(to_hex(value.background_weak));
        colors.tab_foreground = Some(to_hex(value.text_disabled));
        colors.tab_active = Some(to_hex(value.background));
        colors.tab_active_foreground = Some(to_hex(value.text_strong));
        colors.tab_bar = Some(to_hex(value.surface));
        colors.tab_bar_segmented = Some(to_hex(value.surface_strong));
        colors.title_bar = Some(to_hex(value.background_weak));
        colors.title_bar_border = Some(to_hex(value.border));
        colors.window_border = Some(to_hex(value.border));
        colors.tiles = Some(to_hex(value.background));

        colors.scrollbar = Some(to_hex(value.background));
        colors.scrollbar_thumb = Some(to_hex(value.surface));
        colors.scrollbar_thumb_hover = Some(to_hex(value.surface_strong));
        colors.slider_bar = Some(to_hex(value.surface));
        colors.slider_thumb = Some(to_hex(value.accent));
        colors.progress_bar = Some(to_hex(value.accent));

        colors.danger = Some(to_hex(value.error));
        colors.danger_hover = Some(to_hex(value.error));
        colors.danger_active = Some(to_hex(value.error));
        colors.danger_foreground = Some(to_hex(value.text_strong));

        colors.warning = Some(to_hex(value.warning));
        colors.warning_hover = Some(to_hex(value.warning));
        colors.warning_active = Some(to_hex(value.warning));
        colors.warning_foreground = Some(to_hex(value.text_strong));

        colors.info = Some(to_hex(value.info));
        colors.info_hover = Some(to_hex(value.info));
        colors.info_active = Some(to_hex(value.info));
        colors.info_foreground = Some(to_hex(value.text_strong));

        colors.success = Some(to_hex(value.success));
        colors.success_hover = Some(to_hex(value.success));
        colors.success_active = Some(to_hex(value.success));
        colors.success_foreground = Some(to_hex(value.text_strong));

        colors.skeleton = Some(to_hex(value.surface));
        colors.switch = Some(to_hex(value.surface));
        colors.switch_thumb = Some(to_hex(value.text_strong));
        colors.drag_border = Some(to_hex(value.accent_alt));
        colors.drop_target = Some(to_hex(value.surface));
        colors.link = Some(to_hex(value.accent));
        colors.link_hover = Some(to_hex(value.accent_alt));
        colors.link_active = Some(to_hex(value.special));

        colors.chart_1 = Some(to_hex(value.accent));
        colors.chart_2 = Some(to_hex(value.accent_alt));
        colors.chart_3 = Some(to_hex(value.special));
        colors.chart_4 = Some(to_hex(value.accent_muted));
        colors.chart_5 = Some(to_hex(value.success));
        colors.bullish = Some(to_hex(value.success));
        colors.bearish = Some(to_hex(value.error));

        Self {
            colors,
            ..Default::default()
        }
    }
}
