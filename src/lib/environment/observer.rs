pub mod observer {
    use polars::prelude::DataFrame;

    type Memory = Option<DataFrame>;

    #[derive(Debug, Clone)]
    pub struct Observer {
        memory: Memory,
    }

    impl Observer {
        pub fn new(memory: Memory) -> Observer {
            Observer { memory }
        }

        pub fn memory(&self) -> &Memory {
            &self.memory
        }
    }
}
