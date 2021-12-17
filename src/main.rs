
mod system;
use system::{bootstrap_system};

mod lib;
use lib::common::{ExchangeType, Exchanges, Timeframe};
use lib::environment::create as create_environment;
use lib::environment::{Exchange, Observer, Renderer, Strategy};
use lib::fake::FakeApi;

fn main() {
    bootstrap_system();

    // FakeApi::new();
    // let renderer = Renderer {};
    // let strategy = Strategy {};
    // let exchange = Exchange::new(Exchanges::Binance, Timeframe::Minutes(15), ExchangeType::Fake);
    // let observer = Observer::new(None);

    // let environment = create_environment(observer, renderer, Vec::from([exchange]), strategy);
    // environment.run();
}
