# Lotus Rust Client Schema

A Rust client library for interacting with the [Lotus](https://github.com/filecoin-project/lotus) JSON-RPC API. This library provides a type-safe, async interface for communicating with Lotus nodes.

**This has not been rigorously tested! Contributions and feedback are welcome.**

## Features

- Async/await support using Tokio runtime
- Type-safe JSON-RPC interactions
- Comprehensive error handling
- Support for authentication tokens
- Generated API bindings for all Lotus APIs:
  - Full Node API
  - Storage Miner API
  - Gateway API
  - Wallet API
  - Worker API
  - Common API

## Installation

Add this to your `Cargo.toml`:
```toml
[dependencies]
lotus-client = "0.1.0"
```

## Usage
```rust
use lotus_client::LotusClient;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new client
    let client = LotusClient::new(
        "http://127.0.0.1:1234/rpc/v0",
        Some("YOUR_AUTH_TOKEN".to_string())
    ).await?;
    // Use the client to make API calls
    let version = client.version(serde_json::Value::Null).await?;
    println!("Lotus version: {:?}", version
);
    Ok(())
}
```

## API Coverage

The client provides comprehensive coverage of the Lotus JSON-RPC API through generated code:

- `CommonApi` - Common methods shared across different APIs
- `FullNodeApi` - Full node API methods
- `StorageMinerApi` - Storage miner specific methods
- `GatewayApi` - Gateway API methods
- `WalletApi` - Wallet management methods
- `WorkerApi` - Worker API methods

### Codegen
The API bindings are generated from the Lotus JSON-RPC API schema using a custom code generator. To regenerate the bindings:

1. Ensure you have Go installed
2. Run the schema generator script:
```bash
chmod +x ./go-schemagen/generate-schema-for-api.sh
./go-schemagen/generate-schema-for-api.sh
```

## Error Handling

The library uses custom error types for better error handling:

- `ClientError` - Errors from the JSON-RPC client
- `SerializationError` - JSON serialization/deserialization errors
- `JsonRpcError` - Errors returned by the Lotus JSON-RPC API

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- [js-lotus-client-schema](https://github.com/filecoin-shipyard/js-lotus-client-schema) - The schema generator used to generate Rust API code