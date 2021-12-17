use async_trait::async_trait;

#[async_trait]
pub trait ExchangeAPI {
    fn new() -> Self;
    // async fn server_info() -> ();
    // async fn klines() -> ();
    // async fn last_kline() -> ();
    // async fn create_order() -> ();
    // async fn cancell_order() -> ();
}

#[async_trait]
pub trait ExchangeWS {
    async fn klines_listener() -> ();
    async fn account_listener() -> ();
}
