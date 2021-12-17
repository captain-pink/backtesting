pub mod fake_api {
    use crate::lib::common::ExchangeAPI;
    use crate::lib::fake::FakeDb;
    use log::info;

    pub struct FakeApi {
        db: FakeDb,
    }

    impl ExchangeAPI for FakeApi {
        fn new() -> Self {
            let db = FakeDb::new();
            let fake_api = FakeApi { db };
            info!("FakeApi successfully created");

            fake_api
        }
    }
}
