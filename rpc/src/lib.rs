extern crate bytes;
extern crate prost;
extern crate prost_derive;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate tonic;
#[allow(dead_code)]
#[allow(clippy::type_complexity)]
#[allow(clippy::unit_arg)]
#[allow(clippy::redundant_closure)]
#[allow(clippy::upper_case_acronyms)]
#[allow(clippy::derive_partial_eq_without_eq)]
pub mod io_engine {
    use bdev_rpc_client::BdevRpcClient;
    /// AutoGenerated Io Engine Client
    pub use mayastor_client::MayastorClient as IoEngineClient;

    use std::{
        net::{SocketAddr, TcpStream},
        str::FromStr,
        time::Duration,
    };
    use tonic::transport::Channel;

    #[derive(Debug)]
    pub enum Error {
        ParseError,
    }

    impl From<()> for Null {
        fn from(_: ()) -> Self {
            Self {}
        }
    }

    impl FromStr for NvmeAnaState {
        type Err = Error;
        fn from_str(state: &str) -> Result<Self, Self::Err> {
            match state {
                "optimized" => Ok(Self::NvmeAnaOptimizedState),
                "non_optimized" => Ok(Self::NvmeAnaNonOptimizedState),
                "inaccessible" => Ok(Self::NvmeAnaInaccessibleState),
                _ => Err(Error::ParseError),
            }
        }
    }

    include!(concat!(env!("OUT_DIR"), "/mayastor.rs"));

    /// Test Rpc Handle to connect to an io-engine instance via an endpoint.
    /// Gives access to the io-engine client and the bdev client.
    #[derive(Clone)]
    pub struct RpcHandle {
        pub name: String,
        pub endpoint: SocketAddr,
        pub io_engine: IoEngineClient<Channel>,
        pub bdev: BdevRpcClient<Channel>,
    }

    impl RpcHandle {
        /// Connect to the container and return a handle to `Self`
        /// Note: The initial connection with a timeout is using blocking calls
        pub async fn connect(name: &str, endpoint: SocketAddr) -> Result<Self, String> {
            let mut attempts = 40;
            loop {
                if TcpStream::connect_timeout(&endpoint, Duration::from_millis(100)).is_ok() {
                    break;
                } else {
                    std::thread::sleep(Duration::from_millis(100));
                }
                attempts -= 1;
                if attempts == 0 {
                    return Err(format!("Failed to connect to {}/{}", name, endpoint));
                }
            }

            let io_engine = IoEngineClient::connect(format!("http://{}", endpoint))
                .await
                .unwrap();
            let bdev = BdevRpcClient::connect(format!("http://{}", endpoint))
                .await
                .unwrap();

            Ok(Self {
                name: name.to_string(),
                io_engine,
                bdev,
                endpoint,
            })
        }
    }
}

pub mod csi {
    #![allow(clippy::derive_partial_eq_without_eq)]
    include!(concat!(env!("OUT_DIR"), "/csi.v1.rs"));
}

pub mod v1 {
    pub mod pb {
        #![allow(clippy::derive_partial_eq_without_eq)]
        include!(concat!(env!("OUT_DIR"), "/mayastor.v1.rs"));
    }

    /// V1 Registration autogenerated grpc code
    pub mod registration {
        pub use super::pb::{
            registration_client, registration_server, ApiVersion, DeregisterRequest,
            RegisterRequest,
        };
    }

    /// V1 Host autogenerated grpc code
    pub mod host {
        pub use super::pb::{
            block_device::{Filesystem, Partition},
            host_rpc_client, BlockDevice, ListBlockDevicesRequest,
        };
    }

    /// V1 Replica autogenerated grpc code
    pub mod replica {
        pub use super::pb::{
            replica_rpc_client, CreateReplicaRequest, DestroyReplicaRequest, ListReplicaOptions,
            ListReplicasResponse, Replica, ShareReplicaRequest, UnshareReplicaRequest,
        };
    }

    /// V1 Nexus autogenerated grpc code.
    pub mod nexus {
        pub use super::pb::{
            nexus_rpc_client, AddChildNexusRequest, AddChildNexusResponse, Child, ChildState,
            ChildStateReason, CreateNexusRequest, CreateNexusResponse, DestroyNexusRequest,
            ListNexusOptions, ListNexusResponse, Nexus, NexusState, NvmeAnaState,
            PublishNexusRequest, PublishNexusResponse, RemoveChildNexusRequest,
            RemoveChildNexusResponse, UnpublishNexusRequest, UnpublishNexusResponse,
        };
    }
}

/// V1 Alpha api version.
pub mod v1_alpha {
    /// V1 alpha registration autogenerated grpc code.
    pub mod registration {
        include!(concat!(env!("OUT_DIR"), "/v1.registration.rs"));
    }
}
