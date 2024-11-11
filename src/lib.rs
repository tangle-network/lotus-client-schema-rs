pub mod client;
pub mod error;
pub mod gen;

pub use client::LotusClient;

#[cfg(test)]
mod tests {
    use super::*;
    use gen::common::{CommonApi, CommonClient};

    #[tokio::test]
    async fn test_new_client() {
        let client = LotusClient::new("https://api.node.glif.io/rpc/v1", None)
            .await
            .unwrap();
        let common_client = CommonClient::new(client);
        let result = common_client.discover().await.unwrap();
        println!("{:?}", result);
    }
}
