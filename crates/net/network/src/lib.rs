#![warn(missing_docs)]
#![deny(unused_must_use, rust_2018_idioms)]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms), allow(dead_code, unused_variables))
))]
// TODO remove later
#![allow(dead_code)]

//! reth P2P networking.
//!
//! Ethereum's networking protocol is specified in [devp2p](https://github.com/ethereum/devp2p).
//!
//! In order for a node to join the ethereum p2p network it needs to know what nodes are already
//! port of that network. This includes public identities (public key) and addresses (where to reach
//! them).

mod config;
mod discovery;
pub mod error;
mod fetch;
mod listener;
mod manager;
mod message;
mod network;
mod peers;
mod session;
mod state;
mod swarm;
mod transactions;

/// Identifier for a unique node
pub type NodeId = reth_discv4::NodeId;

pub use config::NetworkConfig;
pub use manager::NetworkManager;
pub use network::NetworkHandle;
pub use peers::PeersConfig;