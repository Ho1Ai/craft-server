use axum::{Router, routing::{get, post}, Json, extract::State};
use axum::extract::Path;
use axum::handler::Handler;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::{Error, FromRow, Pool};
use sqlx::postgres::Postgres;

#[derive(Serialize)]
struct PkgInfoJson {
    pkg_list: Vec<String>
}

#[derive(Clone)]
struct AppState {
    pg_pool: PgPool
}

#[derive(FromRow, Debug, Eq, PartialEq, Clone, Ord, PartialOrd)]
struct Item {
    id: i64,
    pkg_name: String,
    pkg_build_type: i16,
    pkg_platform: Vec<String>,
    pkg_path: Vec<String>,
}

impl AppState {
    async fn get_item(&self, pkg_name: String) -> Result<Item, Error> {
        let request = r#"SELECT * FROM pkg_list WHERE pkg_name = $1"#;
            let result: Item = sqlx::query_as(request).bind(pkg_name).fetch_one(&self.pg_pool).await?;
        Ok(result)
    }
}

#[tokio::main]
async fn main() {
    const PG_URL: &str= "postgresql://postgres:postgres@localhost:5432/craft";
    let app_state = AppState{pg_pool: PgPoolOptions::new().connect(PG_URL).await.unwrap()};

    let app = Router::new()
        .route("/", get(base_handler))
        .route("/get-pkg-info/{name}", get(get_pkg_info))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn base_handler() -> &'static str {
    "Base output. The server has been started on 0.0.0.0:8080"
}

async fn get_pkg_info(State(state): State<AppState>, Path(name): Path<String>) -> (StatusCode, Json<PkgInfoJson>) {

    let mut final_res = std::collections::BTreeSet::new();

    let first_res = state.get_item(name).await.unwrap();
    final_res.insert(first_res);

    while !final_res.is_empty() {
        break;
    }

    (StatusCode::OK, Json(PkgInfoJson{pkg_list: vec!["".to_string()]}))
}
