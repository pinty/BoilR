use super::legendary_game::LegendaryGame;
use super::LegendarySettings;
use crate::platforms::{to_shortcuts_simple, ShortcutToImport};
use serde_json::from_str;
use std::process::Command;

#[derive(Clone)]
pub struct LegendaryPlatform {
    pub settings: LegendarySettings,
}

impl LegendaryPlatform {
    pub fn get_shortcut_info(&self) -> eyre::Result<Vec<ShortcutToImport>> {
        to_shortcuts_simple(self.get_shortcuts())
    }

   pub fn render_legendary_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading("Legendary & Rare");
        ui.checkbox(
            &mut self.settings.enabled,
            "Import from Legendary & Rare",
        );
        if self.settings.enabled {
            ui.horizontal(|ui| {
                let mut empty_string = "".to_string();
                let legendary_location = self
                    .settings
                    .executable
                    .as_mut()
                    .unwrap_or(&mut empty_string);
                ui.label("Legendary Executable: ")
                    .on_hover_text("The location of the legendary executable to use");
                if ui.text_edit_singleline(legendary_location).changed() {
                    self.settings.executable = Some(legendary_location.to_string());
                }
            });
        }
    }    

    fn get_shortcuts(&self) -> eyre::Result<Vec<LegendaryGame>> {
        let legendary_string = self
            .settings
            .executable
            .clone()
            .unwrap_or_else(|| "legendary".to_string());
        let legendary = legendary_string.as_str();
        execute_legendary_command(legendary)
    }
}

fn execute_legendary_command(program: &str) -> eyre::Result<Vec<LegendaryGame>> {
    let legendary_command = Command::new(program)
        .arg("list-installed")
        .arg("--json")
        .output()?;
    let json = String::from_utf8_lossy(&legendary_command.stdout);
    let games = from_str(&json)?;
    Ok(games)
}