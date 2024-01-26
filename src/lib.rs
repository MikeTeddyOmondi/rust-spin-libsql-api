// Use the libsql_client module
use libsql_client::{Config, SyncClient};
use spin_sdk::http::{IntoResponse, Request};
use spin_sdk::{http_component, variables};

/// A simple Spin HTTP component.
#[http_component]
fn handle_spin_rust_api(req: Request) -> anyhow::Result<impl IntoResponse> {
    // Get component configuration for the Turso database
    // let turso_url = variables::get("turso_url")?;
    let turso_url = variables::get("turso_url").expect("Error getting turso url variable!");
    // let turso_auth_token = variables::get("turso_auth_token")?;
    let turso_auth_token = variables::get("turso_auth_token").expect("Error getting turso auth token variable!");

    // Create a SyncClient object for querying Turso
    let libsql_config = Config::new(turso_url.as_str())?.with_auth_token(turso_auth_token);
    let libsql_client = SyncClient::from_config(libsql_config)?;

    // // Make a query that returns a ResultSet with one row with one column containing a string
    // let rs = libsql_client.execute("select 'Hello, Turso'")?;
    // let message = rs.rows[0].values[0].to_string();

    println!("Handling request to {:?}", req.header("spin-full-url"));
    Ok(http::Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body("Hello, Fermyon")?)
}
