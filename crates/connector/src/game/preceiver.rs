use std::net::SocketAddr;

use async_std::channel::Receiver;
use de_net::{FromGame, OutPackage, PackageSender, Peers};
use tracing::{error, info, warn};

use super::state::GameState;

/// A package destined to other players in the game.
pub(super) struct PlayersPackage {
    reliable: bool,
    source: SocketAddr,
    data: Vec<u8>,
}

impl PlayersPackage {
    pub(super) fn new(reliable: bool, source: SocketAddr, data: Vec<u8>) -> Self {
        Self {
            reliable,
            source,
            data,
        }
    }
}

pub(super) async fn run(
    port: u16,
    packages: Receiver<PlayersPackage>,
    outputs: PackageSender,
    state: GameState,
) {
    info!("Starting game player package handler on port {port}...");

    loop {
        if packages.is_closed() {
            break;
        }

        if outputs.is_closed() {
            error!("Outputs channel on port {port} was unexpectedly closed.");
            break;
        }

        let Ok(package) = packages.recv().await else {
            break;
        };

        if !state.contains(package.source).await {
            warn!(
                "Received a player message from a non-participating client: {:?}.",
                package.source
            );

            let _ = outputs
                .send(
                    OutPackage::encode_single(
                        &FromGame::NotJoined,
                        package.reliable,
                        Peers::Server,
                        package.source,
                    )
                    .unwrap(),
                )
                .await;
            continue;
        }

        let Some(targets) = state.targets(Some(package.source)).await else {
            continue;
        };

        let result = outputs
            .send(OutPackage::new(
                package.data,
                package.reliable,
                Peers::Players,
                targets,
            ))
            .await;
        if result.is_err() {
            break;
        }
    }

    info!("Game player package handler on port {port} finished.");
}
