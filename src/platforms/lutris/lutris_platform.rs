use super::game_list_parser::parse_lutris_games;
use super::lutris_game::LutrisGame;
use super::settings::LutrisSettings;
use crate::platforms::{to_shortcuts_simple, ShortcutToImport};
use std::process::Command;

#[derive(Clone)]
pub struct LutrisPlatform {
    pub settings: LutrisSettings,
}

impl LutrisPlatform {
    pub fn render_lutris_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading("Lutris");
        ui.checkbox(&mut self.settings.enabled, "Import from Lutris");
        if self.settings.enabled {
            ui.checkbox(&mut self.settings.flatpak, "Flatpak version");
            if !self.settings.flatpak {
                ui.horizontal(|ui| {
                    let lutris_location = &mut self.settings.executable;
                    ui.label("Lutris Location: ");
                    ui.text_edit_singleline(lutris_location);
                });
            } else {
                ui.horizontal(|ui| {
                    let flatpak_image = &mut self.settings.flatpak_image;
                    ui.label("Flatpak image");
                    ui.text_edit_singleline(flatpak_image);
                });
            }
        }
    }

    pub fn get_shortcut_info(&self) -> eyre::Result<Vec<ShortcutToImport>> {
        to_shortcuts_simple(self.get_shortcuts())
    }

    fn get_shortcuts(&self) -> eyre::Result<Vec<LutrisGame>> {
        let output = get_lutris_command_output(&self.settings)?;
        let games = parse_lutris_games(output.as_str());
        let mut res = vec![];
        for mut game in games {
            if game.runner != "steam" {
                game.settings = Some(self.settings.clone());
                res.push(game);
            }
        }
        Ok(res)
    }
}

fn get_lutris_command_output(settings: &LutrisSettings) -> eyre::Result<String> {
    let output = if settings.flatpak {
        let flatpak_image = &settings.flatpak_image;
        #[cfg(not(feature = "flatpak"))]
        {
            let mut command = Command::new("flatpak");
            command
                .arg("run")
                .arg(flatpak_image)
                .arg("-lo")
                .arg("--json")
                .output()?
        }
        #[cfg(feature = "flatpak")]
        {
            let mut command = Command::new("flatpak-spawn");
            command
                .arg("--host")
                .arg("flatpak")
                .arg("run")
                .arg(flatpak_image)
                .arg("-lo")
                .arg("--json")
                .output()?
        }
    } else {
        let mut command = Command::new(&settings.executable);
        command.arg("-lo").arg("--json").output()?
    };

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}