use hyper::header::AUTHORIZATION;
use jsonrpc_core::ErrorCode;
use jsonrpsee::{
    core::client::ClientT,
    http_client::{HeaderMap, HttpClient, HttpClientBuilder},
};
use jsonrpsee_core::{client::SubscriptionClientT, params::ArrayParams};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::RwLock;

use crate::error::LotusError;

/// LotusClient provides a high-level interface for interacting with a Lotus node
/// through JSON-RPC.
#[derive(Clone, Debug)]
pub struct LotusClient {
    client: Arc<HttpClient>,
    auth_token: Arc<RwLock<Option<String>>>,
}

impl LotusClient {
    /// Creates a new LotusClient instance
    pub async fn new(url: &str, auth_token: Option<String>) -> Result<Self, LotusError> {
        let mut header_map: HeaderMap = HeaderMap::new();
        if let Some(token) = &auth_token {
            header_map.insert(AUTHORIZATION, format!("Bearer {}", token).parse().unwrap());
        }

        let client = HttpClientBuilder::default()
            .set_headers(header_map)
            .build(url)
            .map_err(LotusError::ClientError)?;

        Ok(Self {
            client: Arc::new(client),
            auth_token: Arc::new(RwLock::new(auth_token)),
        })
    }

    /// Updates the authentication token
    pub async fn set_auth_token(&self, token: Option<String>) {
        let mut auth_token = self.auth_token.write().await;
        *auth_token = token;
    }

    /// Makes a JSON-RPC request to the Lotus node
    pub async fn request<T: serde::de::DeserializeOwned>(
        &self,
        method: &str,
        params: Vec<Value>,
    ) -> Result<T, jsonrpc_core::Error> {
        let params = self.params(params)?;
        self.client
            .request(method, params)
            .await
            .map_err(Self::handle_client_error)
    }

    /// Makes a JSON-RPC subscription request to the Lotus node
    pub async fn subscribe<T: serde::de::DeserializeOwned + Send + 'static>(
        &self,
        method: &str,
        params: Vec<Value>,
    ) -> Result<mpsc::Receiver<T>, jsonrpc_core::Error> {
        let params = self.params(params)?;
        let (tx, rx) = mpsc::channel(100);

        let mut subscription = self
            .client
            .subscribe(method, params, "subscription")
            .await
            .map_err(Self::handle_client_error)?;

        tokio::spawn(async move {
            while let Some(Ok(msg)) = subscription.next().await {
                if let Ok(value) = serde_json::from_value(msg) {
                    if tx.send(value).await.is_err() {
                        break;
                    }
                }
            }
        });

        Ok(rx)
    }

    /// Generate params for a JSON-RPC request
    pub fn params(&self, params: Vec<Value>) -> Result<ArrayParams, jsonrpc_core::Error> {
        if params.is_empty() {
            Ok(ArrayParams::new())
        } else {
            // Convert params into individual array elements
            let mut array_params = ArrayParams::new();
            for param in params {
                array_params
                    .insert(param)
                    .map_err(|e| jsonrpc_core::Error {
                        code: ErrorCode::InvalidParams,
                        message: e.to_string(),
                        data: None,
                    })?;
            }
            Ok(array_params)
        }
    }

    /// Converts a jsonrpsee ClientError into a jsonrpc_core Error
    fn handle_client_error(error: jsonrpsee_core::ClientError) -> jsonrpc_core::Error {
        match error {
            jsonrpsee_core::ClientError::Call(error_object) => jsonrpc_core::Error {
                code: ErrorCode::from(error_object.code() as i64),
                message: error_object.message().to_string(),
                data: error_object
                    .data()
                    .map(|d| Value::String(d.get().to_string())),
            },
            jsonrpsee_core::ClientError::Transport(error) => jsonrpc_core::Error {
                code: ErrorCode::InternalError,
                message: error.to_string(),
                data: None,
            },
            jsonrpsee_core::ClientError::RestartNeeded(arc) => jsonrpc_core::Error {
                code: ErrorCode::InternalError,
                message: arc.to_string(),
                data: None,
            },
            jsonrpsee_core::ClientError::ParseError(error) => jsonrpc_core::Error {
                code: ErrorCode::ParseError,
                message: error.to_string(),
                data: None,
            },
            jsonrpsee_core::ClientError::InvalidSubscriptionId => jsonrpc_core::Error {
                code: ErrorCode::InvalidParams,
                message: "Invalid subscription ID".to_string(),
                data: None,
            },
            jsonrpsee_core::ClientError::InvalidRequestId(invalid_request_id) => {
                jsonrpc_core::Error {
                    code: ErrorCode::InvalidRequest,
                    message: invalid_request_id.to_string(),
                    data: None,
                }
            }
            jsonrpsee_core::ClientError::RequestTimeout => jsonrpc_core::Error {
                code: ErrorCode::InternalError,
                message: "Request timeout".to_string(),
                data: None,
            },
            jsonrpsee_core::ClientError::Custom(error) => jsonrpc_core::Error {
                code: ErrorCode::InternalError,
                message: error.to_string(),
                data: None,
            },
            jsonrpsee_core::ClientError::HttpNotImplemented => jsonrpc_core::Error {
                code: ErrorCode::InternalError,
                message: "HTTP not implemented".to_string(),
                data: None,
            },
            jsonrpsee_core::ClientError::EmptyBatchRequest(empty_batch_request) => {
                jsonrpc_core::Error {
                    code: ErrorCode::InvalidRequest,
                    message: empty_batch_request.to_string(),
                    data: None,
                }
            }
            jsonrpsee_core::ClientError::RegisterMethod(register_method_error) => {
                jsonrpc_core::Error {
                    code: ErrorCode::InternalError,
                    message: register_method_error.to_string(),
                    data: None,
                }
            }
        }
    }
}
