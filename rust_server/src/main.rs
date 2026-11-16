use axum::{
    body::Body,
    extract::{Path, State},
    http::{header, HeaderMap, HeaderName, HeaderValue, Method, Request, Response, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use bytes::Bytes;
use moka::future::Cache;
use reqwest::Client;
use std::{net::SocketAddr, path::PathBuf, sync::Arc, time::Duration};
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};
use tracing::{error, info};

const PORT: u16 = 8080;
const PUBLIC_DIR: &str = r"C:\Users\shiva\Windy_App\extracted\assets\public";

#[derive(Clone)]
struct AppState {
    client: Client,
    tile_cache: Cache<String, (StatusCode, String, Bytes)>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // High-performance Reqwest client with pooled connections
    let client = Client::builder()
        .pool_max_idle_per_host(100)
        .pool_idle_timeout(Duration::from_secs(60))
        .tcp_nodelay(true)
        .timeout(Duration::from_secs(12))
        .build()
        .expect("Failed to create HTTP client");

    // High-concurrency Moka cache for 5,000 tiles
    let tile_cache = Cache::builder()
        .max_capacity(5_000)
        .time_to_idle(Duration::from_secs(3600))
        .build();

    let state = Arc::new(AppState {
        client,
        tile_cache,
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(Any);

    let serve_dir = ServeDir::new(PathBuf::from(PUBLIC_DIR))
        .append_index_html_on_directories(true);

    let app = Router::new()
        // Proxy routes
        .route("/rdr/*path", get(handle_rdr))
        .route("/sat/*path", get(handle_sat))
        .route("/ims/*path", get(handle_ims))
        .route("/tiles/*path", get(handle_tiles))
        .route("/services/*path", get(handle_node_services))
        .route("/metadata/*path", get(handle_node_metadata))
        .route("/maptile/*path", get(handle_node_maptile))
        .route("/tc/*path", get(handle_node_tc))
        .route("/users/*path", get(handle_node_users))
        // Static assets fallback
        .fallback_service(serve_dir)
        .layer(cors)
        .with_state(state);

    let addr_v4 = SocketAddr::from(([0, 0, 0, 0], PORT));
    let addr_v6 = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 1], PORT));

    info!("Bhartiya Marut Kendra Ultra-Fast Rust Server running on http://127.0.0.1:{}", PORT);
    println!("Bhartiya Marut Kendra Ultra-Fast Rust Server listening on port {} (IPv4 + IPv6 dual-stack)", PORT);

    let listener_v4 = tokio::net::TcpListener::bind(addr_v4)
        .await
        .expect("Failed to bind IPv4");

    let app_v6 = app.clone();
    if let Ok(listener_v6) = tokio::net::TcpListener::bind(addr_v6).await {
        tokio::spawn(async move {
            let _ = axum::serve(listener_v6, app_v6).await;
        });
    }

    axum::serve(listener_v4, app)
        .await
        .expect("Server failed");
}

async fn handle_rdr(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
    req: Request<Body>,
) -> impl IntoResponse {
    let query = req.uri().query().unwrap_or("");
    let url = if query.is_empty() {
        format!("https://rdr.windy.com/{}", path)
    } else {
        format!("https://rdr.windy.com/{}?{}", path, query)
    };
    proxy_request(&state, &url, false, "").await
}

async fn handle_sat(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
    req: Request<Body>,
) -> impl IntoResponse {
    let query = req.uri().query().unwrap_or("");
    let url = if query.is_empty() {
        format!("https://sat.windy.com/{}", path)
    } else {
        format!("https://sat.windy.com/{}?{}", path, query)
    };
    proxy_request(&state, &url, false, "").await
}

async fn handle_ims(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
    req: Request<Body>,
) -> impl IntoResponse {
    let query = req.uri().query().unwrap_or("");
    let cache_key = req.uri().to_string();
    let url = if query.is_empty() {
        format!("https://ims.windy.com/{}", path)
    } else {
        format!("https://ims.windy.com/{}?{}", path, query)
    };
    proxy_request(&state, &url, true, &cache_key).await
}

async fn handle_tiles(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
    req: Request<Body>,
) -> impl IntoResponse {
    let query = req.uri().query().unwrap_or("");
    let cache_key = req.uri().to_string();
    let url = if query.is_empty() {
        format!("https://tiles.windy.com/tiles/{}", path)
    } else {
        format!("https://tiles.windy.com/tiles/{}?{}", path, query)
    };
    proxy_request(&state, &url, true, &cache_key).await
}

async fn handle_node_services(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
    req: Request<Body>,
) -> impl IntoResponse {
    handle_node_proxy(state, format!("/services/{}", path), req).await
}

async fn handle_node_metadata(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
    req: Request<Body>,
) -> impl IntoResponse {
    handle_node_proxy(state, format!("/metadata/{}", path), req).await
}

async fn handle_node_maptile(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
    req: Request<Body>,
) -> impl IntoResponse {
    handle_node_proxy(state, format!("/maptile/{}", path), req).await
}

async fn handle_node_tc(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
    req: Request<Body>,
) -> impl IntoResponse {
    handle_node_proxy(state, format!("/tc/{}", path), req).await
}

async fn handle_node_users(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
    req: Request<Body>,
) -> impl IntoResponse {
    handle_node_proxy(state, format!("/users/{}", path), req).await
}

async fn handle_node_proxy(
    state: Arc<AppState>,
    path: String,
    req: Request<Body>,
) -> impl IntoResponse {
    let query = req.uri().query().unwrap_or("");
    let url = if query.is_empty() {
        format!("https://node.windy.com{}", path)
    } else {
        format!("https://node.windy.com{}?{}", path, query)
    };
    proxy_request(&state, &url, false, "").await
}

async fn proxy_request(
    state: &AppState,
    url: &str,
    is_tile: bool,
    cache_key: &str,
) -> Response<Body> {
    // Check LRU cache for tiles
    if is_tile && !cache_key.is_empty() {
        if let Some((status, content_type, bytes)) = state.tile_cache.get(cache_key).await {
            let mut res = Response::builder()
                .status(status)
                .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
                .header(header::CACHE_CONTROL, "public, max-age=86400")
                .header(header::CONTENT_LENGTH, bytes.len().to_string());

            if !content_type.is_empty() {
                res = res.header(header::CONTENT_TYPE, content_type);
            }

            return res.body(Body::from(bytes)).unwrap_or_else(|_| {
                Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::empty())
                    .unwrap()
            });
        }
    }

    // Fetch upstream with pooled connection
    match state
        .client
        .get(url)
        .header(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36",
        )
        .send()
        .await
    {
        Ok(upstream_resp) => {
            let status = StatusCode::from_u16(upstream_resp.status().as_u16())
                .unwrap_or(StatusCode::OK);

            let content_type = upstream_resp
                .headers()
                .get(header::CONTENT_TYPE)
                .and_then(|v| v.to_str().ok())
                .unwrap_or("")
                .to_string();

            match upstream_resp.bytes().await {
                Ok(bytes) => {
                    // Cache successful tile responses
                    if is_tile && status == StatusCode::OK && !cache_key.is_empty() {
                        state
                            .tile_cache
                            .insert(cache_key.to_string(), (status, content_type.clone(), bytes.clone()))
                            .await;
                    }

                    let mut res = Response::builder()
                        .status(status)
                        .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
                        .header(header::CONTENT_LENGTH, bytes.len().to_string());

                    if is_tile {
                        res = res.header(header::CACHE_CONTROL, "public, max-age=86400");
                    } else {
                        res = res.header(header::CACHE_CONTROL, "no-cache");
                    }

                    if !content_type.is_empty() {
                        res = res.header(header::CONTENT_TYPE, content_type);
                    }

                    res.body(Body::from(bytes)).unwrap_or_else(|_| {
                        Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Body::empty())
                            .unwrap()
                    })
                }
                Err(err) => {
                    error!("Error reading bytes from {}: {}", url, err);
                    Response::builder()
                        .status(StatusCode::BAD_GATEWAY)
                        .body(Body::from(format!("Error reading upstream: {}", err)))
                        .unwrap()
                }
            }
        }
        Err(err) => {
            error!("Error fetching from {}: {}", url, err);
            Response::builder()
                .status(StatusCode::BAD_GATEWAY)
                .body(Body::from(format!("Upstream error: {}", err)))
                .unwrap()
        }
    }
}
