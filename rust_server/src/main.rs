use axum::{
    body::Body,
    extract::{Path, State},
    http::{header, Method, Request, Response, StatusCode},
    middleware::{self, Next},
    response::IntoResponse,
    routing::get,
    Router,
};
use bytes::Bytes;
use moka::future::Cache;
use reqwest::Client;
use std::{
    hash::{Hash, Hasher},
    net::SocketAddr,
    path::{Path as StdPath, PathBuf},
    sync::Arc,
    time::Duration,
};
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};
use tracing::error;

const PORT: u16 = 8080;
const PUBLIC_DIR: &str = r"C:\Users\shiva\Windy_App\extracted\assets\public";
const TILE_CACHE_DIR: &str = r"C:\Users\shiva\Windy_App\tile_cache";

#[derive(Clone)]
struct AppState {
    client: Client,
    tile_cache: Cache<String, (StatusCode, String, Bytes)>,
    disk_cache_dir: PathBuf,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let disk_cache_dir = PathBuf::from(TILE_CACHE_DIR);
    if let Err(e) = tokio::fs::create_dir_all(&disk_cache_dir).await {
        error!("Failed to create tile cache dir: {}", e);
    }

    // High-performance Reqwest client with pooled connections
    let client = Client::builder()
        .pool_max_idle_per_host(100)
        .pool_idle_timeout(Duration::from_secs(60))
        .tcp_nodelay(true)
        .timeout(Duration::from_secs(12))
        .build()
        .expect("Failed to build HTTP client");

    // Two-tier cache: 10,000 tiles in high-speed RAM, TTL 24h
    let tile_cache = Cache::builder()
        .max_capacity(10_000)
        .time_to_live(Duration::from_secs(86400))
        .build();

    let state = Arc::new(AppState {
        client,
        tile_cache,
        disk_cache_dir,
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers(Any);

    let serve_dir = ServeDir::new(PathBuf::from(PUBLIC_DIR))
        .append_index_html_on_directories(true);

    let app = Router::new()
        // Core Proxy routes
        .route("/rdr/*path", get(handle_rdr))
        .route("/sat/*path", get(handle_sat))
        .route("/ims/*path", get(handle_ims))
        .route("/tiles/*path", get(handle_tiles))
        .route("/services/*path", get(handle_node_services))
        .route("/metadata/*path", get(handle_node_metadata))
        .route("/maptile/*path", get(handle_node_maptile))
        .route("/tc/*path", get(handle_node_tc))
        .route("/forecast/*path", get(handle_node_forecast))
        .route("/search/*path", get(handle_node_search))
        .route("/webcams/*path", get(handle_node_webcams))
        .route("/citytile/*path", get(handle_node_citytile))
        .route("/capalerts/*path", get(handle_node_capalerts))
        .route("/tides/*path", get(handle_node_tides))
        .route("/pois/*path", get(handle_node_pois))
        .route("/obs/*path", get(handle_node_obs))
        // Upstream Domain Proxies (Masks all 5 external Windy domains)
        .route("/upstream-tiles/*path", get(handle_upstream_tiles))
        .route("/labels/*path", get(handle_upstream_labels))
        .route("/upstream-node/*path", get(handle_upstream_node).post(handle_upstream_node))
        .route("/upstream-nodes/*path", get(handle_upstream_nodes))
        .route("/upstream-account/*path", get(handle_upstream_account).post(handle_upstream_account))
        .route("/upstream-www/*path", get(handle_upstream_www))
        .route("/patch/*path", get(handle_patch))
        .route(
            "/users/*path",
            get(handle_node_users)
                .post(handle_users_data)
                .put(handle_users_data)
                .delete(handle_users_data),
        )
        .route("/plugins/list", get(handle_plugins_list))
        .route("/api/*path", get(handle_api).post(handle_api))
        // Static assets fallback
        .fallback_service(serve_dir)
        .layer(middleware::from_fn(add_no_cache_headers))
        .layer(cors)
        .with_state(state);

    let addr_v4 = SocketAddr::from(([0, 0, 0, 0], PORT));
    let addr_v6 = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 1], PORT));

    println!("Bhartiya Marut Kendra Ultra-Fast Rust Server listening on port {} (IPv4 + IPv6 dual-stack)", PORT);
    println!("Persistent Two-Tier Tile Cache: Tier 1 (RAM Moka) + Tier 2 (Disk: {})", TILE_CACHE_DIR);

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

async fn add_no_cache_headers(req: Request<Body>, next: Next) -> Response<Body> {
    let mut resp = next.run(req).await;
    // Don't disable tile caching, only static assets, js, css, html
    if !resp.headers().contains_key("X-Cache") {
        resp.headers_mut().insert(
            header::CACHE_CONTROL,
            header::HeaderValue::from_static("no-cache, no-store, must-revalidate"),
        );
        resp.headers_mut().insert(
            header::PRAGMA,
            header::HeaderValue::from_static("no-cache"),
        );
    }
    resp
}

fn get_cache_paths(disk_dir: &StdPath, key: &str) -> (PathBuf, PathBuf) {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    key.hash(&mut hasher);
    let hash = format!("{:016x}", hasher.finish());
    let prefix = &hash[0..2];
    let dir = disk_dir.join(prefix);
    (dir.join(format!("{}.bin", hash)), dir.join(format!("{}.meta", hash)))
}

async fn read_disk_cache(
    disk_dir: &StdPath,
    key: &str,
) -> Option<(StatusCode, String, Bytes)> {
    let (bin_path, meta_path) = get_cache_paths(disk_dir, key);
    if !bin_path.exists() {
        return None;
    }

    let bytes = tokio::fs::read(&bin_path).await.ok()?;
    let meta = tokio::fs::read_to_string(&meta_path).await.unwrap_or_else(|_| "image/png".to_string());
    Some((StatusCode::OK, meta, Bytes::from(bytes)))
}

async fn write_disk_cache(
    disk_dir: &StdPath,
    key: &str,
    content_type: &str,
    bytes: &[u8],
) {
    let (bin_path, meta_path) = get_cache_paths(disk_dir, key);
    if let Some(parent) = bin_path.parent() {
        let _ = tokio::fs::create_dir_all(parent).await;
    }

    let _ = tokio::fs::write(&bin_path, bytes).await;
    let _ = tokio::fs::write(&meta_path, content_type).await;
}

fn make_tile_response(
    status: StatusCode,
    content_type: &str,
    bytes: Bytes,
    cache_status: &str,
) -> Response<Body> {
    Response::builder()
        .status(status)
        .header(
            header::CONTENT_TYPE,
            if content_type.is_empty() {
                "image/png"
            } else {
                content_type
            },
        )
        .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .header(header::CACHE_CONTROL, "public, max-age=86400, immutable")
        .header("X-Cache", cache_status)
        .body(Body::from(bytes))
        .unwrap()
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

async fn handle_upstream_tiles(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
    req: Request<Body>,
) -> impl IntoResponse {
    let query = req.uri().query().unwrap_or("");
    let cache_key = format!("/upstream-tiles/{}", path);
    let url = if query.is_empty() {
        format!("https://tiles.windy.com/{}", path)
    } else {
        format!("https://tiles.windy.com/{}?{}", path, query)
    };
    proxy_request(&state, &url, true, &cache_key).await
}

async fn handle_upstream_labels(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
    req: Request<Body>,
) -> impl IntoResponse {
    let query = req.uri().query().unwrap_or("");
    let cache_key = format!("/labels/{}", path);
    let url = if query.is_empty() {
        format!("https://tiles.windy.com/labels/{}", path)
    } else {
        format!("https://tiles.windy.com/labels/{}?{}", path, query)
    };
    proxy_request(&state, &url, true, &cache_key).await
}

async fn handle_upstream_node(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
    req: Request<Body>,
) -> impl IntoResponse {
    let mut query = req.uri().query().unwrap_or("").to_string();
    if path.contains("minifest") && !query.contains("premium=true") {
        if query.is_empty() {
            query = "premium=true".to_string();
        } else {
            query = format!("{}&premium=true", query);
        }
    }
    let url = if query.is_empty() {
        format!("https://node.windy.com/{}", path)
    } else {
        format!("https://node.windy.com/{}?{}", path, query)
    };
    proxy_request(&state, &url, false, "").await
}

async fn handle_upstream_nodes(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
    req: Request<Body>,
) -> impl IntoResponse {
    let query = req.uri().query().unwrap_or("");
    let url = if query.is_empty() {
        format!("https://node-s.windy.com/{}", path)
    } else {
        format!("https://node-s.windy.com/{}?{}", path, query)
    };
    proxy_request(&state, &url, false, "").await
}

async fn handle_upstream_account(
    State(_state): State<Arc<AppState>>,
    Path(_path): Path<String>,
    _req: Request<Body>,
) -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .body(Body::from(r#"{"status":"ok","tier":"premium","auth":true}"#))
        .unwrap()
}

async fn handle_upstream_www(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
    req: Request<Body>,
) -> impl IntoResponse {
    let query = req.uri().query().unwrap_or("");
    let url = if query.is_empty() {
        format!("https://www.windy.com/{}", path)
    } else {
        format!("https://www.windy.com/{}?{}", path, query)
    };
    proxy_request(&state, &url, false, "").await
}

async fn handle_patch() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/javascript")
        .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .body(Body::from("// Local Patch Script OK\n"))
        .unwrap()
}

async fn handle_node_services(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
    req: Request<Body>,
) -> Response<Body> {
    handle_node_proxy(state, format!("/services/{}", path), req).await
}

async fn handle_node_metadata(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
    req: Request<Body>,
) -> Response<Body> {
    let mut query = req.uri().query().unwrap_or("").to_string();
    if path.contains("minifest") && !query.contains("premium=true") {
        if query.is_empty() {
            query = "premium=true".to_string();
        } else {
            query = format!("{}&premium=true", query);
        }
    }
    let url = if query.is_empty() {
        format!("https://node.windy.com/metadata/{}", path)
    } else {
        format!("https://node.windy.com/metadata/{}?{}", path, query)
    };
    proxy_request(&state, &url, false, "").await
}

async fn handle_node_maptile(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
    req: Request<Body>,
) -> Response<Body> {
    handle_node_proxy(state, format!("/maptile/{}", path), req).await
}

async fn handle_node_forecast(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
    req: Request<Body>,
) -> Response<Body> {
    handle_node_proxy(state, format!("/forecast/{}", path), req).await
}

async fn handle_node_search(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
    req: Request<Body>,
) -> Response<Body> {
    handle_node_proxy(state, format!("/search/{}", path), req).await
}

async fn handle_node_webcams(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
    req: Request<Body>,
) -> Response<Body> {
    handle_node_proxy(state, format!("/webcams/{}", path), req).await
}

async fn handle_node_citytile(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
    req: Request<Body>,
) -> Response<Body> {
    let cache_key = req.uri().to_string();
    let query = req.uri().query().unwrap_or("");
    let url = if query.is_empty() {
        format!("https://node.windy.com/citytile/{}", path)
    } else {
        format!("https://node.windy.com/citytile/{}?{}", path, query)
    };
    proxy_request(&state, &url, true, &cache_key).await
}

async fn handle_node_capalerts(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
    req: Request<Body>,
) -> Response<Body> {
    handle_node_proxy(state, format!("/capalerts/{}", path), req).await
}

async fn handle_node_tides(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
    req: Request<Body>,
) -> Response<Body> {
    let resp = handle_node_proxy(state, format!("/tides/{}", path), req).await;
    if resp.status().is_client_error() || resp.status().is_server_error() {
        return Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
            .body(Body::from(r#"{"status":"ok","data":[]}"#))
            .unwrap();
    }
    resp
}

async fn handle_node_pois(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
    req: Request<Body>,
) -> Response<Body> {
    let resp = handle_node_proxy(state, format!("/pois/{}", path), req).await;
    if resp.status().is_client_error() || resp.status().is_server_error() {
        return Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
            .body(Body::from(r#"{"status":"ok","pois":[],"data":[]}"#))
            .unwrap();
    }
    resp
}

async fn handle_node_obs(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
    req: Request<Body>,
) -> Response<Body> {
    handle_node_proxy(state, format!("/obs/{}", path), req).await
}

async fn handle_node_tc(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
    req: Request<Body>,
) -> Response<Body> {
    handle_node_proxy(state, format!("/tc/{}", path), req).await
}

async fn handle_node_users(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
    req: Request<Body>,
) -> Response<Body> {
    if path.starts_with("v1/data") {
        return Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
            .body(Body::from(r#"{"status":"ok","data":[],"value":[]}"#))
            .unwrap();
    }
    handle_node_proxy(state, format!("/users/{}", path), req).await
}

async fn handle_plugins_list() -> Response<Body> {
    let path = PathBuf::from(PUBLIC_DIR).join("plugins_list.json");
    if let Ok(bytes) = tokio::fs::read(&path).await {
        return Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
            .header(header::CACHE_CONTROL, "public, max-age=3600")
            .body(Body::from(bytes))
            .unwrap();
    }
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .body(Body::from("[]"))
        .unwrap()
}

async fn handle_users_data(
    Path(_path): Path<String>,
) -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .body(Body::from(r#"{"status":"ok","value":{}}"#))
        .unwrap()
}

async fn handle_api(
    Path(path): Path<String>,
) -> Response<Body> {
    if path == "info" || path == "v1/info" {
        return Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(r#"{"auth":true,"userInfo":{"auth":true,"username":"Bhartiya Marut Kendra User","tier":"premium","joindate":1700000000,"logindate":1700000000},"subscriptionInfo":{"status":"active","tier":"premium","state":"active","platform":"web","expires":4102444800000}}"#))
            .unwrap();
    }
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(r#"{"status":"ok"}"#))
        .unwrap()
}

async fn handle_node_proxy(
    state: Arc<AppState>,
    path: String,
    req: Request<Body>,
) -> Response<Body> {
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
    // 1. Tier-1: Check In-Memory Moka Cache (RAM)
    if is_tile && !cache_key.is_empty() {
        if let Some((status, content_type, bytes)) = state.tile_cache.get(cache_key).await {
            return make_tile_response(status, &content_type, bytes, "HIT-RAM");
        }

        // 2. Tier-2: Check Persistent Disk Cache (Offline Tile Database)
        if let Some((status, content_type, bytes)) = read_disk_cache(&state.disk_cache_dir, cache_key).await {
            // Warm Tier-1 RAM cache
            state
                .tile_cache
                .insert(cache_key.to_string(), (status, content_type.clone(), bytes.clone()))
                .await;
            return make_tile_response(status, &content_type, bytes, "HIT-DISK");
        }
    }

    // 3. Upstream Fetch with pooled connection
    match state
        .client
        .get(url)
        .header(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36",
        )
        .header("Referer", "https://www.windy.com/")
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
                    // Cache successful tile responses in RAM & Disk
                    if is_tile && status == StatusCode::OK && !cache_key.is_empty() {
                        state
                            .tile_cache
                            .insert(cache_key.to_string(), (status, content_type.clone(), bytes.clone()))
                            .await;

                        let disk_dir = state.disk_cache_dir.clone();
                        let key = cache_key.to_string();
                        let ct = content_type.clone();
                        let b = bytes.clone();
                        tokio::spawn(async move {
                            write_disk_cache(&disk_dir, &key, &ct, &b).await;
                        });
                    }

                    make_tile_response(status, &content_type, bytes, "MISS-UPSTREAM")
                }
                Err(err) => {
                    error!("Error reading bytes from {}: {}", url, err);
                    if let Some((status, content_type, bytes)) = read_disk_cache(&state.disk_cache_dir, cache_key).await {
                        return make_tile_response(status, &content_type, bytes, "HIT-DISK-OFFLINE");
                    }
                    Response::builder()
                        .status(StatusCode::BAD_GATEWAY)
                        .body(Body::from(format!("Error reading upstream: {}", err)))
                        .unwrap()
                }
            }
        }
        Err(err) => {
            error!("Error fetching from {}: {}", url, err);
            if let Some((status, content_type, bytes)) = read_disk_cache(&state.disk_cache_dir, cache_key).await {
                return make_tile_response(status, &content_type, bytes, "HIT-DISK-OFFLINE");
            }
            Response::builder()
                .status(StatusCode::BAD_GATEWAY)
                .body(Body::from(format!("Error fetching upstream: {}", err)))
                .unwrap()
        }
    }
}
