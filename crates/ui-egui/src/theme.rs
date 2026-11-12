//! Design tokens. Every widget reads colours/sizes from [`Tokens`] so themes apply everywhere.
//!
//! The default theme follows the look of After Effects' dark UI (values estimated from public
//! documentation and product knowledge, tuned by eye; see `plan/aftereffects/README.md` §2). Fonts
//! are Inter + JetBrains Mono (OFL).

use std::sync::Arc;

#[cfg(not(target_arch = "wasm32"))]
use egui::FontTweak;
#[cfg(not(target_arch = "wasm32"))]
use egui::emath::GuiRounding as _;
use egui::{Color32, FontData, FontDefinitions, FontFamily, FontId, Stroke, TextStyle, Visuals};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeKind {
    /// AI Анимация' Brand theme: navy chrome, the brand's blue-violet accent, solid black
    /// menus and windows with a strong shadow, and the brand backdrop round the composition.
    #[default]
    Brand,
    Dark,
    Darker,
    Light,
}

impl ThemeKind {
    pub fn from_name(s: &str) -> Option<ThemeKind> {
        match s.to_ascii_lowercase().as_str() {
            "brand" | "default" => Some(ThemeKind::Brand),
            "dark" => Some(ThemeKind::Dark),
            "darker" | "darkest" => Some(ThemeKind::Darker),
            "light" => Some(ThemeKind::Light),
            _ => None,
        }
    }

    /// The settings value (`appearance.theme`, `appearance.darkTheme`, `appearance.lightTheme`).
    pub fn name(self) -> &'static str {
        match self {
            ThemeKind::Brand => "brand",
            ThemeKind::Dark => "dark",
            ThemeKind::Darker => "darker",
            ThemeKind::Light => "light",
        }
    }

    pub fn is_light(self) -> bool {
        self == ThemeKind::Light
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tokens {
    pub kind: ThemeKind,
    pub app_bg: Color32,
    pub header_bg: Color32,
    pub panel_bg: Color32,
    pub tab_text: Color32,
    pub tab_text_active: Color32,
    pub focus: Color32,
    pub accent: Color32,
    pub accent_hover: Color32,
    pub text: Color32,
    pub text_dim: Color32,
    pub text_faint: Color32,
    pub icon: Color32,
    pub icon_active: Color32,
    pub hover: Color32,
    pub pressed: Color32,
    pub field_bg: Color32,
    pub field_border: Color32,
    pub separator: Color32,
    pub row: Color32,
    pub row_alt: Color32,
    pub row_selected: Color32,
    pub hot_text: Color32,
    pub tl_bg: Color32,
    pub tl_ruler_bg: Color32,
    pub tl_ruler_tick: Color32,
    pub tl_ruler_text: Color32,
    pub cti: Color32,
    pub work_area: Color32,
    pub cache_green: Color32,
    /// Timeline cache bar for frames held in the disk cache (not in RAM).
    pub cache_blue: Color32,
    pub keyframe: Color32,
    pub keyframe_selected: Color32,
    pub pasteboard: Color32,
    pub timecode: Color32,
    pub danger: Color32,
    pub warning: Color32,
    /// Effect Controls effect headers, and the header of the effect being worked on (it or one
    /// of its properties selected).
    pub fx_header: Color32,
    pub fx_header_active: Color32,
    /// Timeline: a selected layer's row in the time graph, the boxes behind the A/V and layer
    /// switches, the time navigator and the work area bar.
    pub tl_graph_selected: Color32,
    pub switch_well: Color32,
    pub tl_nav_bar: Color32,
    pub tl_work_area_bar: Color32,
    /// The Timeline's expression field and the colour of enabled expressions (their text and
    /// the names of properties that have one).
    pub expr_bg: Color32,
    pub expr_text: Color32,
    /// Label colours (`Label::ALL` order), from Settings ▸ Labels.
    pub labels: [Color32; 17],
    pub radius: f32,
    pub radius_sm: f32,
    pub gap: f32,
    pub tab_h: f32,
    pub row_h: f32,
    /// Settings ▸ Appearance ▸ Use Gradients: panel tab strips and buttons get a soft vertical
    /// gradient (off: flat fills).
    pub gradients: bool,
}

/// A vertical gradient fill (`top` → `bottom`) over `rect`.
pub fn gradient_rect(painter: &egui::Painter, rect: egui::Rect, top: Color32, bottom: Color32) {
    let mut m = egui::Mesh::default();
    m.colored_vertex(rect.left_top(), top);
    m.colored_vertex(rect.right_top(), top);
    m.colored_vertex(rect.right_bottom(), bottom);
    m.colored_vertex(rect.left_bottom(), bottom);
    m.add_triangle(0, 1, 2);
    m.add_triangle(0, 2, 3);
    painter.add(egui::Shape::mesh(m));
}

impl Tokens {
    /// The gradient top colour for a fill (a little lighter), or the fill itself when gradients
    /// are off.
    pub fn grad_top(&self, c: Color32) -> Color32 {
        if self.gradients { c.lerp_to_gamma(Color32::WHITE, 0.06) } else { c }
    }

    /// The tokens a property's value fields use (#1306): with an expression driving the property
    /// (`expression`, [`Property::has_expression`]) its values show in the expression colour, red
    /// as in After Effects, hovered and dragged too; otherwise these tokens. For the values only:
    /// the name, stopwatch and swatches keep the ordinary tokens.
    ///
    /// [`Property::has_expression`]: effectcraft_engine::project::Property::has_expression
    pub fn for_value(self, expression: bool) -> Tokens {
        if expression { Tokens { hot_text: self.expr_text, accent_hover: self.expr_text, ..self } } else { self }
    }
}

impl Tokens {
    pub fn for_kind(kind: ThemeKind) -> Self {
        let dark = Tokens {
            kind,
            app_bg: Color32::from_rgb(0x12, 0x12, 0x12),
            header_bg: Color32::from_rgb(0x1f, 0x1f, 0x1f),
            panel_bg: Color32::from_rgb(0x23, 0x23, 0x23),
            tab_text: Color32::from_rgb(0x9a, 0x9a, 0x9a),
            tab_text_active: Color32::from_rgb(0xe3, 0xe3, 0xe3),
            focus: Color32::from_rgb(0x2d, 0x8c, 0xeb),
            accent: Color32::from_rgb(0x2d, 0x8c, 0xeb),
            accent_hover: Color32::from_rgb(0x4a, 0xa0, 0xf5),
            text: Color32::from_rgb(0xc8, 0xc8, 0xc8),
            text_dim: Color32::from_rgb(0x9a, 0x9a, 0x9a),
            text_faint: Color32::from_rgb(0x8c, 0x8c, 0x8c),
            icon: Color32::from_rgb(0xb4, 0xb4, 0xb4),
            icon_active: Color32::from_rgb(0xf0, 0xf0, 0xf0),
            hover: Color32::from_rgb(0x30, 0x30, 0x30),
            pressed: Color32::from_rgb(0x45, 0x45, 0x45),
            field_bg: Color32::from_rgb(0x17, 0x17, 0x17),
            field_border: Color32::from_rgb(0x3a, 0x3a, 0x3a),
            separator: Color32::from_rgb(0x34, 0x34, 0x34),
            row: Color32::from_rgb(0x23, 0x23, 0x23),
            row_alt: Color32::from_rgb(0x27, 0x27, 0x27),
            row_selected: Color32::from_rgb(0x3a, 0x3a, 0x3a),
            hot_text: Color32::from_rgb(0x3d, 0x8f, 0xf5),
            tl_bg: Color32::from_rgb(0x1b, 0x1b, 0x1b),
            tl_ruler_bg: Color32::from_rgb(0x23, 0x23, 0x23),
            tl_ruler_tick: Color32::from_rgb(0x70, 0x70, 0x70),
            tl_ruler_text: Color32::from_rgb(0x9a, 0x9a, 0x9a),
            cti: Color32::from_rgb(0x2d, 0x8c, 0xeb),
            work_area: Color32::from_rgb(0x4a, 0x4a, 0x4a),
            cache_green: Color32::from_rgb(0x3c, 0xa6, 0x4c),
            cache_blue: Color32::from_rgb(0x3a, 0x6f, 0xd8),
            keyframe: Color32::from_rgb(0xa8, 0xa8, 0xa8),
            keyframe_selected: Color32::from_rgb(0x3d, 0x8f, 0xf5),
            pasteboard: Color32::from_rgb(0x1a, 0x1a, 0x1a),
            timecode: Color32::from_rgb(0x3d, 0x8f, 0xf5),
            danger: Color32::from_rgb(0xe0, 0x4a, 0x3c),
            warning: Color32::from_rgb(0xe8, 0x9a, 0x2c),
            fx_header: Color32::from_rgb(0x2a, 0x2a, 0x2a),
            fx_header_active: Color32::from_rgb(0x2f, 0x3a, 0x52),
            tl_graph_selected: Color32::from_rgb(0x2a, 0x2a, 0x2a),
            switch_well: Color32::from_rgb(0x19, 0x19, 0x19),
            tl_nav_bar: Color32::from_rgb(0x55, 0x55, 0x55),
            tl_work_area_bar: Color32::from_rgb(0x5c, 0x5c, 0x5c),
            expr_bg: Color32::from_rgb(0x1a, 0x1a, 0x1a),
            expr_text: Color32::from_rgb(0xe8, 0x7c, 0x5c),
            labels: default_labels(),
            radius: 6.0,
            radius_sm: 3.0,
            gap: 4.0,
            tab_h: 30.0,
            row_h: 19.0,
            gradients: true,
        };
        match kind {
            // AI Анимация' Dark: the NeuDark palette (chrome #262D3A, cards #282F3D, canvas
            // #1B202A, fields #202632, text #E4EBF6 / #C4CFE0 / #A0ADC2, accent #5CA8FF).
            ThemeKind::Dark => Tokens {
                app_bg: Color32::from_rgb(0x1c, 0x22, 0x2c),
                header_bg: Color32::from_rgb(0x26, 0x2d, 0x3a),
                panel_bg: Color32::from_rgb(0x28, 0x2f, 0x3d),
                tab_text: Color32::from_rgb(0xa0, 0xad, 0xc2),
                tab_text_active: Color32::from_rgb(0xe4, 0xeb, 0xf6),
                focus: Color32::from_rgb(0x5c, 0xa8, 0xff),
                accent: Color32::from_rgb(0x5c, 0xa8, 0xff),
                accent_hover: Color32::from_rgb(0x8e, 0xc6, 0xff),
                text: Color32::from_rgb(0xc4, 0xcf, 0xe0),
                text_dim: Color32::from_rgb(0xa0, 0xad, 0xc2),
                text_faint: Color32::from_rgb(0x84, 0x91, 0xa8),
                icon: Color32::from_rgb(0xc4, 0xcf, 0xe0),
                icon_active: Color32::from_rgb(0xf2, 0xf6, 0xff),
                hover: Color32::from_rgb(0x32, 0x3a, 0x4a),
                pressed: Color32::from_rgb(0x3b, 0x45, 0x58),
                field_bg: Color32::from_rgb(0x20, 0x26, 0x32),
                field_border: Color32::from_rgb(0x46, 0x51, 0x66),
                separator: Color32::from_rgb(0x1c, 0x22, 0x2c),
                row: Color32::from_rgb(0x28, 0x2f, 0x3d),
                row_alt: Color32::from_rgb(0x2c, 0x33, 0x42),
                row_selected: Color32::from_rgb(0x2c, 0x4a, 0x6e),
                hot_text: Color32::from_rgb(0x8e, 0xc6, 0xff),
                tl_bg: Color32::from_rgb(0x22, 0x28, 0x34),
                tl_ruler_bg: Color32::from_rgb(0x28, 0x2f, 0x3d),
                tl_ruler_tick: Color32::from_rgb(0x84, 0x91, 0xa8),
                tl_ruler_text: Color32::from_rgb(0xa0, 0xad, 0xc2),
                cti: Color32::from_rgb(0x5c, 0xa8, 0xff),
                work_area: Color32::from_rgb(0x46, 0x51, 0x66),
                keyframe: Color32::from_rgb(0xb4, 0xc0, 0xd4),
                keyframe_selected: Color32::from_rgb(0x8e, 0xc6, 0xff),
                pasteboard: Color32::from_rgb(0x1b, 0x20, 0x2a),
                timecode: Color32::from_rgb(0x8e, 0xc6, 0xff),
                fx_header: Color32::from_rgb(0x2d, 0x35, 0x44),
                fx_header_active: Color32::from_rgb(0x2c, 0x4a, 0x6e),
                tl_graph_selected: Color32::from_rgb(0x30, 0x38, 0x48),
                switch_well: Color32::from_rgb(0x1e, 0x24, 0x30),
                tl_nav_bar: Color32::from_rgb(0x5a, 0x66, 0x7c),
                tl_work_area_bar: Color32::from_rgb(0x62, 0x6e, 0x86),
                expr_bg: Color32::from_rgb(0x20, 0x26, 0x32),
                radius: 8.0,
                radius_sm: 5.0,
                ..dark
            },
            // The brand gradient's navy (#0a0a0f → #1a1a2e → #16213e) for the chrome, the brand's
            // blue-violet (#667eea) for the accent; the composition frame keeps its own colour.
            ThemeKind::Brand => Tokens {
                app_bg: Color32::from_rgb(0x07, 0x07, 0x0d),
                header_bg: Color32::from_rgb(0x0c, 0x0d, 0x17),
                panel_bg: Color32::from_rgb(0x16, 0x18, 0x2a),
                tab_text: Color32::from_rgb(0xb7, 0xbb, 0xd8),
                tab_text_active: Color32::WHITE,
                focus: Color32::from_rgb(0x8a, 0x9b, 0xff),
                accent: Color32::from_rgb(0x66, 0x7e, 0xea),
                accent_hover: Color32::from_rgb(0x8a, 0x9b, 0xff),
                text: Color32::from_rgb(0xee, 0xf0, 0xff),
                text_dim: Color32::from_rgb(0xb7, 0xbb, 0xd8),
                text_faint: Color32::from_rgb(0x8d, 0x91, 0xb4),
                icon: Color32::from_rgb(0xe0, 0xe3, 0xfa),
                icon_active: Color32::WHITE,
                hover: Color32::from_rgb(0x22, 0x25, 0x3e),
                pressed: Color32::from_rgb(0x2d, 0x31, 0x52),
                field_bg: Color32::from_rgb(0x1f, 0x22, 0x38),
                field_border: Color32::from_rgb(0x4a, 0x4f, 0x75),
                separator: Color32::from_rgb(0x2a, 0x2d, 0x46),
                row: Color32::from_rgb(0x16, 0x18, 0x2a),
                row_alt: Color32::from_rgb(0x1a, 0x1c, 0x31),
                row_selected: Color32::from_rgb(0x2a, 0x30, 0x66),
                hot_text: Color32::from_rgb(0xa9, 0xb4, 0xff),
                tl_bg: Color32::from_rgb(0x11, 0x12, 0x22),
                tl_ruler_bg: Color32::from_rgb(0x16, 0x18, 0x2a),
                tl_ruler_tick: Color32::from_rgb(0x8d, 0x91, 0xb4),
                tl_ruler_text: Color32::from_rgb(0xb7, 0xbb, 0xd8),
                cti: Color32::from_rgb(0x8a, 0x9b, 0xff),
                work_area: Color32::from_rgb(0x3a, 0x3e, 0x5e),
                keyframe: Color32::from_rgb(0xc4, 0xc8, 0xe8),
                keyframe_selected: Color32::from_rgb(0xa9, 0xb4, 0xff),
                pasteboard: BRAND_CANVAS,
                timecode: Color32::from_rgb(0xa9, 0xb4, 0xff),
                fx_header: Color32::from_rgb(0x1f, 0x22, 0x38),
                fx_header_active: Color32::from_rgb(0x2a, 0x30, 0x66),
                tl_graph_selected: Color32::from_rgb(0x22, 0x25, 0x3e),
                switch_well: Color32::from_rgb(0x0e, 0x0f, 0x1c),
                tl_nav_bar: Color32::from_rgb(0x4a, 0x4f, 0x75),
                tl_work_area_bar: Color32::from_rgb(0x55, 0x5b, 0x88),
                expr_bg: Color32::from_rgb(0x11, 0x12, 0x22),
                radius: 6.0,
                radius_sm: 4.0,
                ..dark
            },
            ThemeKind::Darker => Tokens {
                app_bg: Color32::from_rgb(0x08, 0x08, 0x08),
                header_bg: Color32::from_rgb(0x16, 0x16, 0x16),
                panel_bg: Color32::from_rgb(0x19, 0x19, 0x19),
                row: Color32::from_rgb(0x19, 0x19, 0x19),
                row_alt: Color32::from_rgb(0x1d, 0x1d, 0x1d),
                tl_bg: Color32::from_rgb(0x12, 0x12, 0x12),
                tl_ruler_bg: Color32::from_rgb(0x19, 0x19, 0x19),
                field_bg: Color32::from_rgb(0x0e, 0x0e, 0x0e),
                pasteboard: Color32::from_rgb(0x11, 0x11, 0x11),
                ..dark
            },
            // AI Анимация' Light: the NeuLight palette (chrome #E4ECF6, cards #E6EEF8, canvas
            // #D6E0ED, fields #DCE6F3, text #1E2C42 / #34445E / #4E5E78, accent #3480E2, accent
            // text #1652A8).
            ThemeKind::Light => Tokens {
                app_bg: Color32::from_rgb(0xc9, 0xd5, 0xe5),
                header_bg: Color32::from_rgb(0xe4, 0xec, 0xf6),
                panel_bg: Color32::from_rgb(0xe6, 0xee, 0xf8),
                tab_text: Color32::from_rgb(0x4e, 0x5e, 0x78),
                tab_text_active: Color32::from_rgb(0x1e, 0x2c, 0x42),
                focus: Color32::from_rgb(0x34, 0x80, 0xe2),
                accent: Color32::from_rgb(0x34, 0x80, 0xe2),
                accent_hover: Color32::from_rgb(0x16, 0x52, 0xa8),
                text: Color32::from_rgb(0x34, 0x44, 0x5e),
                text_dim: Color32::from_rgb(0x4e, 0x5e, 0x78),
                text_faint: Color32::from_rgb(0x6a, 0x79, 0x92),
                hot_text: Color32::from_rgb(0x16, 0x52, 0xa8),
                timecode: Color32::from_rgb(0x16, 0x52, 0xa8),
                icon: Color32::from_rgb(0x34, 0x44, 0x5e),
                icon_active: Color32::from_rgb(0x1e, 0x2c, 0x42),
                hover: Color32::from_rgb(0xd5, 0xe0, 0xee),
                pressed: Color32::from_rgb(0xc8, 0xd6, 0xea),
                field_bg: Color32::from_rgb(0xdc, 0xe6, 0xf3),
                field_border: Color32::from_rgb(0xb3, 0xc2, 0xd6),
                separator: Color32::from_rgb(0xc9, 0xd5, 0xe5),
                row: Color32::from_rgb(0xe6, 0xee, 0xf8),
                row_alt: Color32::from_rgb(0xdf, 0xe8, 0xf4),
                row_selected: Color32::from_rgb(0xc2, 0xd8, 0xf5),
                tl_bg: Color32::from_rgb(0xdc, 0xe5, 0xf1),
                tl_ruler_bg: Color32::from_rgb(0xe9, 0xf0, 0xf8),
                tl_ruler_tick: Color32::from_rgb(0x6a, 0x79, 0x92),
                tl_ruler_text: Color32::from_rgb(0x4e, 0x5e, 0x78),
                cti: Color32::from_rgb(0x34, 0x80, 0xe2),
                work_area: Color32::from_rgb(0xb3, 0xc2, 0xd6),
                pasteboard: Color32::from_rgb(0xd6, 0xe0, 0xed),
                // Keyframes have no outline: dark enough to stand out on the light time graph.
                keyframe: Color32::from_rgb(0x4e, 0x5e, 0x78),
                keyframe_selected: Color32::from_rgb(0x16, 0x52, 0xa8),
                // Custom-painted surfaces in the Light palette, so dark text stays readable on
                // them (#644).
                fx_header: Color32::from_rgb(0xda, 0xe3, 0xef),
                fx_header_active: Color32::from_rgb(0xc2, 0xd8, 0xf5),
                tl_graph_selected: Color32::from_rgb(0xcc, 0xd9, 0xea),
                switch_well: Color32::from_rgb(0xee, 0xf3, 0xfa),
                tl_nav_bar: Color32::from_rgb(0x9a, 0xab, 0xc4),
                tl_work_area_bar: Color32::from_rgb(0x8d, 0x9e, 0xb8),
                expr_bg: Color32::from_rgb(0xee, 0xf3, 0xfa),
                expr_text: Color32::from_rgb(0xa3, 0x34, 0x17),
                radius: 8.0,
                radius_sm: 5.0,
                ..dark
            },
        }
    }

    pub fn mono(size: f32) -> FontId {
        FontId::new(size, FontFamily::Monospace)
    }
    pub fn ui(size: f32) -> FontId {
        FontId::new(size, FontFamily::Proportional)
    }
    pub fn semibold(size: f32) -> FontId {
        FontId::new(size, FontFamily::Name("semibold".into()))
    }
    pub fn medium(size: f32) -> FontId {
        FontId::new(size, FontFamily::Name("medium".into()))
    }
    /// sRGB colour of an item/layer label (Settings ▸ Labels).
    pub fn label(&self, l: effectcraft_color::Label) -> Color32 {
        let i = effectcraft_color::Label::ALL.iter().position(|x| *x == l).unwrap_or(0);
        self.labels[i]
    }

    /// Theme tokens for the current settings: theme, UI brightness and label colours.
    /// The tokens for theme `kind` (resolved from the appearance mode) with the rest of the
    /// Appearance settings: brightness, label colours, gradients.
    pub fn from_prefs(p: &effectcraft_engine::prefs::Prefs, kind: ThemeKind) -> Tokens {
        let mut t = Tokens::for_kind(kind).with_brightness(p.appearance.brightness as f32);
        for (i, l) in effectcraft_color::Label::ALL.iter().enumerate() {
            let [r, g, b] = p.label_rgb(*l);
            t.labels[i] = Color32::from_rgb(r, g, b);
        }
        t.gradients = p.appearance.use_gradients;
        t
    }

    /// Lighten (b > 0) or darken (b < 0) the interface greys, like After Effects' brightness
    /// slider. Accent colours, labels and keyframe colours are kept.
    pub fn with_brightness(mut self, b: f32) -> Tokens {
        let b = b.clamp(-1.0, 1.0);
        if b.abs() < 1e-3 {
            return self;
        }
        let adj = |c: &mut Color32| {
            let f = |v: u8| -> u8 {
                let v = v as f32;
                (if b > 0.0 { v + (110.0 - v).max(0.0) * b * 0.8 + 10.0 * b } else { v * (1.0 + b * 0.6) }).round().clamp(0.0, 255.0) as u8
            };
            *c = Color32::from_rgb(f(c.r()), f(c.g()), f(c.b()));
        };
        for c in [
            &mut self.app_bg,
            &mut self.header_bg,
            &mut self.panel_bg,
            &mut self.hover,
            &mut self.pressed,
            &mut self.field_bg,
            &mut self.field_border,
            &mut self.separator,
            &mut self.row,
            &mut self.row_alt,
            &mut self.row_selected,
            &mut self.tl_bg,
            &mut self.tl_ruler_bg,
            &mut self.pasteboard,
            &mut self.work_area,
            &mut self.fx_header,
            &mut self.tl_graph_selected,
            &mut self.switch_well,
            &mut self.expr_bg,
        ] {
            adj(c);
        }
        self
    }
}

/// The default label colours, one RGB colour per [`effectcraft_color::Label`] in
/// `Label::ALL` order.
fn default_labels() -> [Color32; 17] {
    effectcraft_color::Label::ALL.map(|l| {
        let [r, g, b] = l.rgb();
        Color32::from_rgb(r, g, b)
    })
}

static INTER_REGULAR: &[u8] = include_bytes!("../../../assets/fonts/Inter-Regular.ttf");
static INTER_MEDIUM: &[u8] = include_bytes!("../../../assets/fonts/Inter-Medium.ttf");
static INTER_SEMIBOLD: &[u8] = include_bytes!("../../../assets/fonts/Inter-SemiBold.ttf");
static JETBRAINS_MONO: &[u8] = include_bytes!("../../../assets/fonts/JetBrainsMono-Regular.ttf");
/// Chrome symbols (`✔` U+2714, `▣` U+25A3, …) that Inter, Inter Medium/SemiBold and
/// JetBrains Mono do not cover. 0.37's default font set no longer includes the old emoji
/// fonts, and named families (`medium`, `semibold`) never received egui's default fallbacks.
static NOTO_SYMBOLS2: &[u8] = include_bytes!("../../../assets/fonts/NotoSansSymbols2-Regular.ttf");

/// Install fonts (Inter, Inter Medium/SemiBold, JetBrains Mono) and egui visuals. `language` is the
/// resolved interface language from `crate::i18n::language` (`en`, `ja`, `zh-hans` or `zh-hant`),
/// not the stored `general.language` preference: `system` is already resolved there, so a Chinese
/// system draws Chinese with the default setting. The CJK fallback that draws the interface's own
/// script is registered first, so a Chinese interface is drawn by a Chinese face and a Japanese one
/// by a Japanese face. `install` is called again when the language changes (see the `EffectcraftApp`
/// frame loop), so the fonts do not need a restart.
pub fn install(ctx: &egui::Context, t: &Tokens, language: &str) {
    let mut fonts = FontDefinitions::default();
    let proportional_fallbacks = fonts.families.get(&FontFamily::Proportional).cloned().unwrap_or_default();
    fonts.font_data.insert("inter".into(), Arc::new(FontData::from_static(INTER_REGULAR)));
    fonts.font_data.insert("inter-medium".into(), Arc::new(FontData::from_static(INTER_MEDIUM)));
    fonts.font_data.insert("inter-semibold".into(), Arc::new(FontData::from_static(INTER_SEMIBOLD)));
    fonts.font_data.insert("jbmono".into(), Arc::new(FontData::from_static(JETBRAINS_MONO)));
    fonts.font_data.insert("noto-symbols2".into(), Arc::new(FontData::from_static(NOTO_SYMBOLS2)));
    fonts.families.entry(FontFamily::Proportional).or_default().insert(0, "inter".into());
    fonts.families.entry(FontFamily::Monospace).or_default().insert(0, "jbmono".into());
    // Named weights need the same symbol fallback chain as ordinary labels.
    // The bundled Inter subset and JetBrains Mono do not cover every UI symbol.
    for (family, face) in [("semibold", "inter-semibold"), ("medium", "inter-medium")] {
        let mut stack = vec![face.into(), "inter".into()];
        stack.extend(proportional_fallbacks.iter().cloned());
        fonts.families.insert(FontFamily::Name(family.into()), stack);
    }
    // Inter and egui's own fallbacks have no small triangles (the `▸` of "File ▸ Import"), which
    // would draw as missing-glyph boxes: JetBrains Mono fills in the symbols the others lack.
    // Noto Sans Symbols 2 follows it for the check/box glyphs (`✔`, `▣`) that JetBrains Mono
    // lacks; without it, named families such as `medium` fall straight through to tofu.
    for (family, stack) in &mut fonts.families {
        if *family != FontFamily::Monospace {
            stack.push("jbmono".into());
        }
        stack.push("noto-symbols2".into());
    }
    install_script_fallbacks(&mut fonts, language);
    ctx.set_fonts(fonts);
    apply_visuals(ctx, t);
}

/// Scripts the bundled fonts lack beyond CJK, each probed with one of its letters: names typed in
/// them (layers, items, comments) are drawn by an installed face instead of missing-glyph boxes
/// (#394). egui shapes each run of one face, so Arabic letters join and a word reads right to
/// left; it has no bidirectional layout, so the words of a name still follow each other left to
/// right.
#[cfg(not(target_arch = "wasm32"))]
const SCRIPT_PROBES: &[(char, &str)] = &[
    ('م', "arabic-system"),
    ('א', "hebrew-system"),
    ('क', "devanagari-system"),
    ('ক', "bengali-system"),
    ('ਕ', "gurmukhi-system"),
    ('ક', "gujarati-system"),
    ('க', "tamil-system"),
    ('క', "telugu-system"),
    ('ಕ', "kannada-system"),
    ('ക', "malayalam-system"),
    ('ก', "thai-system"),
    ('ა', "georgian-system"),
];

/// The size the fallback baseline correction is derived at. The correction is an em fraction, so the
/// value holds at every interface size; any size in the interface's range gives the same answer.
#[cfg(not(target_arch = "wasm32"))]
const BASELINE_METRICS_PX: f32 = 12.0;

/// A face's ascent and row height at `px`, rounded the way epaint rounds them: ascent, descent and
/// line gap separately to 1/32, then `ascent + descent + line_gap` (epaint's descent is negative,
/// this one is positive).
#[cfg(not(target_arch = "wasm32"))]
fn ascent_and_height(m: &effectcraft_text::fonts::VMetrics) -> (f32, f32) {
    let ascent = m.ascent.round_ui();
    let descent = m.descent.round_ui();
    let line_gap = m.line_gap.round_ui();
    (ascent, ascent + descent + line_gap)
}

/// The `FontTweak` that puts a fallback face's glyphs on the row font's baseline.
///
/// epaint draws a glyph at `A_face + 0.5·(H_row − H_face)` (`galley_from_rows` in epaint's
/// `text_layout.rs`), where `A` and `H` are the ascent and the row height of the face that draws the
/// glyph and of the row's own font, both from `styled_metrics` in epaint's `font.rs`. Shifting the
/// face by `(A_row − A_face) − 0.5·(H_row − H_face)` cancels the offset; as an em fraction one
/// number serves every size, because `FontTweak` applies `size · y_offset_factor`.
///
/// The same expression also carries `valign · (max_row_height − line_height)`, which cancels here:
/// every glyph of a row takes the section font's row height, so it is the same term for the Latin
/// and the fallback glyph. The remaining `snap_to_pixel` difference is likewise shared and cancels.
/// (#1309)
///
/// The UI families do not share a row font — Proportional uses Inter, Monospace JetBrains Mono —
/// and their factors differ by 0.0026 em, which is 0.03 px at 12 pt: inside the 1/32 that is already
/// rounded away above, so one factor per face is enough. The test
/// `fallback_glyphs_sit_on_the_row_baseline` pins the result.
#[cfg(not(target_arch = "wasm32"))]
fn baseline_tweak(row: &effectcraft_text::fonts::Face, face: &effectcraft_text::fonts::Face) -> FontTweak {
    let px = BASELINE_METRICS_PX;
    let (a_row, h_row) = ascent_and_height(&row.metrics(px));
    let (a_face, h_face) = ascent_and_height(&face.metrics(px));
    FontTweak { y_offset_factor: ((a_row - a_face) - 0.5 * (h_row - h_face)) / px, ..Default::default() }
}

/// Register the system fallbacks for scripts the bundled fonts lack in every font family, and
/// return their names in registration order (empty when nothing installed covers a probe, so a
/// machine without those fonts simply keeps the bundled ones). A face that covers several probes
/// (Segoe UI draws Arabic and Hebrew, Nirmala UI every Indic script) is registered once.
///
/// CJK comes first. The text engine has its own script-aware fallback (#84), but which face it
/// picks for Han ideographs depends on the locale environment: a Chinese interface on a
/// Japanese-locale machine would be drawn with the Japanese face. The interface language is the
/// better signal, so the probe for its script comes first — kana for Japanese, a Han character for
/// the Chinese catalogs. Both scripts are registered, so file, layer and template names in the
/// other one still render; then [`SCRIPT_PROBES`], then Hangul, since the Japanese and Chinese faces
/// may lack it (Hiragino Sans on macOS has none). The web build has no system fonts
/// and installs nothing.
fn install_script_fallbacks(family_fonts: &mut FontDefinitions, language: &str) -> Vec<String> {
    #[cfg(target_arch = "wasm32")]
    {
        let _ = (family_fonts, language);
        Vec::new()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        use effectcraft_text::fonts;
        // Kana is unambiguously Japanese; Han ideographs are shared, so the order decides which
        // language's face draws them.
        const KANA: (char, &str) = ('あ', "japanese-system");
        // Hangul is probed last: with no Korean font installed the lookup can land on a catch-all
        // face (Unifont) that also covers Arabic or Indic text but can't shape it, so the
        // script-specific faces must be registered ahead of it. A face already seen is skipped.
        const HANGUL: (char, &str) = ('한', "korean-system");
        // Han text follows the interface language, not the operating system's locale.
        fonts::set_cjk_locale(cjk_locale(language));
        let base = fonts::resolve("Inter", "Regular").face;
        let base_face = fonts::face(base);
        let han = (han_probe(language), "chinese-system");
        let cjk = if is_chinese(language) { [han, KANA] } else { [KANA, han] };
        let mut names = Vec::new();
        let mut seen = Vec::new();
        // A font covering one Han probe may be a sparse subset. Register additional faces
        // needed by the actual interface catalog instead of assuming whole-script coverage.
        let extra = if is_chinese(language) { crate::i18n::catalog_characters(language) } else { Vec::new() };
        let extra: Vec<_> = extra.into_iter().filter(|ch| *ch >= '\u{2e80}').map(|ch| (ch, "catalog-system")).collect();
        // 256-character blocks where a catalog character found no face: with nothing installed
        // covering them, each lookup would read every installed font file again.
        let mut uncovered_blocks = std::collections::HashSet::new();
        for &(probe, name) in cjk.iter().chain(SCRIPT_PROBES).chain(std::iter::once(&HANGUL)).chain(&extra) {
            let catalog = name == "catalog-system";
            if catalog && (uncovered_blocks.contains(&(probe as u32 >> 8)) || seen.iter().any(|&id| fonts::face(id).has_char(probe))) {
                continue;
            }
            // The interface asks for the platform's UI face first; the order a composed frame
            // renders with is `fonts::fallback_for`'s and is not touched here. Another context may
            // install a different language concurrently, so the shared fallback is asked with this
            // context's own locale rather than the global request.
            let id = interface_fallback_candidates(name, language)
                .iter()
                .find_map(|family| {
                    let r = fonts::resolve(family, "Regular");
                    (!r.missing && fonts::face(r.face).has_char(probe)).then_some(r.face)
                })
                .unwrap_or_else(|| fonts::fallback_for_locale(probe, base, cjk_locale(language)));
            let face = fonts::face(id);
            if !face.has_char(probe) {
                if catalog {
                    uncovered_blocks.insert(probe as u32 >> 8);
                }
                continue; // nothing installed covers this script
            }
            if seen.contains(&id) {
                continue;
            }
            let Some(font) = face.font() else { continue };
            seen.push(id);
            // Use the already-read, parsed bytes rather than reading a font file twice.
            let mut data = FontData::from_owned(font.data().as_bytes().to_vec());
            data.index = face.info.index;
            let name = if name == "catalog-system" { format!("{name}-{:x}", probe as u32) } else { name.to_string() };
            data.tweak = baseline_tweak(&base_face, &face);
            family_fonts.font_data.insert(name.clone(), Arc::new(data));
            for family in family_fonts.families.values_mut() {
                family.push(name.to_string());
            }
            names.push(name.to_string());
        }
        names
    }
}

/// The faces the interface asks for before it settles on the one a composed frame renders with.
///
/// Windows ships a UI variant of its Japanese and Chinese families - `Meiryo`/`Meiryo UI`, `Yu
/// Gothic`/`Yu Gothic UI`, `MS Gothic`/`MS UI Gothic`, `Microsoft YaHei`/`Microsoft YaHei UI`,
/// `Microsoft JhengHei`/`Microsoft JhengHei UI` - and its own link table for `Segoe UI`
/// (`FontLink\SystemLink`) picks the UI one for what it draws on screen. `PMingLiU` is the one face
/// here that is not a sibling: it is `MingLiU`'s proportional relative, and proportional spacing is
/// what makes the chrome read better in it.
///
/// Only the interface takes this order. `fonts::fallback_for` keeps the order a document renders
/// with, so an existing project's output does not move when the chrome changes.
#[cfg(not(target_arch = "wasm32"))]
fn interface_fallback_candidates(name: &str, language: &str) -> &'static [&'static str] {
    match (name, cjk_locale(language)) {
        // Kana, and the Han ideographs of a Japanese interface: the language asks for the Japanese
        // face, so its UI sibling is the one the platform would draw chrome with.
        ("japanese-system", _) | ("chinese-system", "ja") => &["Meiryo UI", "Yu Gothic UI", "MS UI Gothic"],
        ("chinese-system", "zh-hant") => &["Microsoft JhengHei UI", "PMingLiU"],
        ("chinese-system", "zh-hans") => &["Microsoft YaHei UI"],
        // The other scripts have no UI variant, and their shared order already picks the face made
        // for the script - the one that draws it correctly at all.
        _ => &[],
    }
}

/// The CJK locale the text engine should prefer for Han text: the interface language when it names
/// one, otherwise the empty string, which leaves the operating system's locale in charge.
#[cfg(not(target_arch = "wasm32"))]
fn cjk_locale(language: &str) -> &str {
    match language {
        "ja" | "zh-hans" | "zh-hant" => language,
        _ => "",
    }
}

/// The Han character that picks the Chinese fallback face. A Simplified Chinese interface asks for
/// a hanzi used by its catalog. Some Japanese faces also cover 页 while lacking 选 or 删, so
/// coverage of this probe alone is not enough: the fallback must use the interface's own language
/// policy, even while another context is installing Japanese fonts (#1141).
#[cfg(not(target_arch = "wasm32"))]
fn han_probe(language: &str) -> char {
    if language == "zh-hans" { '页' } else { '文' }
}

/// Whether `language` is one of the Chinese catalogs.
#[cfg(not(target_arch = "wasm32"))]
fn is_chinese(language: &str) -> bool {
    matches!(language, "zh-hans" | "zh-hant")
}

/// egui's theme preference for an appearance mode showing a `light` (or dark) theme. Sync with
/// System leaves it at System: a concrete preference makes egui set the native window's
/// appearance, and on macOS a window with its own appearance no longer hears when the system's
/// changes (#642). A fixed Light or Dark mode pins the window to match.
pub fn theme_preference(appearance_mode: &str, light: bool) -> egui::ThemePreference {
    match appearance_mode {
        "auto" => egui::ThemePreference::System,
        _ if light => egui::ThemePreference::Light,
        _ => egui::ThemePreference::Dark,
    }
}

pub fn apply_visuals(ctx: &egui::Context, t: &Tokens) {
    let light = t.kind.is_light();
    let mut v = if light { Visuals::light() } else { Visuals::dark() };
    v.panel_fill = t.panel_bg;
    v.window_fill = t.panel_bg;
    v.extreme_bg_color = t.field_bg;
    v.faint_bg_color = t.row_alt;
    v.override_text_color = Some(t.text);
    v.selection.bg_fill = t.accent;
    v.selection.stroke = Stroke::new(1.0, Color32::WHITE);
    v.hyperlink_color = t.accent;
    v.window_stroke = Stroke::new(1.0, t.field_border);
    v.window_corner_radius = egui::CornerRadius::same(8);
    v.menu_corner_radius = egui::CornerRadius::same(6);
    v.popup_shadow = egui::epaint::Shadow { offset: [0, 6], blur: 20, spread: 0, color: Color32::from_black_alpha(150) };
    v.window_shadow = egui::epaint::Shadow { offset: [0, 10], blur: 36, spread: 0, color: Color32::from_black_alpha(170) };
    for w in [&mut v.widgets.noninteractive, &mut v.widgets.inactive, &mut v.widgets.hovered, &mut v.widgets.active, &mut v.widgets.open] {
        w.corner_radius = egui::CornerRadius::same(t.radius_sm as u8);
    }
    v.widgets.noninteractive.bg_fill = t.panel_bg;
    v.widgets.noninteractive.weak_bg_fill = t.panel_bg;
    v.widgets.noninteractive.bg_stroke = Stroke::new(1.0, t.separator);
    v.widgets.noninteractive.fg_stroke = Stroke::new(1.0, t.text);
    v.widgets.inactive.bg_fill = t.field_bg;
    v.widgets.inactive.weak_bg_fill = t.field_bg;
    v.widgets.inactive.bg_stroke = Stroke::new(1.0, t.field_border);
    v.widgets.inactive.fg_stroke = Stroke::new(1.0, t.text);
    v.widgets.hovered.bg_fill = t.hover;
    v.widgets.hovered.weak_bg_fill = t.hover;
    v.widgets.hovered.bg_stroke = Stroke::new(1.0, t.field_border);
    v.widgets.hovered.fg_stroke = Stroke::new(1.0, t.tab_text_active);
    v.widgets.active.bg_fill = t.pressed;
    v.widgets.active.weak_bg_fill = t.pressed;
    v.widgets.active.fg_stroke = Stroke::new(1.0, t.tab_text_active);
    v.widgets.open.bg_fill = t.hover;
    v.widgets.open.weak_bg_fill = t.hover;
    match t.kind {
        // Neumorphic Light and Dark: rounded windows and menus, soft shadows tinted with the
        // palette's blue instead of grey.
        ThemeKind::Light | ThemeKind::Dark => {
            let shade = if light { Color32::from_rgba_unmultiplied(0x46, 0x5a, 0x82, 70) } else { Color32::from_black_alpha(150) };
            v.window_corner_radius = egui::CornerRadius::same(10);
            v.menu_corner_radius = egui::CornerRadius::same(8);
            v.popup_shadow = egui::epaint::Shadow { offset: [0, 6], blur: 18, spread: 0, color: shade };
            v.window_shadow = egui::epaint::Shadow { offset: [0, 10], blur: 30, spread: 0, color: shade };
            v.selection.bg_fill =
                if light { Color32::from_rgba_unmultiplied(0x34, 0x80, 0xe2, 0x55) } else { Color32::from_rgba_unmultiplied(0x5c, 0xa8, 0xff, 0x66) };
            v.selection.stroke = Stroke::new(1.0, t.accent);
            v.hyperlink_color = t.hot_text;
        }
        // Brand: menus, popups and windows are solid black blocks with a strong black shadow.
        ThemeKind::Brand => {
            v.window_fill = Color32::BLACK;
            v.panel_fill = t.panel_bg;
            v.window_stroke = Stroke::new(1.0, Color32::from_rgb(0x26, 0x28, 0x3c));
            v.window_corner_radius = egui::CornerRadius::same(8);
            v.menu_corner_radius = egui::CornerRadius::same(6);
            v.popup_shadow = egui::epaint::Shadow { offset: [0, 8], blur: 32, spread: 0, color: Color32::BLACK };
            v.window_shadow = egui::epaint::Shadow { offset: [0, 10], blur: 36, spread: 2, color: Color32::BLACK };
            v.selection.bg_fill = Color32::from_rgba_unmultiplied(0x66, 0x7e, 0xea, 0x88);
            v.hyperlink_color = Color32::from_rgb(0xa9, 0xb4, 0xff);
        }
        ThemeKind::Darker => {}
    }
    // egui keeps one style per light/dark theme and picks it by its theme preference (see
    // [`theme_preference`]); both get ours, so whichever it picks, egui's defaults never show.
    ctx.all_styles_mut(|s| {
        s.visuals = v.clone();
        s.spacing.item_spacing = egui::vec2(6.0, 4.0);
        s.spacing.button_padding = egui::vec2(8.0, 3.0);
        s.spacing.interact_size.y = 22.0;
        s.spacing.menu_margin = egui::Margin::same(5);
        // Scroll bars always show (egui's float in only over the list), so long menus and lists
        // can be dragged without a mouse wheel or touchpad (#269).
        s.spacing.scroll = egui::style::ScrollStyle::solid();
        // The handle takes the text colour: the fill it would otherwise get is the colour of its
        // track (both are the field background), which left it invisible until hovered.
        s.spacing.scroll.foreground_color = true;
        s.text_styles.insert(TextStyle::Body, FontId::new(12.0, FontFamily::Proportional));
        s.text_styles.insert(TextStyle::Button, FontId::new(12.0, FontFamily::Proportional));
        s.text_styles.insert(TextStyle::Small, FontId::new(11.0, FontFamily::Proportional));
        s.text_styles.insert(TextStyle::Heading, FontId::new(15.0, FontFamily::Name("semibold".into())));
        s.text_styles.insert(TextStyle::Monospace, FontId::new(12.0, FontFamily::Monospace));
        s.animation_time = 0.1;
    });
}

/// The colour the Brand backdrop reads as: the area round the composition in the Brand theme.
pub const BRAND_CANVAS: Color32 = Color32::from_rgb(0x16, 0x18, 0x2C);

/// The Brand theme's backdrop, as the owner's site draws it in CSS: the brand gradient
/// (#0a0a0f → #1a1a2e → #16213e at 135°), three soft glows, a 30 px grid of 5% white lines and an
/// inner shadow round the edges.
pub fn paint_brand_backdrop(painter: &egui::Painter, rect: egui::Rect) {
    use egui::epaint::{Mesh, Vertex, WHITE_UV};
    if !(rect.width().is_finite() && rect.height().is_finite()) || rect.width() <= 0.0 || rect.height() <= 0.0 {
        return;
    }
    let vertex = |pos: egui::Pos2, color: Color32| Vertex { pos, uv: WHITE_UV, color };
    let quad = |m: &mut Mesh, corners: [(egui::Pos2, Color32); 4]| {
        let base = m.vertices.len() as u32;
        m.vertices.extend(corners.iter().map(|(p, c)| vertex(*p, *c)));
        m.indices.extend([base, base + 1, base + 2, base, base + 2, base + 3]);
    };
    // The gradient: at 135° the top-left corner is the first stop, the bottom-right the last, and
    // the other two corners sit on the middle stop.
    let (a, mid, b) = (Color32::from_rgb(0x0A, 0x0A, 0x0F), Color32::from_rgb(0x1A, 0x1A, 0x2E), Color32::from_rgb(0x16, 0x21, 0x3E));
    let mut m = Mesh::default();
    quad(&mut m, [(rect.left_top(), a), (rect.right_top(), mid), (rect.right_bottom(), b), (rect.left_bottom(), mid)]);
    painter.add(egui::Shape::mesh(m));
    // The glows: a fan from the centre colour out to transparent at half the farthest-corner distance.
    for (fx, fy, rgb, alpha) in [(0.2_f32, 0.8_f32, (120, 119, 198), 0.30_f32), (0.8, 0.2, (255, 119, 198), 0.30), (0.4, 0.4, (120, 219, 255), 0.20)] {
        let c = rect.min + egui::vec2(rect.width() * fx, rect.height() * fy);
        let reach = egui::vec2(rect.width() * fx.max(1.0 - fx), rect.height() * fy.max(1.0 - fy)).length() * 0.5;
        let inner = Color32::from_rgba_unmultiplied(rgb.0, rgb.1, rgb.2, (alpha * 255.0) as u8);
        let mut g = Mesh::default();
        g.vertices.push(vertex(c, inner));
        const STEPS: u32 = 64;
        for i in 0..=STEPS {
            let angle = std::f32::consts::TAU * i as f32 / STEPS as f32;
            g.vertices.push(vertex(c + egui::vec2(angle.cos(), angle.sin()) * reach, Color32::TRANSPARENT));
        }
        for i in 1..=STEPS {
            g.indices.extend([0, i, i + 1]);
        }
        painter.add(egui::Shape::mesh(g));
    }
    // The grid: lines 30 points apart from the top-left corner (at most a few hundred).
    let line = Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 13));
    let mut x = rect.left() + 0.5;
    let mut n = 0;
    while x < rect.right() && n < 400 {
        painter.vline(x, rect.y_range(), line);
        x += 30.0;
        n += 1;
    }
    let mut y = rect.top() + 0.5;
    n = 0;
    while y < rect.bottom() && n < 400 {
        painter.hline(rect.x_range(), y, line);
        y += 30.0;
        n += 1;
    }
    // The inner shadow: solid for the 10 px spread, then fading over the 35 px blur.
    let dark = |a: f32| Color32::from_rgba_unmultiplied(0, 0, 0, (a * 153.0) as u8);
    let mut sh = Mesh::default();
    for (i0, i1, a0, a1) in [(0.0_f32, 10.0_f32, 1.0_f32, 0.75_f32), (10.0, 45.0, 0.75, 0.0)] {
        let r = rect;
        quad(
            &mut sh,
            [
                (egui::pos2(r.left(), r.top() + i0), dark(a0)),
                (egui::pos2(r.right(), r.top() + i0), dark(a0)),
                (egui::pos2(r.right(), r.top() + i1), dark(a1)),
                (egui::pos2(r.left(), r.top() + i1), dark(a1)),
            ],
        );
        quad(
            &mut sh,
            [
                (egui::pos2(r.left(), r.bottom() - i1), dark(a1)),
                (egui::pos2(r.right(), r.bottom() - i1), dark(a1)),
                (egui::pos2(r.right(), r.bottom() - i0), dark(a0)),
                (egui::pos2(r.left(), r.bottom() - i0), dark(a0)),
            ],
        );
        quad(
            &mut sh,
            [
                (egui::pos2(r.left() + i0, r.top()), dark(a0)),
                (egui::pos2(r.left() + i1, r.top()), dark(a1)),
                (egui::pos2(r.left() + i1, r.bottom()), dark(a1)),
                (egui::pos2(r.left() + i0, r.bottom()), dark(a0)),
            ],
        );
        quad(
            &mut sh,
            [
                (egui::pos2(r.right() - i1, r.top()), dark(a1)),
                (egui::pos2(r.right() - i0, r.top()), dark(a0)),
                (egui::pos2(r.right() - i0, r.bottom()), dark(a0)),
                (egui::pos2(r.right() - i1, r.bottom()), dark(a1)),
            ],
        );
    }
    painter.add(egui::Shape::mesh(sh));
}

/// The area round the composition: the brand backdrop in the Brand theme (`fill` is
/// [`BRAND_CANVAS`]), else `fill`.
pub fn paint_canvas_fill(painter: &egui::Painter, rect: egui::Rect, fill: Color32) {
    if fill == BRAND_CANVAS {
        paint_brand_backdrop(painter, rect);
    } else {
        painter.rect_filled(rect, 0.0, fill);
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod script_font_tests {
    use super::{SCRIPT_PROBES, interface_fallback_candidates};
    use effectcraft_text::fonts;

    /// Whether an installed face covers `probe` (else the glyph check is skipped).
    fn installed(probe: char) -> bool {
        let base = fonts::resolve("Inter", "Regular").face;
        fonts::face(fonts::fallback_for(probe, base)).has_char(probe)
    }

    /// Every character of `text` draws a real glyph in every UI font family with the `language`
    /// interface's fonts installed.
    fn assert_drawn(language: &str, text: &str) {
        let ctx = egui::Context::default();
        super::install(&ctx, &super::Tokens::for_kind(super::ThemeKind::Dark), language);
        let mut out = ctx.run_ui(egui::RawInput::default(), |_| {});
        // No renderer here: drop the frame's texture uploads (egui asserts on unhandled ones in debug).
        out.textures_delta.clear();
        ctx.fonts_mut(|fonts| {
            for family in [
                egui::FontFamily::Proportional,
                egui::FontFamily::Monospace,
                egui::FontFamily::Name("medium".into()),
                egui::FontFamily::Name("semibold".into()),
            ] {
                let font = egui::FontId::new(13.0, family);
                // epaint 0.36's has_glyph compares face keys, so it reports a false
                // negative when a real glyph shares the replacement face.
                // Check the rendered atlas glyph instead of that face-level predicate.
                let missing = fonts.layout_no_wrap("\u{10ffff}".into(), font.clone(), egui::Color32::WHITE);
                let missing_uv = missing.rows[0].glyphs[0].uv_rect;
                for ch in text.chars() {
                    let rendered = fonts.layout_no_wrap(ch.to_string(), font.clone(), egui::Color32::WHITE);
                    let uv = rendered.rows[0].glyphs[0].uv_rect;
                    assert!(uv.size.x > 0.0 && uv.size.y > 0.0, "empty {ch} in {font:?}");
                    assert_ne!((uv.min, uv.max), (missing_uv.min, missing_uv.max), "replacement glyph for {ch} in {font:?}");
                }
            }
        });
    }

    #[test]
    fn installed_japanese_fallback_is_available_in_all_ui_families() {
        if !installed('あ') {
            eprintln!("no Japanese system font installed; skipping glyph coverage");
            return;
        }
        assert_drawn("ja", "日本語コンポジションレイヤーエフェクト設定");
    }

    /// #394: Arabic, Hebrew, Indic and Thai names are drawn by an installed face in an English
    /// interface: the UI fonts were Inter and the CJK fallbacks only, so a layer named in Arabic
    /// showed as missing-glyph boxes.
    #[test]
    fn installed_script_fallbacks_draw_arabic_hebrew_indic_and_thai_names() {
        // Letters only: a vowel sign or virama on its own is drawn on a dotted circle.
        for text in ["طبقةمرحبا", "שכבה", "परत", "কলম", "ਕਲਮ", "કલમ", "கடல", "కలమ", "ಕಲಮ", "കലമ", "กขค"]
        {
            let Some(probe) = text.chars().next() else { continue };
            if !installed(probe) {
                eprintln!("no system font covers {text}; skipping its glyph coverage");
                continue;
            }
            assert_drawn("en", text);
        }
    }

    #[test]
    fn the_interface_asks_for_the_platforms_ui_faces() {
        // The face a window draws its own chrome with, which is not the one a composed frame
        // renders with: the two orders are deliberately separate.
        let ja = &["Meiryo UI", "Yu Gothic UI", "MS UI Gothic"][..];
        assert_eq!(interface_fallback_candidates("japanese-system", "ja"), ja);
        assert_eq!(interface_fallback_candidates("chinese-system", "ja"), ja);
        assert_eq!(interface_fallback_candidates("chinese-system", "zh-hans"), &["Microsoft YaHei UI"][..]);
        assert_eq!(interface_fallback_candidates("chinese-system", "zh-hant"), &["Microsoft JhengHei UI", "PMingLiU"][..]);
        // Every other script the install probes has no UI variant, so it keeps the shared order.
        for &(_, name) in SCRIPT_PROBES.iter() {
            assert!(interface_fallback_candidates(name, "en").is_empty(), "{name} asked for a UI face");
        }
    }

    /// The Georgian interface draws its Mkhedruli text with an installed Georgian face (Sylfaen,
    /// SF Georgian, Noto Sans Georgian…): Inter has no Georgian, so the labels showed as boxes.
    #[test]
    fn installed_georgian_fallback_draws_the_georgian_interface() {
        if !installed('ა') {
            eprintln!("no Georgian system font installed; skipping glyph coverage");
            return;
        }
        assert_drawn("ka", "კომპოზიციაფენასაკვანძოკადრიეფექტიპარამეტრები");
    }

    /// #1141: switching the interface to Simplified Chinese drew most labels as boxes. A Japanese face
    /// can cover the Han probe while lacking other catalog characters. The context must keep its
    /// own language policy even when another context installs Japanese fallbacks concurrently.
    #[test]
    fn simplified_chinese_interface_draws_simplified_hanzi_after_another_language() {
        let ctx = egui::Context::default();
        let tokens = super::Tokens::for_kind(super::ThemeKind::Dark);
        super::install(&ctx, &tokens, "en");
        super::install(&ctx, &tokens, "zh-hans");
        let mut out = ctx.run_ui(egui::RawInput::default(), |_| {});
        out.textures_delta.clear();
        let font = egui::FontId::new(13.0, egui::FontFamily::Proportional);
        let missing: String = ctx.fonts_mut(|fonts| {
            let replacement = fonts.layout_no_wrap("\u{10ffff}".into(), font.clone(), egui::Color32::WHITE).rows[0].glyphs[0].uv_rect;
            // Representative hanzi of the zh-hans catalog (选择, 删除, 图层, 设置, 导入, 页面).
            "选删层设导页"
                .chars()
                .filter(|ch| {
                    let uv = fonts.layout_no_wrap(ch.to_string(), font.clone(), egui::Color32::WHITE).rows[0].glyphs[0].uv_rect;
                    (uv.min, uv.max) == (replacement.min, replacement.max)
                })
                .collect()
        });
        // Asked after the layout: the coverage lookup itself loads a face with these hanzi.
        let drawable: String = missing.chars().filter(|&ch| installed(ch)).collect();
        assert!(drawable.is_empty(), "an installed face covers {drawable}, but the Simplified Chinese interface draws boxes");
    }

    /// egui shapes each run of one face: Arabic letters take their joined forms, and an Arabic
    /// word reads right to left (its first letter drawn rightmost).
    #[test]
    fn arabic_words_are_joined_and_read_right_to_left() {
        if !installed('م') {
            eprintln!("no Arabic system font installed; skipping");
            return;
        }
        let ctx = egui::Context::default();
        super::install(&ctx, &super::Tokens::for_kind(super::ThemeKind::Dark), "en");
        let mut out = ctx.run_ui(egui::RawInput::default(), |_| {});
        out.textures_delta.clear();
        ctx.fonts_mut(|fonts| {
            let font = egui::FontId::new(13.0, egui::FontFamily::Proportional);
            let missing = fonts.layout_no_wrap("\u{10ffff}".into(), font.clone(), egui::Color32::WHITE).rows[0].glyphs[0].uv_rect;
            let alone = fonts.layout_no_wrap("م".into(), font.clone(), egui::Color32::WHITE);
            let word = fonts.layout_no_wrap("مرحبا".into(), font, egui::Color32::WHITE);
            let glyphs = &word.rows[0].glyphs;
            let meem = glyphs.iter().find(|g| g.chr == 'م').expect("the meem is laid out");
            let alif = glyphs.iter().find(|g| g.chr == 'ا').expect("the alif is laid out");
            assert!(meem.pos.x > alif.pos.x, "the first letter is drawn rightmost: {} vs {}", meem.pos.x, alif.pos.x);
            let isolated = alone.rows[0].glyphs[0].uv_rect;
            for uv in [meem.uv_rect, isolated] {
                assert_ne!((uv.min, uv.max), (missing.min, missing.max), "a real glyph, not the replacement");
            }
            assert_ne!((meem.uv_rect.min, meem.uv_rect.max), (isolated.min, isolated.max), "the meem takes its initial form");
        });
    }

    /// A fallback face's glyphs sit on the row font's baseline, in every UI family and at every
    /// interface size. (#1309)
    ///
    /// `A` stands in for the row's baseline: epaint exposes none, but a capital without a descender
    /// has its ink bottom on the baseline, so its ink bottom *is* the row's baseline. A glyph with a
    /// descender (`g`) or a round one with an overshoot (`O`) would quietly stop holding, so keep `A`
    /// as the Latin half of this pair.
    ///
    /// Both glyphs of `Aあ` end on the baseline, so their ink bottoms agree up to the difference
    /// between the two outlines themselves - the kana reaches a little below the Latin letter in the
    /// Japanese faces. Without the correction the kana sits 3 pt high at 12 pt (measured), so the
    /// tolerance below separates "the correction is applied" from "it is missing"; it is not a claim
    /// about the outlines. Two pixels per point, because epaint truncates a glyph's box height to
    /// whole pixels, which would otherwise put up to a point of noise on the measurement.
    #[test]
    fn fallback_glyphs_sit_on_the_row_baseline() {
        if !installed('\u{3042}') {
            eprintln!("no Japanese system font installed; skipping the baseline check");
            return;
        }
        let ctx = egui::Context::default();
        ctx.set_pixels_per_point(2.0);
        super::install(&ctx, &super::Tokens::for_kind(super::ThemeKind::Dark), "ja");
        let mut out = ctx.run_ui(egui::RawInput::default(), |_| {});
        out.textures_delta.clear();
        let bottom = |g: &egui::epaint::text::Glyph| g.pos.y + g.uv_rect.offset.y + g.uv_rect.size.y;
        ctx.fonts_mut(|fonts| {
            for family in [
                egui::FontFamily::Proportional,
                egui::FontFamily::Monospace,
                egui::FontFamily::Name("medium".into()),
                egui::FontFamily::Name("semibold".into()),
            ] {
                for size in [12.0_f32, 15.0, 18.0] {
                    let font = egui::FontId::new(size, family.clone());
                    let galley = fonts.layout_no_wrap("A\u{3042}".into(), font, egui::Color32::WHITE);
                    let row = &galley.rows[0].glyphs;
                    assert_eq!(row.len(), 2, "A\u{3042} should shape to two glyphs");
                    let (latin, kana) = (bottom(&row[0]), bottom(&row[1]));
                    let drift = kana - latin;
                    // Without the correction the kana sits 3-5 pt *above* the Latin ink, so the sign is
                    // what tells the two apart. With it, the kana reaches a little *below* the Latin
                    // one, by however far the face's kana overshoot past the baseline - measured at
                    // 1.94 pt for Noto Sans CJK JP against Yu Gothic's 0.4 - plus up to half a point of
                    // whole-point rounding in `pos.y` for each glyph.
                    assert!(
                        drift > -0.5,
                        "the kana sits {drift:.2} pt above the Latin baseline at {size} pt in {family:?} (latin {latin}, kana {kana}); the fallback tweak is missing or wrong"
                    );
                    assert!(
                        drift < 2.5,
                        "the kana sits {drift:.2} pt below the Latin baseline at {size} pt in {family:?} (latin {latin}, kana {kana}); the fallback tweak is too large"
                    );
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The last `n` font names of `family`, in order.
    fn tail(family: &[String], n: usize) -> Vec<&str> {
        family.iter().rev().take(n).rev().map(String::as_str).collect()
    }

    #[test]
    fn cjk_fallbacks_follow_the_interface_language() {
        // A Chinese interface asks for the Han probe first, so Han text is drawn by a Chinese face
        // even when the operating system's locale is not Chinese.
        assert_eq!(cjk_locale("zh-hans"), "zh-hans");
        assert_eq!(cjk_locale("zh-hant"), "zh-hant");
        assert_eq!(cjk_locale("ja"), "ja");
        assert_eq!(cjk_locale("en"), "");
        assert_eq!(cjk_locale("system"), "");
        let mut chinese = FontDefinitions::default();
        let installed = install_script_fallbacks(&mut chinese, "zh-hans");
        let cjk = |names: &[String]| names.iter().filter(|n| matches!(n.as_str(), "chinese-system" | "japanese-system")).cloned().collect::<Vec<_>>();
        if cjk(&installed).is_empty() {
            eprintln!("no CJK font is installed: only the language plumbing can be checked here");
        } else {
            assert_eq!(installed.first().map(String::as_str), Some("chinese-system"), "the Han probe comes first");
            assert!(chinese.font_data.contains_key("chinese-system"));
        }
        let expected: Vec<&str> = installed.iter().map(String::as_str).collect();
        for (family, stack) in &chinese.families {
            assert_eq!(tail(stack, installed.len()), expected, "{family:?}");
        }
        // A Japanese interface keeps the kana probe first, as it was before the language existed.
        let mut japanese = FontDefinitions::default();
        let installed = install_script_fallbacks(&mut japanese, "ja");
        if let Some(first) = cjk(&installed).first() {
            assert_eq!(installed.first(), Some(first));
            assert_eq!(first, "japanese-system", "the kana probe comes first");
        }
        // And an English interface leaves the operating system's locale in charge.
        let mut english = FontDefinitions::default();
        install_script_fallbacks(&mut english, "en");
    }

    #[test]
    fn every_family_reaches_the_symbol_fallback() {
        let ctx = egui::Context::default();
        install(&ctx, &Tokens::for_kind(Default::default()), "en");
        ctx.run_ui(Default::default(), |_| {}).textures_delta.clear();
        for family in [FontFamily::Proportional, FontFamily::Monospace, FontFamily::Name("medium".into()), FontFamily::Name("semibold".into())] {
            let id = FontId::new(12.0, family);
            // Not has_glyph: it compares face keys, so Inter's `•` reads as missing when no face in
            // the stack has egui's `◻` and Inter's `?` becomes the replacement, as happens once
            // another test has loaded a system face without `◻` (the fallback tries loaded faces first).
            let uv = |s: String| ctx.fonts_mut(|f| f.layout_no_wrap(s, id.clone(), egui::Color32::WHITE).rows[0].glyphs[0].uv_rect);
            let missing = uv("\u{10ffff}".into());
            // Every chrome symbol the panels, menus, shortcut labels and buttons draw.
            for c in "✔▣✓⚠▶◀▸●○…×›•·−←→↑↓⌘⇧⌥⌃".chars() {
                let drawn = uv(c.to_string());
                assert_ne!((drawn.min, drawn.max), (missing.min, missing.max), "{c} has no glyph in {id:?}");
            }
        }
    }

    #[test]
    fn hangul_gets_a_face_when_one_is_installed() {
        use effectcraft_text::fonts;
        let mut fonts_def = FontDefinitions::default();
        install_script_fallbacks(&mut fonts_def, "en");
        let base = fonts::resolve("Inter", "Regular").face;
        let id = fonts::fallback_for('한', base);
        let face = fonts::face(id);
        if !face.has_char('한') {
            eprintln!("no Hangul font is installed: nothing to check here");
            return;
        }
        let Some(font) = face.font() else { return };
        let bytes = font.data().as_bytes();
        let covered = fonts_def.font_data.values().any(|d| d.bytes() == bytes && d.index == face.info.index);
        assert!(covered, "a face covering Hangul must be registered with egui");
    }

    #[test]
    fn a_fallback_face_is_registered_at_most_once() {
        let mut fonts = FontDefinitions::default();
        let installed = install_script_fallbacks(&mut fonts, "zh-hant");
        let mut unique = installed.clone();
        unique.sort();
        unique.dedup();
        assert_eq!(unique.len(), installed.len(), "no name may be registered twice: {installed:?}");
        let faces: Vec<_> = installed
            .iter()
            .map(|name| {
                let data = fonts.font_data.get(name).unwrap_or_else(|| panic!("{name} is missing its font data"));
                (data.bytes(), data.index)
            })
            .collect();
        for (i, a) in faces.iter().enumerate() {
            assert!(!faces[i + 1..].contains(a), "{} shares its face with a later fallback: {installed:?}", installed[i]);
        }
        // Leave the operating system's locale behind for whatever runs next.
        install_script_fallbacks(&mut fonts, "system");
    }
}
