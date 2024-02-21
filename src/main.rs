use std::sync::{Arc, Mutex};

use crate::routes::create_routes;

mod anyhow_errors;
mod routes;

fn main() {
    println!("Hello, world!");
    let manifest = AppManifest {
        name: "saleor app".to_owned(),
    };

    let app_state = AppState {
        manifest,
        saleor_app: Arc::new(Mutex::new(SaleorApp {
            apl: RedisApl {
                client: "Client".to_owned(),
            },
        })),
    };

    let router = create_routes(app_state);
}

#[derive(Clone, Debug)]
pub struct AppManifest {
    name: String,
}

#[derive(Clone, Debug)]
pub struct AppState<A: APL> {
    pub saleor_app: Arc<Mutex<SaleorApp<A>>>,
    pub manifest: AppManifest,
}

#[derive(Clone, Debug)]
pub struct SaleorApp<A: APL> {
    apl: A,
}

pub trait APL: Sized + Send + Sync + Clone + std::fmt::Debug {
    async fn get(&self) -> u32;
}

#[derive(Debug, Clone)]
pub struct RedisApl {
    client: String,
}

impl APL for RedisApl {
    async fn get(&self) -> u32 {
        32
    }
}
