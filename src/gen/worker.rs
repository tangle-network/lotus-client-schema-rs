// Code generated by github.com/tangle-network/lotus-client-schema-rs/rust-schemagen. DO NOT EDIT.

// Code generated by github.com/tangle-network/lotus-client-schema-rs/rust-schemagen. DO NOT EDIT.

use crate::client::LotusClient;
use jsonrpc_core::Error;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::mpsc;

#[async_trait::async_trait]
pub trait WorkerApi {
    async fn add_piece(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
        param4: Value,
        param5: Value,
    ) -> Result<Value, Error>;

    async fn enabled(&self, param1: Value) -> Result<Value, Error>;

    async fn fetch(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
        param4: Value,
        param5: Value,
    ) -> Result<Value, Error>;

    async fn finalize_replica_update(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
    ) -> Result<Value, Error>;

    async fn finalize_sector(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
    ) -> Result<Value, Error>;

    async fn generate_sector_key_from_data(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
    ) -> Result<Value, Error>;

    async fn info(&self, param1: Value) -> Result<Value, Error>;

    async fn move_storage(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
    ) -> Result<Value, Error>;

    async fn paths(&self, param1: Value) -> Result<Value, Error>;

    async fn process_session(&self, param1: Value) -> Result<Value, Error>;

    async fn prove_replica_update1(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
        param4: Value,
        param5: Value,
    ) -> Result<Value, Error>;

    async fn prove_replica_update2(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
        param4: Value,
        param5: Value,
        param6: Value,
    ) -> Result<Value, Error>;

    async fn release_unsealed(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
    ) -> Result<Value, Error>;

    async fn remove(&self, param1: Value, param2: Value) -> Result<Value, Error>;

    async fn replica_update(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
    ) -> Result<Value, Error>;

    async fn seal_commit1(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
        param4: Value,
        param5: Value,
        param6: Value,
    ) -> Result<Value, Error>;

    async fn seal_commit2(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
    ) -> Result<Value, Error>;

    async fn seal_pre_commit1(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
        param4: Value,
    ) -> Result<Value, Error>;

    async fn seal_pre_commit2(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
    ) -> Result<Value, Error>;

    async fn session(&self, param1: Value) -> Result<Value, Error>;

    async fn set_enabled(&self, param1: Value, param2: Value) -> Result<Value, Error>;

    async fn storage_add_local(&self, param1: Value, param2: Value) -> Result<Value, Error>;

    async fn task_disable(&self, param1: Value, param2: Value) -> Result<Value, Error>;

    async fn task_enable(&self, param1: Value, param2: Value) -> Result<Value, Error>;

    async fn task_types(&self, param1: Value) -> Result<Value, Error>;

    async fn unseal_piece(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
        param4: Value,
        param5: Value,
        param6: Value,
    ) -> Result<Value, Error>;

    async fn version(&self, param1: Value) -> Result<Value, Error>;

    async fn wait_quiet(&self, param1: Value) -> Result<Value, Error>;
}

#[derive(Debug, Clone)]
pub struct WorkerClient {
    client: LotusClient,
}

#[async_trait::async_trait]
impl WorkerApi for WorkerClient {
    async fn add_piece(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
        param4: Value,
        param5: Value,
    ) -> Result<Value, Error> {
        self.client
            .request("AddPiece", vec![param1, param2, param3, param4, param5])
            .await
    }

    async fn enabled(&self, param1: Value) -> Result<Value, Error> {
        self.client.request("Enabled", vec![param1]).await
    }

    async fn fetch(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
        param4: Value,
        param5: Value,
    ) -> Result<Value, Error> {
        self.client
            .request("Fetch", vec![param1, param2, param3, param4, param5])
            .await
    }

    async fn finalize_replica_update(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
    ) -> Result<Value, Error> {
        self.client
            .request("FinalizeReplicaUpdate", vec![param1, param2, param3])
            .await
    }

    async fn finalize_sector(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
    ) -> Result<Value, Error> {
        self.client
            .request("FinalizeSector", vec![param1, param2, param3])
            .await
    }

    async fn generate_sector_key_from_data(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
    ) -> Result<Value, Error> {
        self.client
            .request("GenerateSectorKeyFromData", vec![param1, param2, param3])
            .await
    }

    async fn info(&self, param1: Value) -> Result<Value, Error> {
        self.client.request("Info", vec![param1]).await
    }

    async fn move_storage(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
    ) -> Result<Value, Error> {
        self.client
            .request("MoveStorage", vec![param1, param2, param3])
            .await
    }

    async fn paths(&self, param1: Value) -> Result<Value, Error> {
        self.client.request("Paths", vec![param1]).await
    }

    async fn process_session(&self, param1: Value) -> Result<Value, Error> {
        self.client.request("ProcessSession", vec![param1]).await
    }

    async fn prove_replica_update1(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
        param4: Value,
        param5: Value,
    ) -> Result<Value, Error> {
        self.client
            .request(
                "ProveReplicaUpdate1",
                vec![param1, param2, param3, param4, param5],
            )
            .await
    }

    async fn prove_replica_update2(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
        param4: Value,
        param5: Value,
        param6: Value,
    ) -> Result<Value, Error> {
        self.client
            .request(
                "ProveReplicaUpdate2",
                vec![param1, param2, param3, param4, param5, param6],
            )
            .await
    }

    async fn release_unsealed(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
    ) -> Result<Value, Error> {
        self.client
            .request("ReleaseUnsealed", vec![param1, param2, param3])
            .await
    }

    async fn remove(&self, param1: Value, param2: Value) -> Result<Value, Error> {
        self.client.request("Remove", vec![param1, param2]).await
    }

    async fn replica_update(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
    ) -> Result<Value, Error> {
        self.client
            .request("ReplicaUpdate", vec![param1, param2, param3])
            .await
    }

    async fn seal_commit1(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
        param4: Value,
        param5: Value,
        param6: Value,
    ) -> Result<Value, Error> {
        self.client
            .request(
                "SealCommit1",
                vec![param1, param2, param3, param4, param5, param6],
            )
            .await
    }

    async fn seal_commit2(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
    ) -> Result<Value, Error> {
        self.client
            .request("SealCommit2", vec![param1, param2, param3])
            .await
    }

    async fn seal_pre_commit1(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
        param4: Value,
    ) -> Result<Value, Error> {
        self.client
            .request("SealPreCommit1", vec![param1, param2, param3, param4])
            .await
    }

    async fn seal_pre_commit2(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
    ) -> Result<Value, Error> {
        self.client
            .request("SealPreCommit2", vec![param1, param2, param3])
            .await
    }

    async fn session(&self, param1: Value) -> Result<Value, Error> {
        self.client.request("Session", vec![param1]).await
    }

    async fn set_enabled(&self, param1: Value, param2: Value) -> Result<Value, Error> {
        self.client
            .request("SetEnabled", vec![param1, param2])
            .await
    }

    async fn storage_add_local(&self, param1: Value, param2: Value) -> Result<Value, Error> {
        self.client
            .request("StorageAddLocal", vec![param1, param2])
            .await
    }

    async fn task_disable(&self, param1: Value, param2: Value) -> Result<Value, Error> {
        self.client
            .request("TaskDisable", vec![param1, param2])
            .await
    }

    async fn task_enable(&self, param1: Value, param2: Value) -> Result<Value, Error> {
        self.client
            .request("TaskEnable", vec![param1, param2])
            .await
    }

    async fn task_types(&self, param1: Value) -> Result<Value, Error> {
        self.client.request("TaskTypes", vec![param1]).await
    }

    async fn unseal_piece(
        &self,
        param1: Value,
        param2: Value,
        param3: Value,
        param4: Value,
        param5: Value,
        param6: Value,
    ) -> Result<Value, Error> {
        self.client
            .request(
                "UnsealPiece",
                vec![param1, param2, param3, param4, param5, param6],
            )
            .await
    }

    async fn version(&self, param1: Value) -> Result<Value, Error> {
        self.client.request("Version", vec![param1]).await
    }

    async fn wait_quiet(&self, param1: Value) -> Result<Value, Error> {
        self.client.request("WaitQuiet", vec![param1]).await
    }
}
