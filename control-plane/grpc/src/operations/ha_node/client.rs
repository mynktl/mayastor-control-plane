use crate::{
    context::{Client, Context, TracedChannel},
    ha_cluster_agent::ha_rpc_client::HaRpcClient,
    operations::ha_node::traits::{ClusterAgentOperations, NodeInfo, ReportFailedPathsInfo},
};
use common_lib::{
    transport_api::{ReplyError, TimeoutOptions},
    types::v0::transport::MessageIdVs,
};
use std::ops::Deref;
use tonic::transport::Uri;

/// Cluster-Agent RPC client
pub struct ClusterAgentClient {
    inner: Client<HaRpcClient<TracedChannel>>,
}

impl ClusterAgentClient {
    /// creates a new base tonic endpoint with the timeout options and the address
    pub async fn new<O: Into<Option<TimeoutOptions>>>(addr: Uri, opts: O) -> Self {
        let client = Client::new(addr, opts, HaRpcClient::new).await;
        Self { inner: client }
    }
}

impl Deref for ClusterAgentClient {
    type Target = Client<HaRpcClient<TracedChannel>>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[tonic::async_trait]
impl ClusterAgentOperations for ClusterAgentClient {
    #[tracing::instrument(
        name = "ClusterAgentClient::register",
        level = "debug",
        skip(self),
        err
    )]
    async fn register(
        &self,
        request: &dyn NodeInfo,
        context: Option<Context>,
    ) -> Result<(), ReplyError> {
        let req = self.request(request, context, MessageIdVs::RegisterHaNode);
        let _ = self.client().register_node_agent(req).await?;
        Ok(())
    }

    #[tracing::instrument(
        name = "ClusterAgentClient::report_failed_nvme_paths",
        level = "debug",
        skip(self),
        err
    )]
    /// Report failed NVMe paths.
    async fn report_failed_nvme_paths(
        &self,
        request: &dyn ReportFailedPathsInfo,
        context: Option<Context>,
    ) -> Result<(), ReplyError> {
        let req = self.request(request, context, MessageIdVs::ReportFailedPaths);
        match self.client().report_failed_nvme_paths(req).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }
}
