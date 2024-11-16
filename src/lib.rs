pub mod client;
pub mod error;
pub mod gen;

pub use client::LotusClient;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use gen::{common::{CommonApi, CommonClient}, fullnode::{Cid, FullNodeApi, FullNodeClient, TipSetKey}};
    use serde_json::Value;

    #[tokio::test]
    async fn test_new_client() {
        let client = LotusClient::new("https://api.node.glif.io/rpc/v1", None)
            .await
            .unwrap();
        let common_client = CommonClient::new(client.clone());
        let result: HashMap<String, Value> = common_client.discover().await.unwrap();
        assert!(!result.is_empty());
    
        let full_node_client = FullNodeClient::new(client);
        
        // Test with None (current tipset)
        let result = full_node_client.state_list_miners(None).await.unwrap();
        assert!(!result.is_empty());
    
        // Test with Some tipset
        let result = full_node_client.state_list_miners(Some(TipSetKey {
            cids: vec![Cid {
                str: "bafy2bzacedboojlbcn72jrrmfjbqii4dxidtro62oszxbupzu4yq37loqc6i6".to_string(),
            }]
        })).await.unwrap();
        assert!(!result.is_empty());
        assert!(!result.is_empty());
    }
}
