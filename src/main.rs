mod lib;
use lib::common::{Exchanges, Timeframe, ExchangeType};
use lib::core::{Currencies, Instrument};
use lib::environment::{create as create_environment};
use lib::environment::{Exchange, Observer, Renderer, Strategy};

fn main() {
    let renderer = Renderer {};
    let strategy = Strategy {};
    let exchange = Exchange::new(Exchanges::Binance, Timeframe::Minutes(15), ExchangeType::Fake);
    let observer = Observer::new(None);

    let environment = create_environment(observer, renderer, Vec::from([exchange]), strategy);
    environment.run();
}
