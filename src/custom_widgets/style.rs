use egui::style::{NumericColorSpace, Selection, TextCursorStyle, Widgets};
use egui::{Checkbox, Color32, Response, Slider, Style, Ui, Widget};
use std::ops::RangeInclusive;

pub struct StyleEditor<'a>(&'a mut Style);

impl<'a> StyleEditor<'a> {
    pub fn new(style: &'a mut Style) -> Self {
        Self(style)
    }
}

impl Widget for StyleEditor<'_> {
    #[allow(clippy::too_many_lines)]
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            ui.collapsing("Visuals", |ui| {
                let visuals = &mut self.0.visuals;

                ui.add(bool_editor(
                    "Dark Mode:",
                    "Summary of whether the colors form a dark theme",
                    &mut visuals.dark_mode,
                ));
                ui.add(color_editor(
                    "Window Fill:",
                    "Background color for panels such as the color selector",
                    &mut visuals.window_fill,
                ));
                ui.add(optional_color_editor(
                    "Override text color:",
                    "Override the text color for every widget",
                    &mut visuals.override_text_color,
                ));
                ui.add(section(
                    "Widget Style",
                    widget_styles_editor(&mut visuals.widgets),
                ));
                ui.add(section(
                    "Selection Style",
                    selection_editor(&mut visuals.selection),
                ));

                for (label, tooltip, color) in [
                    (
                        "Hyperlink Color:",
                        "Color used for hyperlinks",
                        &mut visuals.hyperlink_color,
                    ),
                    (
                        "Faint BG color:",
                        "Subtle backgrounds such as striped grids",
                        &mut visuals.faint_bg_color,
                    ),
                    (
                        "Extreme BG color:",
                        "Background used to distinguish contrasting UI regions",
                        &mut visuals.extreme_bg_color,
                    ),
                    (
                        "Code BG color:",
                        "Background color for code blocks",
                        &mut visuals.code_bg_color,
                    ),
                    (
                        "Warning color:",
                        "Color used for warnings",
                        &mut visuals.warn_fg_color,
                    ),
                    (
                        "Error color:",
                        "Color used for errors",
                        &mut visuals.error_fg_color,
                    ),
                ] {
                    ui.add(color_editor(label, tooltip, color));
                }

                ui.add(section(
                    "Window Corner Radius",
                    &mut visuals.window_corner_radius,
                ));
                ui.add(section("Window Shadow", &mut visuals.window_shadow))
                    .on_hover_text_at_pointer("The window shadow is similar to a CSS drop shadow");
                ui.add(section("Window Stroke", &mut visuals.window_stroke));
                ui.add(bool_editor(
                    "Highlight topmost window:",
                    "Highlight the topmost window",
                    &mut visuals.window_highlight_topmost,
                ));
                ui.add(section(
                    "Menu Corner Radius",
                    &mut visuals.menu_corner_radius,
                ));
                ui.add(color_editor(
                    "Panel fill:",
                    "Background color for the main window",
                    &mut visuals.panel_fill,
                ));
                ui.add(section("Popup Shadow", &mut visuals.popup_shadow));
                ui.add(f32_editor(
                    "Corner size:",
                    "Size of the window resize corner",
                    &mut visuals.resize_corner_size,
                    0.0..=300.0,
                ));
                ui.add(section(
                    "Text Cursor Style",
                    text_cursor_editor(&mut visuals.text_cursor),
                ));
                ui.add(f32_editor(
                    "Clip Rect Margin:",
                    "Allows child widgets on a border to retain their stroke",
                    &mut visuals.clip_rect_margin,
                    0.0..=300.0,
                ));

                for (label, tooltip, value) in [
                    (
                        "Button Frame:",
                        "Show a background on buttons",
                        &mut visuals.button_frame,
                    ),
                    (
                        "Collapsing Header Frame:",
                        "Show a background behind collapsing headers",
                        &mut visuals.collapsing_header_frame,
                    ),
                    (
                        "Indent has Left Vertical Line:",
                        "Draw a vertical line to the left of indented regions",
                        &mut visuals.indent_has_left_vline,
                    ),
                    ("Striped:", "Stripe grids and tables", &mut visuals.striped),
                    (
                        "Slider trailing fill:",
                        "Show a trailing color behind slider handles",
                        &mut visuals.slider_trailing_fill,
                    ),
                    (
                        "Show image loading spinner:",
                        "Show a spinner while loading an image",
                        &mut visuals.image_loading_spinners,
                    ),
                ] {
                    ui.add(bool_editor(label, tooltip, value));
                }

                ui.add(numeric_color_space_editor(&mut visuals.numeric_color_space));
            });
        })
        .response
    }
}

fn labeled<'a>(
    label: &'static str,
    tooltip: &'static str,
    widget: impl Widget + 'a,
) -> impl Widget + 'a {
    move |ui: &mut Ui| {
        ui.horizontal(|ui| {
            ui.label(label).on_hover_text(tooltip);
            ui.add(widget);
        })
        .response
    }
}

fn section<'a>(title: &'static str, widget: impl Widget + 'a) -> impl Widget + 'a {
    move |ui: &mut Ui| {
        ui.vertical(|ui| {
            ui.collapsing(title, |ui| {
                ui.add(widget);
            });
        })
        .response
    }
}

fn color_editor<'a>(
    label: &'static str,
    tooltip: &'static str,
    color: &'a mut Color32,
) -> impl Widget + 'a {
    labeled(label, tooltip, move |ui: &mut Ui| {
        ui.color_edit_button_srgba(color)
    })
}

fn optional_color_editor<'a>(
    label: &'static str,
    tooltip: &'static str,
    color: &'a mut Option<Color32>,
) -> impl Widget + 'a {
    color_editor(label, tooltip, color.get_or_insert(Color32::WHITE))
}

fn bool_editor<'a>(
    label: &'static str,
    tooltip: &'static str,
    value: &'a mut bool,
) -> impl Widget + 'a {
    labeled(label, tooltip, Checkbox::without_text(value))
}

fn f32_editor<'a>(
    label: &'static str,
    tooltip: &'static str,
    value: &'a mut f32,
    range: RangeInclusive<f32>,
) -> impl Widget + 'a {
    labeled(label, tooltip, Slider::new(value, range))
}

fn widget_styles_editor(widgets: &mut Widgets) -> impl Widget + '_ {
    move |ui: &mut Ui| ui.vertical(|ui| widgets.ui(ui)).response
}

fn selection_editor(selection: &mut Selection) -> impl Widget + '_ {
    move |ui: &mut Ui| ui.vertical(|ui| selection.ui(ui)).response
}

fn text_cursor_editor(style: &mut TextCursorStyle) -> impl Widget + '_ {
    move |ui: &mut Ui| {
        ui.vertical(|ui| {
            ui.add(labeled(
                "Stroke:",
                "Appearance of the text cursor",
                &mut style.stroke,
            ));
            ui.add(bool_editor(
                "Preview:",
                "Show where the cursor would be after clicking",
                &mut style.preview,
            ));
            ui.add(bool_editor(
                "Blink:",
                "Whether the cursor should blink",
                &mut style.blink,
            ));
            ui.add(f32_editor(
                "On duration:",
                "Time the cursor remains visible",
                &mut style.on_duration,
                0.0..=300.0,
            ));
            ui.add(f32_editor(
                "Off duration:",
                "Time the cursor remains hidden",
                &mut style.off_duration,
                0.0..=300.0,
            ));
        })
        .response
    }
}

fn numeric_color_space_editor(value: &mut NumericColorSpace) -> impl Widget + '_ {
    move |ui: &mut Ui| {
        ui.horizontal(|ui| {
            ui.label("How to display Colors:")
                .on_hover_text("Gamma byte uses 0-255; linear uses 0-1");
            egui::ComboBox::from_id_salt("numeric_color_space")
                .selected_text(format!("{value:?}"))
                .show_ui(ui, |ui| {
                    ui.selectable_value(value, NumericColorSpace::GammaByte, "GammaByte");
                    ui.selectable_value(value, NumericColorSpace::Linear, "Linear");
                });
        })
        .response
    }
}
