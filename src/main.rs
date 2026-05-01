use std::string::ToString;
use axum::{Router, routing::{get, post}, Json, http::StatusCode};
use axum::extract::{State, Path};
use axum::response::Json as ResponseJson;
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, Pool};
use sqlx::postgres::{PgPool, PgPoolOptions};

#[derive(Serialize, FromRow, Debug)]
struct PackageInstance {
    id: i64,
    pkg_name: String,
    pkg_build_type: i16,
    pkg_platform: Vec<String>,
    pkg_path: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
struct CreatePackage{
    pkg_name: String,
    pkg_build_type: i16,
    pkg_platform: Vec<String>,
}

const DEFAULT_UNRECOGNIZED_PACKAGE: PackageInstance = PackageInstance{
    id: -1,
    pkg_name: String::new(),
    pkg_build_type: -1,
    pkg_platform: vec![],
    pkg_path: vec![]
};

#[derive(FromRow, Debug, Serialize)]
struct CreatePackageReturn {
    id: i64,
}

#[derive(Clone)]
struct AppState {
    pg_pool: PgPool,
}

impl AppState {
    async fn get_package (&self, pkg_name: String) -> Result<PackageInstance, Error>{
        let postgres_request = r#"select * from pkg_list where pkg_name = $1"#;

        let response: PackageInstance = sqlx::query_as(postgres_request).bind(pkg_name).fetch_one(&self.pg_pool).await?;
        Ok(response)
    }

    async fn create_package_in_db (&self, create_package: CreatePackage) -> Result<CreatePackageReturn, Error>{
        let postgres_request = r#"insert into pkg_list (pkg_name, pkg_build_type, pkg_platform, pkg_path) values ($1, $2, $3, $4) returning id"#;

        let response: CreatePackageReturn = sqlx::query_as(postgres_request).bind(create_package.pkg_name.clone()).bind(1).bind(create_package.pkg_platform.clone()).bind(vec![create_package.pkg_name.clone()]).fetch_one(&self.pg_pool).await?;
        Ok(response)
    }

    async fn get_packages(&self) -> Result<Vec<PackageInstance>, Error>{
        let postgres_request = r#"select * from pkg_list"#;

        let response = sqlx::query_as(postgres_request).fetch_all(&self.pg_pool).await?;
        Ok(response)
    }
}

#[tokio::main]
async fn main() {
    const PG_URL: &str = "postgres://postgres:password@localhost:5432/craft";

    let application = Router::new()
        .route("/", get(root))
        .route("/get-pkg-info/{name}", get(get_package_info))
        .route("/get-pkg-list", get(get_all_packages))
        .route("/create-package", post(create_package))
        .with_state(AppState{pg_pool: PgPoolOptions::new().connect("postgres://postgres:password@localhost:5432/craft").await.unwrap()});

    let listener = tokio::net::TcpListener::bind("localhost:8080").await.unwrap();
    axum::serve(listener, application).await.unwrap();
}

async fn root() -> &'static str {
    "The server has been started on 0.0.0.0:8080"
}

async fn get_package_info (State(state): State<AppState>, Path(name): Path<String>) -> (StatusCode, ResponseJson<PackageInstance>) {
    let pkg_info_from_db = state.get_package(name).await;
    if (pkg_info_from_db.is_ok()) {
        let pkg_info = pkg_info_from_db.unwrap();
        println!("{:#?}", pkg_info);
        return (StatusCode::OK, Json(pkg_info));
    }
    else {
        return (StatusCode::NOT_FOUND, Json(DEFAULT_UNRECOGNIZED_PACKAGE));
    }
}

async fn create_package (State(state): State<AppState>, Json(payload): Json<CreatePackage>) -> (StatusCode, ResponseJson<CreatePackageReturn>) {
    let pkg_info = state.create_package_in_db(payload).await;

    if(pkg_info.is_ok()) {
        return (StatusCode::CREATED, Json(pkg_info.unwrap()));
    } else {
        return (StatusCode::BAD_REQUEST, Json(CreatePackageReturn { id: -1 }));
    }
}

async fn get_all_packages (State(state): State<AppState>) -> (StatusCode, ResponseJson<Vec<PackageInstance>>) {
    let pkg_list = state.get_packages().await;
    if(pkg_list.is_ok()) {
        (StatusCode::OK, Json(pkg_list.unwrap()))
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, Json(vec![]))
    }
}