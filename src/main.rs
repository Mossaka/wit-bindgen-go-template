use std::{convert::Infallible, net::SocketAddr, path::PathBuf};

use anyhow::{bail, Context, Result};
use handler::Response;
use host::add_to_linker;

use wasi_cap_std_sync::WasiCtxBuilder;
use wasmtime::{
    component::{Component, Linker},
    Config, Engine, Store, WasmBacktraceDetails,
};

use hyper::{body::HttpBody as _, header::HeaderName, http::HeaderValue, StatusCode};
use hyper::{Body as HyperBody, Request as HyperRequest, Response as HyperResponse, Server};
use routerify::ext::RequestExt;
use routerify::{Router, RouterService};
use routerify_cors::enable_cors_all;

use crate::handler::{Method, Request};

wasmtime::component::bindgen!({
    world: "world",
    path: "./wit",
    async: true,}
);

impl From<&hyper::Method> for Method {
    fn from(method: &hyper::Method) -> Self {
        match *method {
            hyper::Method::GET => Method::Get,
            hyper::Method::POST => Method::Post,
            hyper::Method::PUT => Method::Put,
            hyper::Method::DELETE => Method::Delete,
            hyper::Method::PATCH => Method::Patch,
            hyper::Method::HEAD => Method::Head,
            hyper::Method::OPTIONS => Method::Options,
            _ => panic!("failed due to unsupported method, currently supported methods are: GET, POST, PUT, DELETE, PATCH, HEAD, and OPTIONS"),
        }
    }
}

pub struct HttpHeader<'a>(Vec<(&'a str, &'a str)>);

impl<'a> HttpHeader<'a> {
    pub fn inner(self) -> Vec<(&'a str, &'a str)> {
        self.0
    }
}

impl<'a> From<&'a hyper::HeaderMap> for HttpHeader<'a> {
    fn from(headers: &'a hyper::HeaderMap) -> Self {
        Self(
            headers
                .iter()
                .map(|(name, value)| (name.as_str(), value.to_str().unwrap()))
                .collect(),
        )
    }
}

pub struct HttpBody(Vec<u8>);

impl HttpBody {
    pub async fn from_body(body: hyper::Body) -> Result<Self> {
        const MAX_ALLOWED_SIZE: u64 = u64::MAX;
        let content_length = match body.size_hint().upper() {
            Some(v) => v,
            None => bail!("failed to read HTTP body size"),
        };

        if content_length < MAX_ALLOWED_SIZE {
            let body_bytes = hyper::body::to_bytes(body).await?;
            let owned_body = Self(body_bytes.to_vec());
            return Ok(owned_body);
        }

        bail!(
            "failed due to HTTP body being too large (size: {}, allowed size: {})",
            content_length,
            MAX_ALLOWED_SIZE
        );
    }

    pub fn inner(self) -> Vec<u8> {
        self.0
    }
}

impl From<Response> for hyper::Response<HyperBody> {
    fn from(res: Response) -> Self {
        let mut response = if let Some(body) = res.body {
            hyper::Response::new(HyperBody::from(body))
        } else {
            hyper::Response::new(HyperBody::empty())
        };
        *response.status_mut() = StatusCode::from_u16(res.status).unwrap();
        if let Some(headers) = res.headers {
            let headers = hyper::HeaderMap::from_iter(headers.iter().map(|(key, value)| {
                (
                    HeaderName::from_bytes(key.as_bytes()).unwrap(),
                    HeaderValue::from_str(value).unwrap(),
                )
            }));
            *response.headers_mut() = headers;
        }
        response
    }
}

async fn hello(hyper_req: HyperRequest<HyperBody>) -> Result<HyperResponse<HyperBody>, Infallible> {
    let (parts, body) = hyper_req.into_parts();
    let (component, engine, linker) = parts.data().unwrap();

    // add wasi to linker
    let mut store = Store::new(
        engine,
        WasiCtxBuilder::new()
            .inherit_stdin()
            .inherit_stdout()
            .build(),
    );

    // get component
    let (s, _) = Serverless::instantiate_async(&mut store, component, linker)
        .await
        .unwrap();
    let handler = s.handler();

    // create a request
    let params = parts.params();
    let params: Vec<(&str, &str)> = params
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();
    let method: Method = (&parts.method).into();
    let headers: HttpHeader = (&parts.headers).into();

    let bytes = HttpBody::from_body(body).await.unwrap().inner();
    let uri = &parts.uri.to_string();

    let req = Request {
        method,
        uri,
        headers: &headers.inner(),
        body: Some(&bytes),
        params: &params,
    };

    let res = handler.call_handle_http(&mut store, req).await.unwrap();
    match res {
        Ok(res) => Ok(res.into()),
        Err(err) => {
            panic!("Host: error: {err}");
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let input = PathBuf::from(
        std::env::args()
            .collect::<Vec<String>>()
            .get(1)
            .context("must provide an input file")?,
    );

    let mut config = Config::new();
    config.cache_config_load_default().unwrap();
    config.wasm_backtrace_details(WasmBacktraceDetails::Enable);
    config.wasm_component_model(true);
    config.async_support(true);
    let engine = Engine::new(&config).unwrap();
    let mut linker = Linker::new(&engine);
    add_to_linker(&mut linker, |x| x).unwrap();

    let component = Component::from_file(&engine, &input).unwrap();

    // We'll bind to 127.0.0.1:3001
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));

    let mut router = Router::builder()
        .middleware(enable_cors_all())
        .data((component, engine, linker));
    router = router.get("/", hello);
    let built = router.build().unwrap();
    let service = RouterService::new(built).unwrap();
    let server = Server::bind(&addr).serve(service);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {e}");
    }

    Ok(())
}
