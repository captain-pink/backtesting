use dotenv::dotenv;

pub fn read_env_vars() {
    dotenv().ok();
}
