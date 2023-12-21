use anyhow::Result;
use serde::{Deserialize, Serialize};
use spin_sdk::http::{IntoResponse, Request, Response};

#[derive(Debug, Deserialize, Serialize)]
pub struct Route {
    pub source: String,
    pub destination: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Router {
    #[serde(rename = "route")]
    pub routes: Vec<Route>,
}

impl Router {
    pub fn default() -> Result<Self> {
        let routes = vec![
            Route {
                source: "/".to_string(),
                destination: "https://www.google.com".to_string(),
            },
            Route {
                source: "/rust".to_string(),
                destination: "https://www.rust-lang.org".to_string(),
            },
        ];

        Ok(Self { routes })
    }

    pub fn redirect(self, req: Request) -> anyhow::Result<impl IntoResponse> {
        let path_info = req
            .header("spin-path-info")
            .and_then(|h| h.as_str())
            .expect("cannot get path info from request headers");
        let route = match self.path(path_info) {
            Some(r) => r,
            None => return Ok(Response::new(404, "")),
        };
        let res = Response::builder()
            .status(http::StatusCode::PERMANENT_REDIRECT)
            .header(http::header::LOCATION.as_str(), route.destination)
            .body("")
            .build();
        Ok(res)
    }

    fn path(self, path: &str) -> Option<Route> {
        self.routes.into_iter().find(|r| r.source == path)
    }
}
