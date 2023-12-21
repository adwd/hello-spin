use spin_sdk::{
    http::{IntoResponse, Method, Request, Response},
    http_component,
    key_value::Store,
};

#[http_component]
fn handle_request(req: Request) -> anyhow::Result<impl IntoResponse> {
    // Open the default key-value store
    let store = Store::open_default()?;

    let (status, body) = match *req.method() {
        Method::Post => {
            // Add the request (URI, body) tuple to the store
            store.set(req.path(), req.body())?;
            println!(
                "Storing value in the KV store with {:?} as the key",
                req.path()
            );
            (200, None)
        }
        Method::Get => {
            // Get the value associated with the request URI, or return a 404 if it's not present
            match store.get(req.path())? {
                Some(value) => {
                    println!("Found value for the key {:?}", req.path());
                    (200, Some(value))
                }
                None => {
                    println!("No value found for the key {:?}", req.path());
                    (404, None)
                }
            }
        }
        Method::Delete => {
            // Delete the value associated with the request URI, if present
            store.delete(req.path())?;
            println!("Delete key {:?}", req.path());
            (200, None)
        }
        Method::Head => {
            // Like GET, except do not return the value
            let code = if store.exists(req.path())? {
                println!("{:?} key found", req.path());
                200
            } else {
                println!("{:?} key not found", req.path());
                404
            };
            (code, None)
        }
        // No other methods are currently supported
        _ => (405, None),
    };
    Ok(Response::new(status, body))
}
