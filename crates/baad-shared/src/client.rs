use std::sync::OnceLock;

use reqwest::Client;

static GLOBAL_CLIENT: OnceLock<Client> = OnceLock::new();

pub fn set_client(client: Client) { let _ = GLOBAL_CLIENT.set(client); }

pub fn client() -> &'static Client { GLOBAL_CLIENT.get_or_init(Client::new) }
