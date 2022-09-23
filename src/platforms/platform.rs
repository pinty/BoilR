use steam_shortcuts_util::shortcut::ShortcutOwned;

use super::egs::EpicPlatform;
use super::amazon::AmazonPlatform;
use super::bottles::BottlesPlatform;

pub trait Platform<T, E>
where
    T: Into<ShortcutOwned>,
{
    fn enabled(&self) -> bool;

    fn name(&self) -> &str;

    fn get_shortcuts(&self) -> Result<Vec<T>, E>;

    fn settings_valid(&self) -> SettingsValidity;

    #[cfg(target_family = "unix")]
    fn create_symlinks(&self) -> bool;

    // HOME/.local/share/Steam/config/config.vdf
    fn needs_proton(&self, input: &T) -> bool;
}

pub enum SettingsValidity {
    Valid,
    Invalid { reason: String },
}

#[derive(Clone)]
pub enum PlatformEnum {
    Amazon(AmazonPlatform),
    Bottles(BottlesPlatform),
    Epic(EpicPlatform),
}

impl PlatformEnum {
    pub fn name(&self) -> &str {
        match self {
            PlatformEnum::Amazon(_) => "Amazon",
            PlatformEnum::Bottles(_) => "Bottles",
            PlatformEnum::Epic(_) => "Epic",
        }
    }

    pub fn enabled(&self) -> bool {
        match self {
            PlatformEnum::Amazon(p) => p.settings.enabled,
            PlatformEnum::Bottles(p) => p.settings.enabled,
            PlatformEnum::Epic(p) => p.settings.enabled,
        }
    }

    pub fn render_ui(&mut self, ui: &mut egui::Ui) {
        match self {
            PlatformEnum::Amazon(p) => p.render_amazon_settings(ui),
            PlatformEnum::Bottles(p) => p.render_bottles_settings(ui),
            PlatformEnum::Epic(p) => p.render_epic_settings(ui),
        }
    }

    pub fn get_shortcuts(&self) -> eyre::Result<Vec<ShortcutOwned>> {
        match self {
            PlatformEnum::Amazon(p) => to_shortcuts(p.get_amazon_games()),
            PlatformEnum::Bottles(p) => to_shortcuts(p.get_botttles()),
            PlatformEnum::Epic(p) => to_shortcuts(p.get_epic_games()),
        }
    }
}

fn to_shortcuts<T>(
    into_shortcuts: Result<Vec<T>, eyre::ErrReport>,
) -> eyre::Result<Vec<ShortcutOwned>>
where
    T: Clone,
    T: Into<ShortcutOwned>,
{
    let shortcuts = into_shortcuts?;
    let shortcut_owneds = shortcuts.iter().map(|m| m.clone().into()).collect();
    Ok(shortcut_owneds)
}

pub type Platforms = [PlatformEnum;3];

pub fn get_platforms(settings:&crate::settings::Settings) -> Platforms {
    [
        PlatformEnum::Epic(EpicPlatform {
            epic_manifests: None,
            settings: settings.epic_games.clone(),
        }),
        PlatformEnum::Amazon(AmazonPlatform {
            settings: settings.amazon.clone(),
        }),
        PlatformEnum::Bottles(BottlesPlatform {
            settings: settings.bottles.clone(),
        }),
    ]
}
