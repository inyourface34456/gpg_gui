use crate::MyApp;
use egui::Color32;
use egui::Ui;
use sequoia_openpgp::types::KeyFlags;
// use crate::shared::checkbox::CheckboxDropdown;
// use sequoia_openpgp::cert::prelude::*;
#[cfg(target_arch = "wasm32")]
use crate::get_and_display_certs;
use sequoia_openpgp::cert::CipherSuite;

pub fn see_certs(self_: &mut MyApp, ui: &mut Ui) {
    #[cfg(target_arch = "wasm32")]
    get_and_display_certs(self_, ui);

    for i in &self_.certs {
        ui.label(format!(
            "User Ids: {}",
            i.userids().map(|cert| String::from_utf8_lossy(cert.userid().value()).to_string()).collect::<Vec<String>>()[0]
        ));
    }
    if self_.err != String::new() {
        ui.label(&self_.err);
    }
}

pub fn new_cert(self_: &mut MyApp, ui: &mut Ui) {
    ui.heading("New Certifacate");
    ui.separator();
    ui.horizontal(|ui| {
        ui.label("Key Flags");
        self_.cert_status.key_flags.show(ui);
    });
    ui.add_space(7.);

    let key_flags = KeyFlags::new([0]);
    // ["Authentication", "Certification", "Sigining", "Transport Encryption", "Storage Encryption"]
    let enabled_flags = self_.cert_status.key_flags.selected_by_pos();
    key_flags
        .set_authentication_to(enabled_flags[0])
        .set_certification_to(enabled_flags[1])
        .set_signing_to(enabled_flags[2])
        .set_transport_encryption_to(enabled_flags[3])
        .set_storage_encryption_to(enabled_flags[4]);

    ui.horizontal(|ui| {
        ui.label("Primary Crypto Algorithm");
        egui::ComboBox::from_label(" ")
            .selected_text(format!("{:?}", self_.cert_status.crypto_algo))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self_.cert_status.crypto_algo, CipherSuite::Cv25519, "Cv25519");
                ui.selectable_value(&mut self_.cert_status.crypto_algo, CipherSuite::Cv448, "Cv448");
                ui.selectable_value(&mut self_.cert_status.crypto_algo, CipherSuite::P256, "NistP256");
                ui.selectable_value(&mut self_.cert_status.crypto_algo, CipherSuite::P384, "NistP384");
                ui.selectable_value(&mut self_.cert_status.crypto_algo, CipherSuite::P521, "NistP521");
                ui.selectable_value(&mut self_.cert_status.crypto_algo, CipherSuite::RSA2k, "RSA2k");
                ui.selectable_value(&mut self_.cert_status.crypto_algo, CipherSuite::RSA3k, "RSA3k");
                ui.selectable_value(&mut self_.cert_status.crypto_algo, CipherSuite::RSA4k, "RSA4k");
            });
    });

    ui.horizontal(|ui| {
        ui.label("Display Name: ");
        ui.text_edit_singleline(&mut self_.cert_status.display_name);
    });

    ui.horizontal(|ui| {
        ui.label("Comment (optional): ");
        ui.text_edit_singleline(&mut self_.cert_status.comment);
    });

    ui.horizontal(|ui| {
        ui.label("Email (optional): ");
        ui.text_edit_singleline(&mut self_.cert_status.email);
    });

    let user_id;
    if self_.cert_status.comment.is_empty() && !self_.cert_status.email.is_empty() {
        user_id = format!("{} <{}>", self_.cert_status.display_name, self_.cert_status.email);
    } else if !self_.cert_status.comment.is_empty() && self_.cert_status.email.is_empty() {
        user_id = format!("{} ({})", self_.cert_status.display_name, self_.cert_status.comment);
    } else if !self_.cert_status.comment.is_empty() && !self_.cert_status.email.is_empty() {
        user_id = format!("{} ({}) <{}>", self_.cert_status.display_name, self_.cert_status.comment, self_.cert_status.email);
    } else {
        user_id = self_.cert_status.display_name.clone();
    }

    ui.label(user_id);
}

pub fn style(self_: &mut MyApp, ui: &mut Ui) {
    // let style = ui.style_mut();
    ui.collapsing("Visuals", |ui| {
        ui.horizontal(|ui| {
            let label = ui.label("Dark Mode: ");
            label.on_hover_text(
                "Does not really do anything (according to the docs), buts its here",
            );
            let mut temp = self_.style.visuals.dark_mode;
            ui.checkbox(&mut temp, "");
            self_.style.visuals.dark_mode = temp;
        });

        ui.horizontal(|ui| {
            let label = ui.label("Window Fill: ");
            label.on_hover_text("Background color for the for pannels (like the color selector)");
            let mut temp = self_.style.visuals.window_fill.to_srgba_unmultiplied();
            ui.color_edit_button_srgba_unmultiplied(&mut temp);
            self_.style.visuals.window_fill =
                Color32::from_rgba_unmultiplied(temp[0], temp[1], temp[2], temp[3]);
        });

        ui.horizontal(|ui| {
            let label  = ui.label("Override text color: ");
            label.on_hover_text("Text color");
            let mut temp = self_.style.visuals.override_text_color.unwrap_or(Color32::WHITE).to_srgba_unmultiplied();
            ui.color_edit_button_srgba_unmultiplied(&mut temp);
            self_.style.visuals.override_text_color = Some(Color32::from_rgba_unmultiplied(temp[0], temp[1], temp[2], temp[3]));
        });

        ui.label("Placeholder for widgets style");

        ui.collapsing("Selection Style", |ui| {
            ui.horizontal(|ui| {
                let label = ui.label("Background color: ");
                label.on_hover_text("Background color of the inside of the checkbox");
                let mut temp = self_.style.visuals.selection.bg_fill.to_srgba_unmultiplied();
                ui.color_edit_button_srgba_unmultiplied(&mut temp);
                self_.style.visuals.selection.bg_fill =
                    Color32::from_rgba_unmultiplied(temp[0], temp[1], temp[2], temp[3]);
            });
            ui.collapsing("Stroke", |ui| {
                ui.horizontal(|ui| {
                    let label = ui.label("Stroke color: ");
                    label.on_hover_text("color of the outline on checkboxes");
                    let mut temp = self_.style.visuals.selection.stroke.color.to_srgba_unmultiplied();
                    ui.color_edit_button_srgba_unmultiplied(&mut temp);
                    self_.style.visuals.selection.stroke.color =
                        Color32::from_rgba_unmultiplied(temp[0], temp[1], temp[2], temp[3]);
                });
                ui.horizontal(|ui| {
                    let label = ui.label("Stroke width: ");
                    label.on_hover_text("width of the outline on checkboxes");
                    let mut temp = self_.style.visuals.selection.stroke.width;
                    ui.add(eframe::egui::Slider::new(&mut temp, 0.0..=300.));
                    self_.style.visuals.selection.stroke.width = temp;
                });
            });
        });
        
        ui.horizontal(|ui| {
            let label = ui.label("Hyperlink Color: ");
            // label.on_hover_text("Color of any links");
            label.on_hover_ui(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Controls the color of hyper links like");
                    ui.hyperlink("https://google.com");
                });
            });
            let mut temp = self_.style.visuals.hyperlink_color.to_srgba_unmultiplied();
            ui.color_edit_button_srgba_unmultiplied(&mut temp);
            self_.style.visuals.hyperlink_color = Color32::from_rgba_unmultiplied(temp[0], temp[1], temp[2], temp[3]);
        });
        
        ui.horizontal(|ui| {
            let label = ui.label("Faint BG color: ");
            label.on_hover_text("Background color for the light verion of the background, possibly only used in grids");
            let mut temp = self_.style.visuals.faint_bg_color.to_srgba_unmultiplied();
            ui.color_edit_button_srgba_unmultiplied(&mut temp);
            self_.style.visuals.faint_bg_color = Color32::from_rgba_unmultiplied(temp[0], temp[1], temp[2], temp[3]);
        });
        
        ui.horizontal(|ui| {
            let label = ui.label("Extreme BG color: ");
            label.on_hover_text("Background color for differentiating dark and light parts of the ui (bg of scroll bars, etc)");
            let mut temp = self_.style.visuals.extreme_bg_color.to_srgba_unmultiplied();
            ui.color_edit_button_srgba_unmultiplied(&mut temp);
            self_.style.visuals.extreme_bg_color = Color32::from_rgba_unmultiplied(temp[0], temp[1], temp[2], temp[3]);
        });
        
        ui.horizontal(|ui| {
            let label = ui.label("Code BG color: ");
            label.on_hover_text("Background color for code blocks");
            let mut temp = self_.style.visuals.code_bg_color.to_srgba_unmultiplied();
            ui.color_edit_button_srgba_unmultiplied(&mut temp);
            self_.style.visuals.code_bg_color = Color32::from_rgba_unmultiplied(temp[0], temp[1], temp[2], temp[3]);
        });
        
        ui.horizontal(|ui| {
            let label = ui.label("Warning color: ");
            label.on_hover_text("Color for warnings");
            let mut temp = self_.style.visuals.warn_fg_color.to_srgba_unmultiplied();
            ui.color_edit_button_srgba_unmultiplied(&mut temp);
            self_.style.visuals.warn_fg_color = Color32::from_rgba_unmultiplied(temp[0], temp[1], temp[2], temp[3]);
        });
        
        ui.horizontal(|ui| {
            let label = ui.label("Error color: ");
            label.on_hover_text("Color for errors");
            let mut temp = self_.style.visuals.error_fg_color.to_srgba_unmultiplied();
            ui.color_edit_button_srgba_unmultiplied(&mut temp);
            self_.style.visuals.error_fg_color = Color32::from_rgba_unmultiplied(temp[0], temp[1], temp[2], temp[3]);
        });
        
        ui.collapsing("Window Corner Radius", |ui| {
            ui.horizontal(|ui| {
                let label = ui.label("NW corner rounding: ");
                label.on_hover_text("radius of the northwest corner (i dont know what this actually does, i do not see any effects)");
                let mut temp = self_.style.visuals.window_corner_radius.nw;
                ui.add(eframe::egui::Slider::new(&mut temp, 0..=100));
                self_.style.visuals.window_corner_radius.nw = temp;
            });
            
            ui.horizontal(|ui| {
                let label = ui.label("NE corner rounding: ");
                label.on_hover_text("radius of the northeast corner");
                let mut temp = self_.style.visuals.window_corner_radius.ne;
                ui.add(eframe::egui::Slider::new(&mut temp, 0..=100));
                self_.style.visuals.window_corner_radius.ne = temp;
            });
            
            ui.horizontal(|ui| {
                let label = ui.label("SW corner rounding: ");
                label.on_hover_text("radius of the southwest corner");
                let mut temp = self_.style.visuals.window_corner_radius.sw;
                ui.add(eframe::egui::Slider::new(&mut temp, 0..=100));
                self_.style.visuals.window_corner_radius.sw = temp;
            });
            
            ui.horizontal(|ui| {
                let label = ui.label("SE corner rounding: ");
                label.on_hover_text("radius of the southeast corner");
                let mut temp = self_.style.visuals.window_corner_radius.se;
                ui.add(eframe::egui::Slider::new(&mut temp, 0..=100));
                self_.style.visuals.window_corner_radius.se = temp;
            });
            
            if ui.button("Set all to NW corner").clicked() {
                self_.style.visuals.window_corner_radius.ne = self_.style.visuals.window_corner_radius.nw;
                self_.style.visuals.window_corner_radius.sw = self_.style.visuals.window_corner_radius.nw;
                self_.style.visuals.window_corner_radius.se = self_.style.visuals.window_corner_radius.nw;
            }
        });
        
        let shadow_response = ui.collapsing("Window Shadow", |ui| {
            ui.horizontal(|ui| {
                let label = ui.label("Shadow Drop (Right): ");
                label.on_hover_text("drop of the window shadow");
                let mut temp = self_.style.visuals.window_shadow.offset[0];
                ui.add(eframe::egui::Slider::new(&mut temp, 0..=100));
                self_.style.visuals.window_shadow.offset[0] = temp;
                
                ui.label("Left drop: ");
                let mut temp = self_.style.visuals.window_shadow.offset[1];
                ui.add(eframe::egui::Slider::new(&mut temp, 0..=100));
                self_.style.visuals.window_shadow.offset[1] = temp;
                
                if ui.button("Set equal (set left to right)").clicked() {
                    self_.style.visuals.window_shadow.offset[1] = self_.style.visuals.window_shadow.offset[0];
                }
            });
            
            ui.horizontal(|ui| {
                let label = ui.label("Blur radius: ");
                label.on_hover_text("radius of the blur");
                let mut temp = self_.style.visuals.window_shadow.blur;
                ui.add(eframe::egui::Slider::new(&mut temp, 0..=100));
                self_.style.visuals.window_shadow.blur = temp;
            });
            
            ui.horizontal(|ui| {
                let label = ui.label("Spread: ");
                label.on_hover_text("Spread of the shadow");
                let mut temp = self_.style.visuals.window_shadow.spread;
                ui.add(eframe::egui::Slider::new(&mut temp, 0..=100));
                self_.style.visuals.window_shadow.spread = temp;
            });
            
            ui.horizontal(|ui| {
                let label = ui.label("Shadow color: ");
                label.on_hover_text("Color the shadow");
                let mut temp = self_.style.visuals.window_shadow.color.to_srgba_unmultiplied();
                ui.color_edit_button_srgba_unmultiplied(&mut temp);
                self_.style.visuals.window_shadow.color = Color32::from_rgba_unmultiplied(temp[0], temp[1], temp[2], temp[3]);
            });
        });
        
        match shadow_response.body_response {
            Some(body) => {
                body.on_hover_text_at_pointer("Docs say that this is very similer to CSS drop shadow");
            },
            None => {}
        };
        
        ui.collapsing("Window Stroke", |ui| {
            ui.horizontal(|ui| {
                let label = ui.label("Stroke color: ");
                label.on_hover_text("color of the outline on the window");
                let mut temp = self_.style.visuals.window_stroke.color.to_srgba_unmultiplied();
                ui.color_edit_button_srgba_unmultiplied(&mut temp);
                self_.style.visuals.window_stroke.color =
                    Color32::from_rgba_unmultiplied(temp[0], temp[1], temp[2], temp[3]);
            });
            ui.horizontal(|ui| {
                let label = ui.label("Stroke width: ");
                label.on_hover_text("width of the outline on the window");
                let mut temp = self_.style.visuals.window_stroke.width;
                ui.add(eframe::egui::Slider::new(&mut temp, 0.0..=300.));
                self_.style.visuals.window_stroke.width = temp;
            });
        });
        
        ui.horizontal(|ui| {
            let label = ui.label("Highlight topmost window: ");
            label.on_hover_text("If enabled, highlights the topmost window");
            let mut temp = self_.style.visuals.window_highlight_topmost;
            ui.checkbox(&mut temp, "");
            self_.style.visuals.window_highlight_topmost = temp;
        });
        
        ui.collapsing("Menu Corner Radius", |ui| {
            ui.horizontal(|ui| {
                let label = ui.label("NW corner rounding: ");
                label.on_hover_text("radius of the northwest corner (i dont know what this actually does, i do not see any effects)");
                let mut temp = self_.style.visuals.menu_corner_radius.nw;
                ui.add(eframe::egui::Slider::new(&mut temp, 0..=100));
                self_.style.visuals.menu_corner_radius.nw = temp;
            });
            
            ui.horizontal(|ui| {
                let label = ui.label("NE corner rounding: ");
                label.on_hover_text("radius of the northeast corner");
                let mut temp = self_.style.visuals.menu_corner_radius.ne;
                ui.add(eframe::egui::Slider::new(&mut temp, 0..=100));
                self_.style.visuals.menu_corner_radius.ne = temp;
            });
            
            ui.horizontal(|ui| {
                let label = ui.label("SW corner rounding: ");
                label.on_hover_text("radius of the southwest corner");
                let mut temp = self_.style.visuals.menu_corner_radius.sw;
                ui.add(eframe::egui::Slider::new(&mut temp, 0..=100));
                self_.style.visuals.menu_corner_radius.sw = temp;
            });
            
            ui.horizontal(|ui| {
                let label = ui.label("SE corner rounding: ");
                label.on_hover_text("radius of the southeast corner");
                let mut temp = self_.style.visuals.menu_corner_radius.se;
                ui.add(eframe::egui::Slider::new(&mut temp, 0..=100));
                self_.style.visuals.menu_corner_radius.se = temp;
            });
            
            if ui.button("Set all to NW corner").clicked() {
                self_.style.visuals.menu_corner_radius.ne = self_.style.visuals.menu_corner_radius.nw;
                self_.style.visuals.menu_corner_radius.sw = self_.style.visuals.menu_corner_radius.nw;
                self_.style.visuals.menu_corner_radius.se = self_.style.visuals.menu_corner_radius.nw;
            }
        });
        
        ui.horizontal(|ui| {
            let label = ui.label("Pannel fill: ");
            label.on_hover_text("Background color for the main window");
            let mut temp = self_.style.visuals.panel_fill.to_srgba_unmultiplied();
            ui.color_edit_button_srgba_unmultiplied(&mut temp);
            self_.style.visuals.panel_fill = Color32::from_rgba_unmultiplied(temp[0], temp[1], temp[2], temp[3]);
        });
        
        ui.collapsing("Popup Shadow", |ui| {
            ui.horizontal(|ui| {
                let label = ui.label("Shadow Drop (Right): ");
                label.on_hover_text("drop of the window shadow");
                let mut temp = self_.style.visuals.popup_shadow.offset[0];
                ui.add(eframe::egui::Slider::new(&mut temp, 0..=100));
                self_.style.visuals.popup_shadow.offset[0] = temp;
                
                ui.label("Left drop: ");
                let mut temp = self_.style.visuals.popup_shadow.offset[1];
                ui.add(eframe::egui::Slider::new(&mut temp, 0..=100));
                self_.style.visuals.popup_shadow.offset[1] = temp;
                
                if ui.button("Set equal (set left to right)").clicked() {
                    self_.style.visuals.popup_shadow.offset[1] = self_.style.visuals.popup_shadow.offset[0];
                }
            });
            
            ui.horizontal(|ui| {
                let label = ui.label("Blur radius: ");
                label.on_hover_text("radius of the blur");
                let mut temp = self_.style.visuals.popup_shadow.blur;
                ui.add(eframe::egui::Slider::new(&mut temp, 0..=100));
                self_.style.visuals.popup_shadow.blur = temp;
            });
            
            ui.horizontal(|ui| {
                let label = ui.label("Spread: ");
                label.on_hover_text("Spread of the shadow");
                let mut temp = self_.style.visuals.popup_shadow.spread;
                ui.add(eframe::egui::Slider::new(&mut temp, 0..=100));
                self_.style.visuals.popup_shadow.spread = temp;
            });
            
            ui.horizontal(|ui| {
                let label = ui.label("Shadow color: ");
                label.on_hover_text("Color the shadow");
                let mut temp = self_.style.visuals.popup_shadow.color.to_srgba_unmultiplied();
                ui.color_edit_button_srgba_unmultiplied(&mut temp);
                self_.style.visuals.popup_shadow.color = Color32::from_rgba_unmultiplied(temp[0], temp[1], temp[2], temp[3]);
            });
        });
        
        ui.horizontal(|ui| {
            let label = ui.label("Corner size: ");
            label.on_hover_text("no idea what this does, the feild is named resize_corner_size");
            let mut temp = self_.style.visuals.resize_corner_size;
            ui.add(eframe::egui::Slider::new(&mut temp, 0.0..=300.));
            self_.style.visuals.resize_corner_size = temp;
        });
        
        ui.collapsing("Text Cursor Style", |ui| {
            ui.collapsing("Stroke", |ui| {
                ui.horizontal(|ui| {
                    let label = ui.label("Stroke color: ");
                    label.on_hover_text("color of the outline on the text cursor");
                    let mut temp = self_.style.visuals.text_cursor.stroke.color.to_srgba_unmultiplied();
                    ui.color_edit_button_srgba_unmultiplied(&mut temp);
                    self_.style.visuals.text_cursor.stroke.color =
                        Color32::from_rgba_unmultiplied(temp[0], temp[1], temp[2], temp[3]);
                });
                ui.horizontal(|ui| {
                    let label = ui.label("Stroke width: ");
                    label.on_hover_text("width of the outline on the text cursor");
                    let mut temp = self_.style.visuals.text_cursor.stroke.width;
                    ui.add(eframe::egui::Slider::new(&mut temp, 0.0..=300.));
                    self_.style.visuals.text_cursor.stroke.width = temp;
                });
            });
            
            ui.horizontal(|ui| {
                let label = ui.label("Preview: ");
                label.on_hover_text("Shows where the text cursor would be if you clicked");
                let mut temp = self_.style.visuals.text_cursor.preview;
                ui.checkbox(&mut temp, "");
                self_.style.visuals.text_cursor.preview = temp;
            });
            
            ui.horizontal(|ui| {
                let label = ui.label("Blink: ");
                label.on_hover_text("Should the cursor blink");
                let mut temp = self_.style.visuals.text_cursor.blink;
                ui.checkbox(&mut temp, "");
                self_.style.visuals.text_cursor.blink = temp;
            });
            
            ui.horizontal(|ui| {
                let label = ui.label("On duration: ");
                label.on_hover_text("amount of time the cursor stays on during the blink cycle");
                let mut temp = self_.style.visuals.text_cursor.on_duration;
                ui.add(eframe::egui::Slider::new(&mut temp, 0.0..=300.));
                self_.style.visuals.text_cursor.on_duration = temp;
            });
            
            ui.horizontal(|ui| {
                let label = ui.label("Off duration: ");
                label.on_hover_text("amount of time the cursor stays off during the blink cycle");
                let mut temp = self_.style.visuals.text_cursor.off_duration;
                ui.add(eframe::egui::Slider::new(&mut temp, 0.0..=300.));
                self_.style.visuals.text_cursor.off_duration = temp;
            });
        });
        
        ui.horizontal(|ui| {
            let label = ui.label("Clip Rect Margin: ");
            label.on_hover_text("Also dont really understand this one (Allow child widgets to be just on the border and still have a stroke with some thickness according to docs)");
            let mut temp = self_.style.visuals.clip_rect_margin;
            ui.add(eframe::egui::Slider::new(&mut temp, 0.0..=300.));
            self_.style.visuals.clip_rect_margin = temp;
        });
        
        ui.horizontal(|ui| {
            let label = ui.label("Button Fame: ");
            label.on_hover_text("Waether or not to show a background on buttons");
            let mut temp = self_.style.visuals.button_frame;
            ui.checkbox(&mut temp, "");
            self_.style.visuals.button_frame = temp;
        });
        
        ui.horizontal(|ui| {
            let label = ui.label("Collapsing Header Frame: ");
            label.on_hover_text("Show a background behind collapsing headers");
            let mut temp = self_.style.visuals.collapsing_header_frame;
            ui.checkbox(&mut temp, "");
            self_.style.visuals.collapsing_header_frame = temp;
        });
        
        ui.horizontal(|ui| {
            let label = ui.label("Indent has Left Vertical Line: ");
            label.on_hover_text("Draw a vertical lien left of indented region");
            let mut temp = self_.style.visuals.indent_has_left_vline;
            ui.checkbox(&mut temp, "");
            self_.style.visuals.indent_has_left_vline = temp;
        });
        
        ui.horizontal(|ui| {
            let label = ui.label("Striped: ");
            label.on_hover_text("Weather or not to stripe grids and tables");
            let mut temp = self_.style.visuals.striped;
            ui.checkbox(&mut temp, "");
            self_.style.visuals.striped = temp;
        });
        
        ui.horizontal(|ui| {
            let label = ui.label("Slider trailing fill: ");
            label.on_hover_text("Show trailing color behind the circle of a Slider");
            let mut temp = self_.style.visuals.slider_trailing_fill;
            ui.checkbox(&mut temp, "");
            self_.style.visuals.slider_trailing_fill = temp;
        });
        
        ui.horizontal(|ui| {
            let label = ui.label("Show image loading spinner: ");
            label.on_hover_text("Show a spinner when loading an image");
            let mut temp = self_.style.visuals.image_loading_spinners;
            ui.checkbox(&mut temp, "");
            self_.style.visuals.image_loading_spinners = temp;
        });
        
        ui.horizontal(|ui| {
            let label = ui.label("How to display Colors: ");
            label.on_hover_text("How to display numeric color values (gamma byte is 0-255, linear is 0-1)");
            let mut temp = self_.style.visuals.numeric_color_space;
            egui::ComboBox::from_label("")
                .selected_text(format!("{:?}", temp))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut temp, eframe::egui::style::NumericColorSpace::GammaByte, "GammaByte");
                    ui.selectable_value(&mut temp, eframe::egui::style::NumericColorSpace::Linear, "Linear");
                });
            self_.style.visuals.numeric_color_space = temp;
        });
    });
}
