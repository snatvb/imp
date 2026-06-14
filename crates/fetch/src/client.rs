use std::sync::OnceLock;

use reqwest::Client;

pub fn get_client() -> &'static Client {
    static CLIENT: OnceLock<Client> = OnceLock::new();
    CLIENT.get_or_init(|| {
        Client::builder()
            .user_agent("ImpJS/0.1")
            .build()
            .expect("failed to create HTTP client")
    })
}
