/// This crate contains the main link for lighthouse to rust-libp2p. It therefore re-exports
/// all required libp2p functionality.
///
/// This crate builds and manages the libp2p services required by the beacon node.
#[macro_use]
extern crate lazy_static;

pub mod behaviour;
mod config;
mod metrics;
mod peer_manager;
pub mod rpc;
mod service;
pub mod types;

pub use crate::types::{error, Enr, GossipTopic, NetworkGlobals, PubsubMessage};
pub use behaviour::{BehaviourEvent, PeerRequestId, Request, Response};
pub use config::Config as NetworkConfig;
pub use discv5;
pub use libp2p::gossipsub::{MessageId, Topic, TopicHash};
pub use libp2p::{core::ConnectedPoint, PeerId, Swarm};
pub use libp2p::{multiaddr, Multiaddr};
pub use metrics::scrape_discovery_metrics;
pub use peer_manager::discovery;
pub use peer_manager::{
    client::Client,
    discovery::{CombinedKeyExt, EnrExt, Eth2Enr},
    PeerDB, PeerInfo, PeerSyncStatus, SyncInfo,
};
pub use service::{Libp2pEvent, Service, NETWORK_KEY_FILENAME};
