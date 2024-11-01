// Code generated by github.com/tangle-network/lotus-client-schema-rs/rust-schemagen. DO NOT EDIT.

// Code generated by github.com/tangle-network/lotus-client-schema-rs/rust-schemagen. DO NOT EDIT.

use crate::client::LotusClient;
use jsonrpc_core::Error;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::mpsc;

#[async_trait::async_trait]
pub trait GatewayApi {
    async fn chain_get_block_messages(&self, param1: Value, param2: Value) -> Result<Value, Error>;

    async fn chain_get_genesis(&self, param1: Value) -> Result<Value, Error>;

    async fn chain_get_message(&self, param1: Value, param2: Value) -> Result<Value, Error>;

    async fn chain_get_parent_messages(&self, param1: Value, param2: Value)
        -> Result<Value, Error>;

    async fn chain_get_parent_receipts(&self, param1: Value, param2: Value)
        -> Result<Value, Error>;

    async fn chain_get_path(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
    ) -> Result<Value, Error>;

    async fn chain_get_tip_set(&self, param1: Value, param2: Value) -> Result<Value, Error>;

    async fn chain_get_tip_set_after_height(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
    ) -> Result<Value, Error>;

    async fn chain_get_tip_set_by_height(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
    ) -> Result<Value, Error>;

    async fn chain_has_obj(&self, param1: Value, param2: Value) -> Result<Value, Error>;

    async fn chain_head(&self, param1: Value) -> Result<Value, Error>;

    async fn chain_notify(&self, param1: Value) -> Result<mpsc::Receiver<Value>, Error>;

    async fn chain_read_obj(&self, param1: Value, param2: Value) -> Result<Value, Error>;

    async fn gas_estimate_message_gas(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
        param4: Value,
    ) -> Result<Value, Error>;

    async fn mpool_push(&self, param1: Value, param2: Value) -> Result<Value, Error>;

    async fn msig_get_available_balance(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
    ) -> Result<Value, Error>;

    async fn msig_get_pending(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
    ) -> Result<Value, Error>;

    async fn msig_get_vested(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
        param4: Value,
    ) -> Result<Value, Error>;

    async fn state_account_key(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
    ) -> Result<Value, Error>;

    async fn state_deal_provider_collateral_bounds(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
        param4: Value,
    ) -> Result<Value, Error>;

    async fn state_get_actor(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
    ) -> Result<Value, Error>;

    async fn state_list_miners(&self, param1: Value, param2: Value) -> Result<Value, Error>;

    async fn state_lookup_id(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
    ) -> Result<Value, Error>;

    async fn state_market_balance(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
    ) -> Result<Value, Error>;

    async fn state_market_storage_deal(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
    ) -> Result<Value, Error>;

    async fn state_miner_info(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
    ) -> Result<Value, Error>;

    async fn state_miner_power(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
    ) -> Result<Value, Error>;

    async fn state_miner_proving_deadline(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
    ) -> Result<Value, Error>;

    async fn state_network_version(&self, param1: Value, param2: Value) -> Result<Value, Error>;

    async fn state_read_state(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
    ) -> Result<Value, Error>;

    async fn state_search_msg(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
        param4: Value,
        param5: Value,
    ) -> Result<Value, Error>;

    async fn state_sector_get_info(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
        param4: Value,
    ) -> Result<Value, Error>;

    async fn state_verified_client_status(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
    ) -> Result<Value, Error>;

    async fn state_wait_msg(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
        param4: Value,
        param5: Value,
    ) -> Result<Value, Error>;

    async fn version(&self, param1: Value) -> Result<Value, Error>;

    async fn wallet_balance(&self, param1: Value, param2: Value) -> Result<Value, Error>;
}

#[derive(Debug, Clone)]
pub struct GatewayClient {
    client: LotusClient,
}

#[async_trait::async_trait]
impl GatewayApi for GatewayClient {
    async fn chain_get_block_messages(&self, param1: Value, param2: Value) -> Result<Value, Error> {
        self.client
            .request("ChainGetBlockMessages", vec![param1, param2])
            .await
    }

    async fn chain_get_genesis(&self, param1: Value) -> Result<Value, Error> {
        self.client.request("ChainGetGenesis", vec![param1]).await
    }

    async fn chain_get_message(&self, param1: Value, param2: Value) -> Result<Value, Error> {
        self.client
            .request("ChainGetMessage", vec![param1, param2])
            .await
    }

    async fn chain_get_parent_messages(
        &self,
        param1: Value,
        param2: Value,
    ) -> Result<Value, Error> {
        self.client
            .request("ChainGetParentMessages", vec![param1, param2])
            .await
    }

    async fn chain_get_parent_receipts(
        &self,
        param1: Value,
        param2: Value,
    ) -> Result<Value, Error> {
        self.client
            .request("ChainGetParentReceipts", vec![param1, param2])
            .await
    }

    async fn chain_get_path(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
    ) -> Result<Value, Error> {
        self.client
            .request("ChainGetPath", vec![param1, param2, param3])
            .await
    }

    async fn chain_get_tip_set(&self, param1: Value, param2: Value) -> Result<Value, Error> {
        self.client
            .request("ChainGetTipSet", vec![param1, param2])
            .await
    }

    async fn chain_get_tip_set_after_height(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
    ) -> Result<Value, Error> {
        self.client
            .request("ChainGetTipSetAfterHeight", vec![param1, param2, param3])
            .await
    }

    async fn chain_get_tip_set_by_height(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
    ) -> Result<Value, Error> {
        self.client
            .request("ChainGetTipSetByHeight", vec![param1, param2, param3])
            .await
    }

    async fn chain_has_obj(&self, param1: Value, param2: Value) -> Result<Value, Error> {
        self.client
            .request("ChainHasObj", vec![param1, param2])
            .await
    }

    async fn chain_head(&self, param1: Value) -> Result<Value, Error> {
        self.client.request("ChainHead", vec![param1]).await
    }

    async fn chain_notify(&self, param1: Value) -> Result<mpsc::Receiver<Value>, Error> {
        self.client.subscribe("ChainNotify", vec![param1]).await
    }

    async fn chain_read_obj(&self, param1: Value, param2: Value) -> Result<Value, Error> {
        self.client
            .request("ChainReadObj", vec![param1, param2])
            .await
    }

    async fn gas_estimate_message_gas(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
        param4: Value,
    ) -> Result<Value, Error> {
        self.client
            .request(
                "GasEstimateMessageGas",
                vec![param1, param2, param3, param4],
            )
            .await
    }

    async fn mpool_push(&self, param1: Value, param2: Value) -> Result<Value, Error> {
        self.client.request("MpoolPush", vec![param1, param2]).await
    }

    async fn msig_get_available_balance(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
    ) -> Result<Value, Error> {
        self.client
            .request("MsigGetAvailableBalance", vec![param1, param2, param3])
            .await
    }

    async fn msig_get_pending(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
    ) -> Result<Value, Error> {
        self.client
            .request("MsigGetPending", vec![param1, param2, param3])
            .await
    }

    async fn msig_get_vested(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
        param4: Value,
    ) -> Result<Value, Error> {
        self.client
            .request("MsigGetVested", vec![param1, param2, param3, param4])
            .await
    }

    async fn state_account_key(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
    ) -> Result<Value, Error> {
        self.client
            .request("StateAccountKey", vec![param1, param2, param3])
            .await
    }

    async fn state_deal_provider_collateral_bounds(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
        param4: Value,
    ) -> Result<Value, Error> {
        self.client
            .request(
                "StateDealProviderCollateralBounds",
                vec![param1, param2, param3, param4],
            )
            .await
    }

    async fn state_get_actor(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
    ) -> Result<Value, Error> {
        self.client
            .request("StateGetActor", vec![param1, param2, param3])
            .await
    }

    async fn state_list_miners(&self, param1: Value, param2: Value) -> Result<Value, Error> {
        self.client
            .request("StateListMiners", vec![param1, param2])
            .await
    }

    async fn state_lookup_id(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
    ) -> Result<Value, Error> {
        self.client
            .request("StateLookupID", vec![param1, param2, param3])
            .await
    }

    async fn state_market_balance(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
    ) -> Result<Value, Error> {
        self.client
            .request("StateMarketBalance", vec![param1, param2, param3])
            .await
    }

    async fn state_market_storage_deal(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
    ) -> Result<Value, Error> {
        self.client
            .request("StateMarketStorageDeal", vec![param1, param2, param3])
            .await
    }

    async fn state_miner_info(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
    ) -> Result<Value, Error> {
        self.client
            .request("StateMinerInfo", vec![param1, param2, param3])
            .await
    }

    async fn state_miner_power(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
    ) -> Result<Value, Error> {
        self.client
            .request("StateMinerPower", vec![param1, param2, param3])
            .await
    }

    async fn state_miner_proving_deadline(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
    ) -> Result<Value, Error> {
        self.client
            .request("StateMinerProvingDeadline", vec![param1, param2, param3])
            .await
    }

    async fn state_network_version(&self, param1: Value, param2: Value) -> Result<Value, Error> {
        self.client
            .request("StateNetworkVersion", vec![param1, param2])
            .await
    }

    async fn state_read_state(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
    ) -> Result<Value, Error> {
        self.client
            .request("StateReadState", vec![param1, param2, param3])
            .await
    }

    async fn state_search_msg(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
        param4: Value,
        param5: Value,
    ) -> Result<Value, Error> {
        self.client
            .request(
                "StateSearchMsg",
                vec![param1, param2, param3, param4, param5],
            )
            .await
    }

    async fn state_sector_get_info(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
        param4: Value,
    ) -> Result<Value, Error> {
        self.client
            .request("StateSectorGetInfo", vec![param1, param2, param3, param4])
            .await
    }

    async fn state_verified_client_status(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
    ) -> Result<Value, Error> {
        self.client
            .request("StateVerifiedClientStatus", vec![param1, param2, param3])
            .await
    }

    async fn state_wait_msg(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
        param4: Value,
        param5: Value,
    ) -> Result<Value, Error> {
        self.client
            .request("StateWaitMsg", vec![param1, param2, param3, param4, param5])
            .await
    }

    async fn version(&self, param1: Value) -> Result<Value, Error> {
        self.client.request("Version", vec![param1]).await
    }

    async fn wallet_balance(&self, param1: Value, param2: Value) -> Result<Value, Error> {
        self.client
            .request("WalletBalance", vec![param1, param2])
            .await
    }
}
