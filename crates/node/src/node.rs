//! # AlphaNet Node types configuration
//!
//! The [AlphaNetNode] type implements the [NodeTypes] trait, and configures the engine types
//! required for the optimism engine API.

use crate::evm::AlphaNetEvmConfig;
use reth::builder::{
    components::{ComponentsBuilder, ExecutorBuilder},
    BuilderContext, Node, NodeTypes,
};
use reth_node_api::FullNodeTypes;
use reth_node_optimism::{
    args::RollupArgs,
    node::{OptimismNetworkBuilder, OptimismPayloadBuilder, OptimismPoolBuilder},
    OpExecutorProvider, OptimismEngineTypes,
};

/// Type configuration for a regular AlphaNet node.
#[derive(Debug, Clone, Default)]
pub struct AlphaNetNode {
    /// Additional Optimism args
    pub args: RollupArgs,
}

impl AlphaNetNode {
    /// Creates a new instance of the Optimism node type.
    pub const fn new(args: RollupArgs) -> Self {
        Self { args }
    }

    /// Returns the components for the given [RollupArgs].
    pub fn components<Node>(
        args: RollupArgs,
    ) -> ComponentsBuilder<
        Node,
        OptimismPoolBuilder,
        OptimismPayloadBuilder<AlphaNetEvmConfig>,
        OptimismNetworkBuilder,
        AlphaNetExecutorBuilder,
    >
    where
        Node: FullNodeTypes<Engine = OptimismEngineTypes>,
    {
        let RollupArgs { disable_txpool_gossip, compute_pending_block, .. } = args;
        ComponentsBuilder::default()
            .node_types::<Node>()
            .pool(OptimismPoolBuilder::default())
            .payload(OptimismPayloadBuilder::new(
                compute_pending_block,
                AlphaNetEvmConfig::default(),
            ))
            .network(OptimismNetworkBuilder { disable_txpool_gossip })
            .executor(AlphaNetExecutorBuilder::default())
    }
}

/// Configure the node types
impl NodeTypes for AlphaNetNode {
    type Primitives = ();
    type Engine = OptimismEngineTypes;
}

impl<N> Node<N> for AlphaNetNode
where
    N: FullNodeTypes<Engine = OptimismEngineTypes>,
{
    type ComponentsBuilder = ComponentsBuilder<
        N,
        OptimismPoolBuilder,
        OptimismPayloadBuilder<AlphaNetEvmConfig>,
        OptimismNetworkBuilder,
        AlphaNetExecutorBuilder,
    >;

    fn components_builder(self) -> Self::ComponentsBuilder {
        let Self { args } = self;
        Self::components(args)
    }
}

/// The AlphaNet evm and executor builder.
#[derive(Debug, Default, Clone, Copy)]
#[non_exhaustive]
pub struct AlphaNetExecutorBuilder;

impl<Node> ExecutorBuilder<Node> for AlphaNetExecutorBuilder
where
    Node: FullNodeTypes,
{
    type EVM = AlphaNetEvmConfig;
    type Executor = OpExecutorProvider<Self::EVM>;

    async fn build_evm(
        self,
        ctx: &BuilderContext<Node>,
    ) -> eyre::Result<(Self::EVM, Self::Executor)> {
        let chain_spec = ctx.chain_spec();
        let evm_config = AlphaNetEvmConfig::default();
        let executor = OpExecutorProvider::new(chain_spec, evm_config);

        Ok((evm_config, executor))
    }
}
