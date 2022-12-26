use bevy::{app::PluginGroupBuilder, prelude::PluginGroup};
use iyes_progress::prelude::*;
use stages::StagesPlugin;
use state::{GameState, MenuState};
use visibility::VisibilityPlugin;

pub mod assets;
mod errors;
pub mod events;
pub mod frustum;
pub mod gconfig;
pub mod objects;
pub mod player;
pub mod projection;
pub mod screengeom;
pub mod stages;
pub mod state;
pub mod visibility;

pub struct CorePluginGroup;

impl PluginGroup for CorePluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(ProgressPlugin::new(MenuState::MLoading).continue_to(MenuState::MainMenu))
            .add(ProgressPlugin::new(GameState::Loading).continue_to(GameState::Playing))
            .add(StagesPlugin)
            .add(VisibilityPlugin)
    }
}
