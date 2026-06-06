//! Typography — font registration and composite type styles.
//!
//! Registers the bundled Iosevka faces (UI mono) + IosevkaTerm (code) under per-weight
//! named families, plus Phosphor **Light** icons via `egui-phosphor`. Exposes composite
//! [`TypeStyle`] tokens (family + size + line-height + tracking) for the named roles
//! (`display`/`h1`/…/`code`). Sizes and leadings come from [`core`](crate::tokens::core).

use crate::tokens::core;
use egui::{FontData, FontDefinitions, FontFamily, FontId};

// Registration keys — one per vendored weight.
const SANS_LIGHT: &str = "iosevka-light";
const SANS_REGULAR: &str = "iosevka";
const SANS_MEDIUM: &str = "iosevka-medium";
const SANS_SEMIBOLD: &str = "iosevka-semibold";
const SANS_BOLD: &str = "iosevka-bold";
const MONO_REGULAR: &str = "iosevka-term";
const MONO_BOLD: &str = "iosevka-term-bold";

/// Font weight — selects which registered Iosevka face a style uses.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Weight {
    Light,
    Regular,
    Medium,
    SemiBold,
    Bold,
}

fn sans(weight: Weight) -> FontFamily {
    FontFamily::Name(
        match weight {
            Weight::Light => SANS_LIGHT,
            Weight::Regular => SANS_REGULAR,
            Weight::Medium => SANS_MEDIUM,
            Weight::SemiBold => SANS_SEMIBOLD,
            Weight::Bold => SANS_BOLD,
        }
        .into(),
    )
}

fn mono(bold: bool) -> FontFamily {
    FontFamily::Name(if bold { MONO_BOLD } else { MONO_REGULAR }.into())
}

/// Register all bundled faces + Phosphor icons into a [`FontDefinitions`].
///
/// Sets the default Proportional stack to Iosevka Regular (Phosphor Light as icon
/// fallback) and Monospace to IosevkaTerm; also registers each weight under its own
/// named family so [`TypeStyle`] can target a specific face.
pub fn register(fonts: &mut FontDefinitions) {
    let faces: [(&str, &[u8]); 7] = [
        (
            SANS_LIGHT,
            include_bytes!("../../assets/fonts/Iosevka-Light.ttf"),
        ),
        (
            SANS_REGULAR,
            include_bytes!("../../assets/fonts/Iosevka-Regular.ttf"),
        ),
        (
            SANS_MEDIUM,
            include_bytes!("../../assets/fonts/Iosevka-Medium.ttf"),
        ),
        (
            SANS_SEMIBOLD,
            include_bytes!("../../assets/fonts/Iosevka-SemiBold.ttf"),
        ),
        (
            SANS_BOLD,
            include_bytes!("../../assets/fonts/Iosevka-Bold.ttf"),
        ),
        (
            MONO_REGULAR,
            include_bytes!("../../assets/fonts/IosevkaTerm-Regular.ttf"),
        ),
        (
            MONO_BOLD,
            include_bytes!("../../assets/fonts/IosevkaTerm-Bold.ttf"),
        ),
    ];

    for (name, bytes) in faces {
        fonts
            .font_data
            .insert(name.to_owned(), FontData::from_static(bytes).into());
        // A named family per weight, so a TypeStyle can target this exact face.
        fonts
            .families
            .entry(FontFamily::Name(name.into()))
            .or_default()
            .insert(0, name.to_owned());
    }

    // Default stacks: Iosevka Regular for proportional text, IosevkaTerm for monospace.
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, SANS_REGULAR.to_owned());
    fonts
        .families
        .entry(FontFamily::Monospace)
        .or_default()
        .insert(0, MONO_REGULAR.to_owned());

    // Phosphor Light — registers the "phosphor" face and appends it to Proportional only.
    egui_phosphor::add_to_fonts(fonts, egui_phosphor::Variant::Light);

    // Append Phosphor as an icon fallback to every named face + Monospace, so inline
    // icons resolve regardless of which TypeStyle (named family) renders them — not just
    // the default Proportional stack.
    const ALL_FACES: [&str; 7] = [
        SANS_LIGHT,
        SANS_REGULAR,
        SANS_MEDIUM,
        SANS_SEMIBOLD,
        SANS_BOLD,
        MONO_REGULAR,
        MONO_BOLD,
    ];
    for key in ALL_FACES {
        fonts
            .families
            .entry(FontFamily::Name(key.into()))
            .or_default()
            .push("phosphor".to_owned());
    }
    fonts
        .families
        .entry(FontFamily::Monospace)
        .or_default()
        .push("phosphor".to_owned());
}

/// A composite typography token — family (incl. weight), size, resolved line-height
/// (px) and letter tracking. Build an [`egui::FontId`] with [`TypeStyle::font_id`];
/// the line-height/tracking are applied by the text atom when laying out a galley.
#[derive(Clone, Debug)]
pub struct TypeStyle {
    pub family: FontFamily,
    pub size: f32,
    pub line_height: f32,
    pub tracking: f32,
}

impl TypeStyle {
    /// The [`FontId`] (family + size) for this style.
    pub fn font_id(&self) -> FontId {
        FontId::new(self.size, self.family.clone())
    }
}

fn style(family: FontFamily, size: f32, leading: f32, tracking: f32) -> TypeStyle {
    TypeStyle {
        family,
        size,
        line_height: size * leading,
        tracking,
    }
}

// ── Named roles (classic weight emphasis) ────────────────────────────────────

/// Largest title — Bold 30.
pub fn display() -> TypeStyle {
    style(
        sans(Weight::Bold),
        core::TEXT_3XL,
        core::LEADING_TIGHT,
        core::TRACKING_WIDE,
    )
}
/// H1 — SemiBold 24.
pub fn h1() -> TypeStyle {
    style(
        sans(Weight::SemiBold),
        core::TEXT_2XL,
        core::LEADING_TIGHT,
        core::TRACKING_WIDE,
    )
}
/// H2 — SemiBold 20.
pub fn h2() -> TypeStyle {
    style(
        sans(Weight::SemiBold),
        core::TEXT_XL,
        core::LEADING_TIGHT,
        core::TRACKING_WIDE,
    )
}
/// Section heading — SemiBold 16.
pub fn heading() -> TypeStyle {
    style(
        sans(Weight::SemiBold),
        core::TEXT_LG,
        core::LEADING_TIGHT,
        core::TRACKING_WIDE,
    )
}
/// Body — Regular 14.
pub fn body() -> TypeStyle {
    style(
        sans(Weight::Regular),
        core::TEXT_BASE,
        core::LEADING_NORMAL,
        core::TRACKING_NORMAL,
    )
}
/// Emphasized body — Medium 14.
pub fn body_strong() -> TypeStyle {
    style(
        sans(Weight::Medium),
        core::TEXT_BASE,
        core::LEADING_NORMAL,
        core::TRACKING_NORMAL,
    )
}
/// Label — Medium 13.
pub fn label() -> TypeStyle {
    style(
        sans(Weight::Medium),
        core::TEXT_SM,
        core::LEADING_NORMAL,
        core::TRACKING_NORMAL,
    )
}
/// Caption / small — Regular 12.
pub fn caption() -> TypeStyle {
    style(
        sans(Weight::Regular),
        core::TEXT_XS,
        core::LEADING_NORMAL,
        core::TRACKING_NORMAL,
    )
}
/// Inline code — IosevkaTerm Regular 13.
pub fn code() -> TypeStyle {
    style(
        mono(false),
        core::TEXT_SM,
        core::LEADING_NORMAL,
        core::TRACKING_NORMAL,
    )
}
/// Keyboard key — IosevkaTerm Bold 12 (mono Medium not vendored; Bold reads as a key cap).
pub fn kbd() -> TypeStyle {
    style(
        mono(true),
        core::TEXT_XS,
        core::LEADING_NORMAL,
        core::TRACKING_NORMAL,
    )
}

/// Font for an icon glyph at `size`. Phosphor glyphs (PUA codepoints) resolve via the
/// proportional stack's icon fallback. Atoms call this instead of building a `FontId`.
pub fn icon_font(size: f32) -> FontId {
    FontId::new(size, FontFamily::Proportional)
}
