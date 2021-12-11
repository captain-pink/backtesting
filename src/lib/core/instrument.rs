pub mod instrument {
    use std::fmt::{Display, Formatter, Result};

    #[allow(dead_code)]
    pub enum Currencies {
        USD,
        USDT,
        BTC,
        ETH,
        LINK,
    }

    impl Display for Currencies {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            match self {
                Self::USD => write!(f, "USD"),
                Self::USDT => write!(f, "USDT"),
                Self::BTC => write!(f, "BTC"),
                Self::ETH => write!(f, "ETH"),
                Self::LINK => write!(f, "LINK"),
            }
        }
    }

    pub struct Instrument {
        pub currency: Currencies,
        pub precision: u8,
    }

    impl Display for Instrument {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(
                f,
                "Instrument {{ currency: {}, precisio: {} }}",
                self.currency, self.precision
            )
        }
    }
}
