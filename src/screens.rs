use imgui::*;
use std::process::Command;
use crate::State;

pub fn hello_world(ui: &mut Ui, window: &winit::window::Window, state: &mut State) {
    ui.window("Hello")
        .size([window.inner_size().width as f32, window.inner_size().height as f32], Condition::Always)
        .flags(imgui::WindowFlags::NO_MOVE | imgui::WindowFlags::NO_RESIZE | imgui::WindowFlags::NO_TITLE_BAR | imgui::WindowFlags::MENU_BAR)
        // .menu_bar(true)
        .position([0., 0.], Condition::Always)
        .build(|| {
            menu_bar(ui, state);
            if let Some(tab_bar) = ui.tab_bar("tab_bar") {
                if let Some(vis) = ui.tab_item("Main") {
                    ui.text("main");
                    vis.end()
                }
                if let Some(vis) = ui.tab_item("Logs") {
                    ui.text("logs");
                    for i in state.get_logs() {
                        ui.text(i)
                    }
                    vis.end();
                }
                tab_bar.end();
            }
        });
}

fn import_key(path: std::path::PathBuf) -> Result<String, String> {
    let mut command = Command::new("gpg");
    command.arg("--import").arg(path);
    let output = command.output().expect("could not capture output");
    if let Some(status) = output.status.code() {
        if status == 0 {
            let stdout_str = str::from_utf8(&output.stderr).expect("Not valid UTF-8");
            Ok(stdout_str.to_string())
        } else {
            let stderr_str = str::from_utf8(&output.stderr).expect("Not valid UTF-8");
            Err(stderr_str.to_string())
        }
    } else {
        Err("failed to start".to_owned())
    }
}

fn menu_bar(ui: &Ui, state: &mut State) {
    if let Some(menu_bar) = ui.begin_menu_bar() {
        file_menu(ui, state);
        menu_bar.end()
    }
}

fn file_menu(ui: &Ui, state: &mut State) {
    if let Some(menu) = ui.begin_menu("File") {
        import_key_menu_item(ui, state);
        menu.end();
    }
}

fn import_key_menu_item(ui: &Ui, state: &mut State) {
    if ui.menu_item("Import Key") {
        if let Some(path) = rfd::FileDialog::new().pick_file() {
            let key_imported = import_key(path);
            if key_imported.is_err() {
                state.add_log(key_imported.unwrap_err());
            } else {
                state.add_log(key_imported.unwrap());
            }
            state.open_popup();
        }
    }
}