//! This crate library implements multiplayer functionality for Digital
//! Extinction via [`MultiplayerPluginGroup`].
//!
//! Before a multiplayer game starts, networking and other systems must be
//! started. To do this send [`StartMultiplayerEvent`].
//!
//! After a multiplayer game ends, the multiplayer functionality should be shut
//! down via [`ShutdownMultiplayerEvent`].

use bevy::{app::PluginGroupBuilder, prelude::*};
use game::GamePlugin;
use lifecycle::LifecyclePlugin;
use messages::MessagesPlugin;
use stats::StatsPlugin;

pub use crate::{
    config::{NetGameConf, ServerPort},
    lifecycle::{ShutdownMultiplayerEvent, StartMultiplayerEvent},
    netstate::NetState,
};
use crate::{netstate::NetStatePlugin, network::NetworkPlugin};

mod config;
mod game;
mod lifecycle;
mod messages;
mod netstate;
mod network;
mod stats;

pub struct MultiplayerPluginGroup;

impl PluginGroup for MultiplayerPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(NetStatePlugin)
            .add(LifecyclePlugin)
            .add(NetworkPlugin)
            .add(MessagesPlugin)
            .add(GamePlugin)
            .add(StatsPlugin)
    }
}
