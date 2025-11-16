use serde_json::json;

#[tokio::test]
pub async fn quick_dev() -> color_eyre::Result<()> {
    let hc = httpc_test::new_client("http://127.0.0.1:8000")?;

    hc.do_get("/health").await?.print().await?;
    hc.do_post("/health/query", json!({ "name": "Gerami" })).await?.print().await?;
    hc.do_get("/health/Brian").await?.print().await?;

    Ok(())
}