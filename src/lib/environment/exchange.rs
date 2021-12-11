pub mod exchange {
    use crate::lib::common::{ExchangeConfig, Exchanges, Timeframe, ExchangeType};

    #[derive(Debug, Clone)]
    pub struct Exchange {
        options: ExchangeConfig,
        timeframe: Timeframe,
        exchange_type: ExchangeType,
    }

    impl Exchange {
        pub fn new(exchange: Exchanges, timeframe: Timeframe, exchange_type: ExchangeType) -> Exchange {
            Exchange { options: exchange.load_config(), timeframe, exchange_type }
        }

        pub fn options(&self) -> &ExchangeConfig {
            &self.options
        }

        pub fn timeframe(&self) -> &Timeframe {
            &self.timeframe
        }

        // pub fn klines() {

        // }
    }
}
