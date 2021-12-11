
#[derive(Debug, Clone)]
pub struct Hosts {
    futures: String,
    spot: String,
}

#[derive(Debug, Clone)]
pub struct AccessHeaders {
    key: String,
    secret: String,
}

#[derive(Debug, Clone)]
pub struct ExchangeConfig {
    name: Exchanges,
    api_version: String,
    access_headers: AccessHeaders,
    hosts: Hosts,
}

#[derive(Debug, Clone)]
pub enum ExchangeType {
    Futures,
    Spot,
    Fake,
}

#[derive(Debug, Clone, Copy)]
pub enum Exchanges {
    Binance,
}

impl Exchanges {
    pub fn load_config(&self) -> ExchangeConfig {
        match *self {
            Self::Binance => {
                let hosts = Hosts {
                    futures: String::from("futres.example.com"),
                    spot: String::from("spot.example.com"),
                };
                let access_headers = AccessHeaders {
                    key: String::from("binance_key"),
                    secret: String::from("binance_secret"),
                };
                ExchangeConfig {
                    name: Exchanges::Binance,
                    api_version: String::from("v1"),
                    hosts,
                    access_headers,
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Timeframe {
    Minutes(u8),
    Hours(u8),
    Days(u8),
}
