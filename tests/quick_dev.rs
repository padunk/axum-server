#![allow(unused)]

use anyhow::{Ok, Result};
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;
    // hc.do_get("/hello?name=rustland").await?.print().await?;
    // hc.do_get("/hello2/abraham").await?.print().await?;

    // hc.do_get("/src/main.rs").await?.print().await?;

    // hc.do_post(
    //     "/api/login",
    //     json!({
    //         "username": "user",
    //         "password": "123456",
    //     }),
    // )
    // .await?
    // .print()
    // .await?;

    // hc.do_post(
    //     "/api/tickets",
    //     json!({
    //         "title": "my first ticket - 1",
    //     }),
    // )
    // .await?
    // .print()
    // .await?;

    // hc.do_post(
    //     "/api/tickets",
    //     json!({
    //         "title": "my second ticket - 2",
    //     }),
    // )
    // .await?
    // .print()
    // .await?;

    hc.do_get("/api/tickets").await?.print().await?;

    // hc.do_delete("/api/tickets/2").await?.print().await?;

    // hc.do_get("/api/tickets").await?.print().await?;

    Ok(())
}
