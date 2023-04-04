#[cfg(feature = "ssr")]
mod serv;

use leptos::*;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    serv::serv().await;
}
