pub mod environment {
    use crate::lib::environment::{Exchange, Observer, Renderer, Strategy};

    #[derive(Debug, Clone)]
    pub struct Environment {
        pub observer: Observer,
        pub renderer: Renderer,
        pub exchanges: Vec<Exchange>,
        pub strategy: Strategy,
    }

    impl Environment {
        pub fn preload(&self) {
            // let preload = &self.exchanges.klines(period);
            for exchange in self.exchanges.iter() {
                println!("this is exchange {:?}", exchange);
            }
        }

        pub fn run(&self) -> () {
            self.preload();
        }

    }

    pub fn create(
        observer: Observer,
        renderer: Renderer,
        exchanges: Vec<Exchange>,
        strategy: Strategy,
    ) -> Environment {
        Environment {
            observer,
            renderer,
            exchanges,
            strategy,
        }
    }
}
