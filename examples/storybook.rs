//! Storybook — organized gallery for ouroboros-ui, built with the DS itself.
//!
//! The chrome dogfoods the atoms: nav + toggle are [`Button`]s, every text run is a
//! [`Text`]/[`Heading`], separators are [`Divider`]s, and all spacing/radius/color come
//! from tokens. The only primitive painting is the token swatches/bars/shapes — the demo
//! *of* a token must draw the token. Toggle ◐/◑ to compare light and dark.
//!
//! Run: `cargo run --example storybook`

use egui::{vec2, Align, Color32, CornerRadius, Layout, Sense, Stroke, StrokeKind, Ui};
use egui_phosphor::light;
use ouroboros_ui::atoms::{
    Avatar, AvatarSize, Badge, BadgeVariant, Button, ButtonVariant, Checkbox, Divider, Heading,
    Icon, Input, Radio, Spinner, Switch, Text, TextRole, Tooltip,
};
use ouroboros_ui::auto_layout::{AutoLayout, CrossAlign, MainAlign};
use ouroboros_ui::theme::typography;
use ouroboros_ui::tokens::{core, layout};
use ouroboros_ui::{Mode, Theme};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Page {
    Colors,
    Typography,
    Spacing,
    Radius,
    Shadows,
    Sizing,
    Opacity,
    Motion,
    LayoutTokens,
    AutoLayoutDemo,
    Text,
    Heading,
    Icon,
    Divider,
    Button,
    Checkbox,
    Radio,
    Switch,
    Input,
    Badge,
    Spinner,
    Avatar,
    Tooltip,
}

impl Page {
    fn label(self) -> &'static str {
        match self {
            Page::Colors => "Colors",
            Page::Typography => "Typography",
            Page::Spacing => "Spacing",
            Page::Radius => "Radius",
            Page::Shadows => "Shadows",
            Page::Sizing => "Sizing",
            Page::Opacity => "Opacity & overlays",
            Page::Motion => "Motion",
            Page::LayoutTokens => "Layout & panels",
            Page::AutoLayoutDemo => "Auto-layout (Figma)",
            Page::Text => "Text",
            Page::Heading => "Heading",
            Page::Icon => "Icon",
            Page::Divider => "Divider",
            Page::Button => "Button",
            Page::Checkbox => "Checkbox",
            Page::Radio => "Radio",
            Page::Switch => "Switch",
            Page::Input => "Input",
            Page::Badge => "Badge",
            Page::Spinner => "Spinner",
            Page::Avatar => "Avatar",
            Page::Tooltip => "Tooltip",
        }
    }
}

const NAV: &[(&str, &[Page])] = &[
    (
        "TOKENS",
        &[
            Page::Colors,
            Page::Typography,
            Page::Spacing,
            Page::Radius,
            Page::Shadows,
            Page::Sizing,
            Page::Opacity,
            Page::Motion,
        ],
    ),
    ("LAYOUT", &[Page::LayoutTokens, Page::AutoLayoutDemo]),
    (
        "ATOMS",
        &[
            Page::Text,
            Page::Heading,
            Page::Icon,
            Page::Divider,
            Page::Button,
            Page::Checkbox,
            Page::Radio,
            Page::Switch,
            Page::Input,
            Page::Badge,
            Page::Spinner,
            Page::Avatar,
            Page::Tooltip,
        ],
    ),
];

fn main() -> eframe::Result<()> {
    let mut installed = false;
    let mut mode = Mode::Dark;
    let mut page = Page::Colors;
    eframe::run_ui_native(
        "ouroboros-ui storybook",
        eframe::NativeOptions::default(),
        move |ui, _frame| {
            if !installed {
                // `set_fonts` only takes effect next frame — install, then skip this frame.
                Theme::install(ui.ctx(), mode);
                installed = true;
                ui.ctx().request_repaint();
                return;
            }
            let theme = Theme::get(ui);
            ui.painter()
                .rect_filled(ui.clip_rect(), 0.0, theme.background);

            egui::Panel::top("header")
                .frame(header_frame(&theme))
                .show_inside(ui, |ui| {
                    ui.horizontal(|ui| {
                        Heading::new("ouroboros-ui").h2().show(ui);
                        ui.add_space(core::SPACE_3);
                        Text::new("design system").muted().show(ui);
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            let label = match mode {
                                Mode::Dark => "◐ Dark",
                                Mode::Light => "◑ Light",
                            };
                            if Button::new(label)
                                .ghost()
                                .sm()
                                .id_source("toggle")
                                .show(ui)
                                .clicked()
                            {
                                mode = match mode {
                                    Mode::Dark => Mode::Light,
                                    Mode::Light => Mode::Dark,
                                };
                                Theme::apply(ui.ctx(), mode);
                                ui.ctx().request_repaint();
                            }
                        });
                    });
                });

            egui::Panel::left("nav")
                .resizable(false)
                .exact_size(190.0)
                .frame(nav_frame(&theme))
                .show_inside(ui, |ui| {
                    egui::ScrollArea::vertical()
                        .id_salt("nav_scroll")
                        .show(ui, |ui| nav(ui, &theme, &mut page));
                });

            egui::CentralPanel::default()
                .frame(content_frame(&theme))
                .show_inside(ui, |ui| {
                    egui::ScrollArea::vertical()
                        .id_salt("content_scroll")
                        .show(ui, |ui| {
                            ui.set_max_width(760.0);
                            Heading::new(page.label()).h1().show(ui);
                            ui.add_space(core::SPACE_3);
                            Divider::horizontal().show(ui);
                            ui.add_space(core::SPACE_5);
                            render_page(ui, &theme, page);
                            ui.add_space(core::SPACE_8);
                        });
                });
        },
    )
}

// ── Frames (token-driven) ─────────────────────────────────────────────────────

fn header_frame(theme: &Theme) -> egui::Frame {
    egui::Frame::default()
        .fill(theme.background)
        .inner_margin(egui::Margin::symmetric(
            core::SPACE_5 as i8,
            core::SPACE_3 as i8,
        ))
        .stroke(Stroke::new(core::BORDER_THIN, theme.border))
}

fn nav_frame(theme: &Theme) -> egui::Frame {
    egui::Frame::default()
        .fill(theme.card)
        .inner_margin(core::SPACE_3)
        .stroke(Stroke::new(core::BORDER_THIN, theme.border))
}

fn content_frame(theme: &Theme) -> egui::Frame {
    egui::Frame::default()
        .fill(theme.background)
        .inner_margin(core::SPACE_6)
}

// ── Nav (dogfoods Button + Text) ──────────────────────────────────────────────

fn nav(ui: &mut Ui, theme: &Theme, page: &mut Page) {
    for (category, pages) in NAV {
        ui.add_space(core::SPACE_3);
        Text::new(*category).caption().muted().show(ui);
        ui.add_space(core::SPACE_1);
        for &p in *pages {
            let variant = if *page == p {
                ButtonVariant::Secondary
            } else {
                ButtonVariant::Ghost
            };
            if Button::new(p.label())
                .variant(variant)
                .sm()
                .id_source(("nav", p.label()))
                .show(ui)
                .clicked()
            {
                *page = p;
            }
        }
    }
    let _ = theme;
}

// ── Pages ─────────────────────────────────────────────────────────────────────

fn render_page(ui: &mut Ui, theme: &Theme, page: Page) {
    match page {
        Page::Colors => page_colors(ui, theme),
        Page::Typography => page_typography(ui, theme),
        Page::Spacing => page_spacing(ui, theme),
        Page::Radius => page_radius(ui, theme),
        Page::Shadows => page_shadows(ui, theme),
        Page::Sizing => page_sizing(ui, theme),
        Page::Opacity => page_opacity(ui, theme),
        Page::Motion => page_motion(ui, theme),
        Page::LayoutTokens => page_layout_tokens(ui, theme),
        Page::AutoLayoutDemo => page_auto_layout(ui, theme),
        Page::Text => page_text(ui, theme),
        Page::Heading => page_heading(ui, theme),
        Page::Icon => page_icon(ui, theme),
        Page::Divider => page_divider(ui, theme),
        Page::Button => page_button(ui, theme),
        Page::Checkbox => page_checkbox(ui, theme),
        Page::Radio => page_radio(ui, theme),
        Page::Switch => page_switch(ui, theme),
        Page::Input => page_input(ui, theme),
        Page::Badge => page_badge(ui, theme),
        Page::Spinner => page_spinner(ui, theme),
        Page::Avatar => page_avatar(ui, theme),
        Page::Tooltip => page_tooltip(ui, theme),
    }
}

fn page_switch(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "States");
    let id = egui::Id::new("demo_switch");
    let mut on = ui.data(|d| d.get_temp::<bool>(id).unwrap_or(true));
    ui.horizontal(|ui| {
        Switch::new(&mut on).show(ui);
        ui.add_space(core::SPACE_2);
        Text::new(if on { "on" } else { "off" }).muted().show(ui);
    });
    ui.data_mut(|d| d.insert_temp(id, on));
    ui.add_space(core::SPACE_3);
    let mut t = true;
    let mut f = false;
    ui.horizontal(|ui| {
        Switch::new(&mut t).disabled().show(ui);
        ui.add_space(core::SPACE_2);
        Switch::new(&mut f).disabled().show(ui);
        ui.add_space(core::SPACE_2);
        Text::new("disabled").muted().show(ui);
    });
}

fn page_input(ui: &mut Ui, _theme: &Theme) {
    fn field(ui: &mut Ui, key: &str, render: impl FnOnce(&mut Ui, &mut String)) {
        let id = egui::Id::new(key);
        let mut s = ui.data(|d| d.get_temp::<String>(id).unwrap_or_default());
        ui.allocate_ui(vec2(320.0, core::CONTROL_MD), |ui| render(ui, &mut s));
        ui.data_mut(|d| d.insert_temp(id, s));
        ui.add_space(core::SPACE_3);
    }
    caption(ui, "Default / placeholder");
    field(ui, "in_a", |ui, s| {
        Input::new(s).placeholder("Type here…").show(ui);
    });
    caption(ui, "Error");
    field(ui, "in_b", |ui, s| {
        Input::new(s).placeholder("Required").error(true).show(ui);
    });
    caption(ui, "Disabled");
    field(ui, "in_c", |ui, s| {
        Input::new(s).placeholder("Disabled").disabled().show(ui);
    });
}

fn page_badge(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "shadcn variants");
    ui.horizontal_wrapped(|ui| {
        for (name, v) in [
            ("Default", BadgeVariant::Default),
            ("Secondary", BadgeVariant::Secondary),
            ("Destructive", BadgeVariant::Destructive),
            ("Outline", BadgeVariant::Outline),
            ("Ghost", BadgeVariant::Ghost),
            ("Link", BadgeVariant::Link),
        ] {
            Badge::new(name).variant(v).show(ui);
            ui.add_space(core::SPACE_2);
        }
    });
    subhead(ui, "Status (domain) — with dot");
    ui.horizontal_wrapped(|ui| {
        for (name, v) in [
            ("Success", BadgeVariant::Success),
            ("Warning", BadgeVariant::Warning),
            ("Info", BadgeVariant::Info),
        ] {
            Badge::new(name).variant(v).dot().show(ui);
            ui.add_space(core::SPACE_2);
        }
    });
}

fn page_spinner(ui: &mut Ui, theme: &Theme) {
    caption(ui, "Sizes + color");
    ui.horizontal(|ui| {
        Spinner::new().sm().show(ui);
        ui.add_space(core::SPACE_4);
        Spinner::new().show(ui);
        ui.add_space(core::SPACE_4);
        Spinner::new().lg().show(ui);
        ui.add_space(core::SPACE_4);
        Spinner::new().lg().color(theme.primary).show(ui);
    });
}

fn page_avatar(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "Sizes");
    ui.horizontal(|ui| {
        Avatar::new("ab").size(AvatarSize::Sm).show(ui);
        ui.add_space(core::SPACE_3);
        Avatar::new("cd").show(ui);
        ui.add_space(core::SPACE_3);
        Avatar::new("ef").size(AvatarSize::Lg).show(ui);
    });
}

fn page_tooltip(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "Hover the button");
    let resp = Button::new("Hover me")
        .secondary()
        .id_source("tt_btn")
        .show(ui);
    Tooltip::new("A token-styled tooltip").show(resp);
}

fn page_checkbox(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "States");
    let id = egui::Id::new("demo_check_a");
    let mut a = ui.data(|d| d.get_temp::<bool>(id).unwrap_or(true));
    Checkbox::new(&mut a).label("Accept terms").show(ui);
    ui.data_mut(|d| d.insert_temp(id, a));
    ui.add_space(core::SPACE_2);
    let mut off = false;
    Checkbox::new(&mut off).label("Unchecked").show(ui);
    ui.add_space(core::SPACE_2);
    let mut on = true;
    Checkbox::new(&mut on)
        .label("Disabled checked")
        .disabled()
        .show(ui);
    let mut un = false;
    Checkbox::new(&mut un).label("Disabled").disabled().show(ui);
}

fn page_radio(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "Single-select group (consumer-managed)");
    let id = egui::Id::new("demo_radio");
    let mut sel = ui.data(|d| d.get_temp::<usize>(id).unwrap_or(0));
    for (i, label) in ["Option A", "Option B", "Option C"].iter().enumerate() {
        if Radio::new(sel == i)
            .label(*label)
            .id_source(("r", i))
            .show(ui)
            .clicked()
        {
            sel = i;
        }
        ui.add_space(core::SPACE_1);
    }
    ui.data_mut(|d| d.insert_temp(id, sel));
    ui.add_space(core::SPACE_2);
    Radio::new(true)
        .label("Disabled selected")
        .disabled()
        .show(ui);
    Radio::new(false).label("Disabled").disabled().show(ui);
}

fn page_colors(ui: &mut Ui, theme: &Theme) {
    caption(ui, "Semantic — surfaces & text");
    swatch_row(
        ui,
        &[
            ("background", theme.background),
            ("card", theme.card),
            ("popover", theme.popover),
            ("muted", theme.muted),
            ("border", theme.border),
            ("border_strong", theme.border_strong),
        ],
        theme,
    );
    swatch_row(
        ui,
        &[
            ("foreground", theme.foreground),
            ("muted_foreground", theme.muted_foreground),
            ("disabled_fg", theme.disabled_foreground),
            ("primary", theme.primary),
            ("ring", theme.ring),
        ],
        theme,
    );
    subhead(ui, "Core — zinc ramp");
    swatch_row(
        ui,
        &[
            ("50", core::ZINC_50),
            ("100", core::ZINC_100),
            ("200", core::ZINC_200),
            ("300", core::ZINC_300),
            ("400", core::ZINC_400),
            ("500", core::ZINC_500),
            ("600", core::ZINC_600),
            ("700", core::ZINC_700),
            ("800", core::ZINC_800),
            ("900", core::ZINC_900),
            ("950", core::ZINC_950),
        ],
        theme,
    );
    subhead(ui, "Status");
    swatch_row(
        ui,
        &[
            ("success", theme.success),
            ("warning", theme.warning),
            ("error", theme.error),
            ("info", theme.info),
            ("neutral", theme.neutral),
        ],
        theme,
    );
}

fn page_typography(ui: &mut Ui, _theme: &Theme) {
    const SAMPLE: &str = "Ouroboros — the serpent renews";
    type_row(ui, "display", |ui| {
        Heading::new(SAMPLE).display().show(ui);
    });
    type_row(ui, "h1", |ui| {
        Heading::new(SAMPLE).h1().show(ui);
    });
    type_row(ui, "h2", |ui| {
        Heading::new(SAMPLE).h2().show(ui);
    });
    type_row(ui, "heading", |ui| {
        Heading::new(SAMPLE).heading().show(ui);
    });
    for (name, role) in [
        ("body", TextRole::Body),
        ("body_strong", TextRole::BodyStrong),
        ("label", TextRole::Label),
        ("caption", TextRole::Caption),
        ("code", TextRole::Code),
        ("kbd", TextRole::Kbd),
    ] {
        type_row(ui, name, move |ui| {
            Text::new(SAMPLE).role(role).show(ui);
        });
    }
}

fn page_spacing(ui: &mut Ui, theme: &Theme) {
    for (name, w) in [
        ("SPACE_1", core::SPACE_1),
        ("SPACE_2", core::SPACE_2),
        ("SPACE_3", core::SPACE_3),
        ("SPACE_4", core::SPACE_4),
        ("SPACE_5", core::SPACE_5),
        ("SPACE_6", core::SPACE_6),
        ("SPACE_8", core::SPACE_8),
        ("SPACE_10", core::SPACE_10),
        ("SPACE_12", core::SPACE_12),
    ] {
        ui.horizontal(|ui| {
            name_cell(ui, name);
            let (rect, _) = ui.allocate_exact_size(vec2(w, core::SPACE_3), Sense::hover());
            ui.painter()
                .rect_filled(rect, CornerRadius::same(2), theme.primary);
            Text::new(format!("{w}")).caption().muted().show(ui);
        });
    }
}

fn page_radius(ui: &mut Ui, theme: &Theme) {
    ui.horizontal(|ui| {
        for (name, r) in [
            ("SM 4", core::RADIUS_SM),
            ("MD 6", core::RADIUS_MD),
            ("LG 8", core::RADIUS_LG),
            ("XL 12", core::RADIUS_XL),
        ] {
            ui.vertical(|ui| {
                let (rect, _) = ui.allocate_exact_size(vec2(56.0, 56.0), Sense::hover());
                ui.painter()
                    .rect_filled(rect, CornerRadius::same(r as u8), theme.muted);
                ui.painter().rect_stroke(
                    rect,
                    CornerRadius::same(r as u8),
                    Stroke::new(core::BORDER_THIN, theme.border_strong),
                    StrokeKind::Inside,
                );
                ui.add_space(core::SPACE_1);
                Text::new(name).caption().muted().show(ui);
            });
            ui.add_space(core::SPACE_4);
        }
    });
}

fn page_shadows(ui: &mut Ui, _theme: &Theme) {
    caption(
        ui,
        "shown on a light panel — dark-theme shadows are subtle by design",
    );
    ui.add_space(core::SPACE_2);
    egui::Frame::default()
        .fill(core::ZINC_200)
        .corner_radius(CornerRadius::same(core::RADIUS_LG as u8))
        .inner_margin(core::SPACE_6)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                for (name, shadow) in [
                    ("sm", core::SHADOW_SM),
                    ("md", core::SHADOW_MD),
                    ("lg", core::SHADOW_LG),
                ] {
                    egui::Frame::default()
                        .fill(core::ZINC_50)
                        .corner_radius(CornerRadius::same(core::RADIUS_LG as u8))
                        .shadow(shadow)
                        .inner_margin(core::SPACE_4)
                        .show(ui, |ui| {
                            Text::new(format!("shadow {name}"))
                                .label()
                                .color(core::ZINC_900)
                                .show(ui);
                        });
                    ui.add_space(core::SPACE_8);
                }
            });
        });
}

fn page_sizing(ui: &mut Ui, theme: &Theme) {
    caption(ui, "Control heights");
    ui.horizontal(|ui| {
        for (name, h) in [
            ("SM 26", core::CONTROL_SM),
            ("MD 32", core::CONTROL_MD),
            ("LG 38", core::CONTROL_LG),
        ] {
            let (rect, _) = ui.allocate_exact_size(vec2(96.0, h), Sense::hover());
            ui.painter()
                .rect_filled(rect, CornerRadius::same(core::RADIUS_MD as u8), theme.muted);
            ui.painter().rect_stroke(
                rect,
                CornerRadius::same(core::RADIUS_MD as u8),
                Stroke::new(core::BORDER_THIN, theme.border_strong),
                StrokeKind::Inside,
            );
            ui.painter().text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                name,
                typography::caption().font_id(),
                theme.muted_foreground,
            );
            ui.add_space(core::SPACE_3);
        }
    });
    subhead(ui, "Icon sizes");
    ui.horizontal(|ui| {
        for (sz, lbl) in [
            (core::ICON_SM, "14"),
            (core::ICON_MD, "16"),
            (core::ICON_LG, "20"),
            (core::ICON_XL, "24"),
        ] {
            ui.vertical(|ui| {
                Icon::new(light::CUBE).size(sz).show(ui);
                Text::new(lbl).caption().muted().show(ui);
            });
            ui.add_space(core::SPACE_4);
        }
    });
}

fn page_opacity(ui: &mut Ui, theme: &Theme) {
    ui.horizontal(|ui| {
        Text::new("enabled").show(ui);
        ui.add_space(core::SPACE_4);
        Text::new("disabled 0.5")
            .color(theme.foreground.gamma_multiply(core::OPACITY_DISABLED))
            .show(ui);
    });
    ui.add_space(core::SPACE_3);
    ui.horizontal(|ui| {
        overlay_tile(ui, "scrim 0.6", core::ZINC_200, core::SCRIM);
        ui.add_space(core::SPACE_4);
        overlay_tile(ui, "hover", theme.muted, theme.hover_overlay);
        ui.add_space(core::SPACE_4);
        overlay_tile(ui, "press", theme.muted, theme.press_overlay);
    });
}

fn page_motion(ui: &mut Ui, theme: &Theme) {
    let t = ui.input(|i| i.time) as f32;
    for (name, dur, easing) in [
        (
            "fast 0.10  EaseOut",
            core::DURATION_FAST,
            core::Easing::EaseOut,
        ),
        (
            "normal 0.18  EaseOut",
            core::DURATION_NORMAL,
            core::Easing::EaseOut,
        ),
        (
            "slow 0.30  EaseInOut",
            core::DURATION_SLOW,
            core::Easing::EaseInOut,
        ),
    ] {
        let period = dur * 2.0 + 0.6;
        let phase = (t % period) / period;
        let leg = if phase < 0.5 {
            phase * 2.0
        } else {
            1.0 - (phase - 0.5) * 2.0
        };
        let e = easing.apply(leg.clamp(0.0, 1.0));
        ui.horizontal(|ui| {
            ui.allocate_ui(vec2(180.0, 16.0), |ui| {
                Text::new(name).code().muted().show(ui);
            });
            let (track, _) = ui.allocate_exact_size(vec2(240.0, 16.0), Sense::hover());
            ui.painter()
                .rect_filled(track, CornerRadius::same(8), theme.muted);
            let r = 6.0;
            let x = track.left() + r + e * (track.width() - 2.0 * r);
            ui.painter()
                .circle_filled(egui::pos2(x, track.center().y), r, theme.primary);
        });
    }
    ui.ctx().request_repaint();
}

fn page_layout_tokens(ui: &mut Ui, theme: &Theme) {
    caption(ui, "Panel widths");
    for (name, w) in [
        ("SIDEBAR 240", layout::SIDEBAR_WIDTH),
        ("INSPECTOR 300", layout::INSPECTOR_WIDTH),
        ("PANEL_MIN 180", layout::PANEL_MIN),
        ("PANEL_MAX 480", layout::PANEL_MAX),
    ] {
        ui.horizontal(|ui| {
            let (rect, _) = ui.allocate_exact_size(vec2(w, core::SPACE_3), Sense::hover());
            ui.painter()
                .rect_filled(rect, CornerRadius::same(2), theme.border_strong);
            Text::new(name).code().muted().show(ui);
        });
    }
    subhead(ui, "Breakpoints");
    Text::new("compact <720  ·  normal <1024  ·  wide ≥1440")
        .code()
        .muted()
        .show(ui);
}

fn page_auto_layout(ui: &mut Ui, theme: &Theme) {
    let (p, pf) = (theme.primary, theme.primary_foreground);
    let (mu, fg) = (theme.muted, theme.foreground);

    caption(ui, "gap Auto — space-between");
    al_box(ui, theme, |ui| {
        AutoLayout::horizontal()
            .gap_auto()
            .pad(core::SPACE_2)
            .cross_align(CrossAlign::Center)
            .hug(|ui| chip(ui, "A", p, pf))
            .hug(|ui| chip(ui, "B", mu, fg))
            .hug(|ui| chip(ui, "C", mu, fg))
            .show(ui);
    });
    caption(ui, "Fill spacer — left group · spacer · right action");
    al_box(ui, theme, |ui| {
        AutoLayout::horizontal()
            .gap(core::SPACE_2)
            .pad(core::SPACE_2)
            .cross_align(CrossAlign::Center)
            .hug(|ui| chip(ui, "File", mu, fg))
            .hug(|ui| chip(ui, "Edit", mu, fg))
            .fill(|_ui| {})
            .hug(|ui| chip(ui, "Run", p, pf))
            .show(ui);
    });
    caption(ui, "main_align Center");
    al_box(ui, theme, |ui| {
        AutoLayout::horizontal()
            .gap(core::SPACE_2)
            .pad(core::SPACE_2)
            .main_align(MainAlign::Center)
            .cross_align(CrossAlign::Center)
            .hug(|ui| chip(ui, "ok", p, pf))
            .hug(|ui| chip(ui, "cancel", mu, fg))
            .show(ui);
    });
    caption(ui, "vertical stack — gap + padding");
    al_box(ui, theme, |ui| {
        AutoLayout::vertical()
            .gap(core::SPACE_2)
            .pad(core::SPACE_3)
            .cross_align(CrossAlign::Start)
            .hug(|ui| chip(ui, "row one", mu, fg))
            .hug(|ui| chip(ui, "row two", mu, fg))
            .hug(|ui| chip(ui, "row three", p, pf))
            .show(ui);
    });
}

fn page_text(ui: &mut Ui, theme: &Theme) {
    for (name, role) in [
        ("body", TextRole::Body),
        ("body_strong", TextRole::BodyStrong),
        ("label", TextRole::Label),
        ("caption", TextRole::Caption),
        ("code", TextRole::Code),
        ("kbd", TextRole::Kbd),
    ] {
        ui.horizontal(|ui| {
            name_cell(ui, name);
            Text::new("Ouroboros — the serpent renews")
                .role(role)
                .show(ui);
        });
    }
    subhead(ui, "Color & decoration");
    ui.horizontal(|ui| {
        Text::new("muted").muted().show(ui);
        ui.add_space(core::SPACE_4);
        Text::new("success token").color(theme.success).show(ui);
        ui.add_space(core::SPACE_4);
        Text::new("underlined").underline().show(ui);
    });
}

fn page_heading(ui: &mut Ui, _theme: &Theme) {
    Heading::new("Display").display().show(ui);
    ui.add_space(core::SPACE_2);
    Heading::new("Heading 1").h1().show(ui);
    ui.add_space(core::SPACE_2);
    Heading::new("Heading 2").h2().show(ui);
    ui.add_space(core::SPACE_2);
    Heading::new("Heading").heading().show(ui);
}

fn page_icon(ui: &mut Ui, theme: &Theme) {
    caption(ui, "Sizes (sm/md/lg/xl) + color");
    ui.horizontal(|ui| {
        Icon::new(light::CUBE).sm().show(ui);
        ui.add_space(core::SPACE_3);
        Icon::new(light::CUBE).md().show(ui);
        ui.add_space(core::SPACE_3);
        Icon::new(light::CUBE).lg().show(ui);
        ui.add_space(core::SPACE_3);
        Icon::new(light::CUBE).xl().show(ui);
        ui.add_space(core::SPACE_5);
        Icon::new(light::CHECK).lg().color(theme.success).show(ui);
        ui.add_space(core::SPACE_3);
        Icon::new(light::WARNING).lg().color(theme.warning).show(ui);
        ui.add_space(core::SPACE_3);
        Icon::new(light::HEART).lg().color(theme.error).show(ui);
    });
    subhead(ui, "Phosphor Light set (sample)");
    ui.horizontal_wrapped(|ui| {
        for g in [
            light::GRID_FOUR,
            light::SQUARES_FOUR,
            light::CUBE,
            light::TERMINAL,
            light::GEAR,
            light::PALETTE,
            light::MAGNIFYING_GLASS,
            light::STAR,
            light::CHECK,
            light::WARNING,
            light::INFO,
            light::LIGHTBULB,
        ] {
            Icon::new(g).lg().show(ui);
            ui.add_space(core::SPACE_3);
        }
    });
}

fn page_divider(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "Horizontal");
    Divider::horizontal().show(ui);
    ui.add_space(core::SPACE_3);
    caption(ui, "Destructive (red rule)");
    Divider::horizontal().destructive().show(ui);
    ui.add_space(core::SPACE_3);
    caption(ui, "Vertical (between content)");
    ui.allocate_ui(vec2(ui.available_width(), 22.0), |ui| {
        ui.horizontal(|ui| {
            Text::new("File").show(ui);
            ui.add_space(core::SPACE_3);
            Divider::vertical().show(ui);
            ui.add_space(core::SPACE_3);
            Text::new("Edit").show(ui);
        });
    });
}

fn page_button(ui: &mut Ui, _theme: &Theme) {
    caption(ui, "Variants × sizes (Sm/Md/Lg)");
    for (name, variant) in [
        ("Default", ButtonVariant::Default),
        ("Secondary", ButtonVariant::Secondary),
        ("Destructive", ButtonVariant::Destructive),
        ("Outline", ButtonVariant::Outline),
        ("Ghost", ButtonVariant::Ghost),
        ("Link", ButtonVariant::Link),
    ] {
        ui.horizontal(|ui| {
            name_cell(ui, name);
            Button::new("Button")
                .variant(variant)
                .sm()
                .id_source((name, "sm"))
                .show(ui);
            ui.add_space(core::SPACE_2);
            Button::new("Button")
                .variant(variant)
                .id_source((name, "md"))
                .show(ui);
            ui.add_space(core::SPACE_2);
            Button::new("Button")
                .variant(variant)
                .lg()
                .id_source((name, "lg"))
                .show(ui);
        });
        ui.add_space(core::SPACE_1);
    }
    subhead(ui, "Icons · icon-only · disabled");
    ui.horizontal(|ui| {
        Button::new("New")
            .icon_left(light::CUBE)
            .id_source("b_new")
            .show(ui);
        ui.add_space(core::SPACE_2);
        Button::new("Next")
            .icon_right(light::CHECK)
            .secondary()
            .id_source("b_next")
            .show(ui);
        ui.add_space(core::SPACE_2);
        Button::new("")
            .icon_only()
            .icon_left(light::GEAR)
            .outline()
            .id_source("b_gear")
            .show(ui);
        ui.add_space(core::SPACE_2);
        Button::new("Delete")
            .destructive()
            .icon_left(light::WARNING)
            .id_source("b_del")
            .show(ui);
        ui.add_space(core::SPACE_2);
        Button::new("Disabled")
            .disabled()
            .id_source("b_dis")
            .show(ui);
    });
}

// ── Helpers (text via atoms) ──────────────────────────────────────────────────

/// A muted caption line.
fn caption(ui: &mut Ui, text: &str) {
    Text::new(text).caption().muted().show(ui);
}

/// A subsection label with space above.
fn subhead(ui: &mut Ui, text: &str) {
    ui.add_space(core::SPACE_5);
    Text::new(text).label().show(ui);
    ui.add_space(core::SPACE_2);
}

/// A fixed-width mono name column (for `name | demo` rows).
fn name_cell(ui: &mut Ui, name: &str) {
    ui.allocate_ui(vec2(112.0, core::TEXT_LG), |ui| {
        Text::new(name).code().muted().show(ui);
    });
}

/// A `name | rendered` typography row.
fn type_row(ui: &mut Ui, name: &str, render: impl FnOnce(&mut Ui)) {
    ui.horizontal(|ui| {
        ui.allocate_ui(vec2(120.0, core::TEXT_3XL), |ui| {
            Text::new(name).code().muted().show(ui);
        });
        render(ui);
    });
    ui.add_space(core::SPACE_1);
}

fn al_box(ui: &mut Ui, theme: &Theme, add: impl FnOnce(&mut Ui)) {
    egui::Frame::default()
        .stroke(Stroke::new(core::BORDER_THIN, theme.border))
        .corner_radius(CornerRadius::same(core::RADIUS_MD as u8))
        .inner_margin(core::SPACE_1)
        .show(ui, |ui| {
            ui.set_width(ui.available_width().min(420.0));
            add(ui);
        });
    ui.add_space(core::SPACE_3);
}

/// A content-sized pill — a stand-in child for the auto-layout demos.
fn chip(ui: &mut Ui, label: &str, fill: Color32, fg: Color32) {
    let pad = vec2(core::SPACE_3, core::SPACE_2);
    let galley = ui
        .painter()
        .layout_no_wrap(label.to_owned(), typography::label().font_id(), fg);
    let size = galley.size() + pad * 2.0;
    let (rect, _) = ui.allocate_exact_size(size, Sense::hover());
    ui.painter()
        .rect_filled(rect, CornerRadius::same(core::RADIUS_MD as u8), fill);
    ui.painter().galley(rect.min + pad, galley, fg);
}

fn hex(c: Color32) -> String {
    format!("#{:02x}{:02x}{:02x}", c.r(), c.g(), c.b())
}

fn overlay_tile(ui: &mut Ui, label: &str, base: Color32, overlay: Color32) {
    ui.vertical(|ui| {
        let (rect, _) = ui.allocate_exact_size(vec2(104.0, 44.0), Sense::hover());
        let cr = CornerRadius::same(core::RADIUS_MD as u8);
        ui.painter().rect_filled(rect, cr, base);
        ui.painter().rect_filled(rect, cr, overlay);
        ui.add_space(core::SPACE_1);
        Text::new(label).caption().color(core::ZINC_400).show(ui);
    });
}

fn swatch_row(ui: &mut Ui, items: &[(&str, Color32)], theme: &Theme) {
    ui.horizontal_wrapped(|ui| {
        for (name, color) in items {
            ui.vertical(|ui| {
                let (rect, _) = ui.allocate_exact_size(vec2(104.0, 44.0), Sense::hover());
                ui.painter()
                    .rect_filled(rect, CornerRadius::same(core::RADIUS_MD as u8), *color);
                ui.painter().rect_stroke(
                    rect,
                    CornerRadius::same(core::RADIUS_MD as u8),
                    Stroke::new(core::BORDER_THIN, theme.border),
                    StrokeKind::Inside,
                );
                ui.add_space(core::SPACE_1);
                Text::new(*name).caption().show(ui);
                Text::new(hex(*color)).code().muted().show(ui);
            });
            ui.add_space(core::SPACE_3);
        }
    });
}
