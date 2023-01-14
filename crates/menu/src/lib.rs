use bevy::{app::PluginGroupBuilder, prelude::PluginGroup};
use gamelisting::GameListingPlugin;
use mainmenu::MainMenuPlugin;
use mapselection::MapSelectionPlugin;
use menu::MenuPlugin;
use signin::SignInPlugin;

mod gamelisting;
mod mainmenu;
mod mapselection;
mod menu;
mod signin;

pub struct MenuPluginGroup;

impl PluginGroup for MenuPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(MenuPlugin)
            .add(MainMenuPlugin)
            .add(MapSelectionPlugin)
            .add(SignInPlugin)
            .add(GameListingPlugin)
    }
}
