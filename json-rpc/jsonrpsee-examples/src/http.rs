use std::net::SocketAddr;
use std::time::Duration;

use hyper::body::Bytes;
use jsonrpsee::core::client::ClientT;
use jsonrpsee::http_client::HttpClientBuilder;
use jsonrpsee::rpc_params;
use jsonrpsee::server::{RpcModule, Server};
// use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tower_http::LatencyUnit;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let filter = tracing_subscriber::EnvFilter::try_from_default_env()?
		.add_directive("jsonrpsee[method_call{name = \"say_hello\"}]=trace".parse()?);
	tracing_subscriber::FmtSubscriber::builder().with_env_filter(filter).finish().try_init()?;

	let server_addr = run_server().await?;
	let url = format!("http://{}", server_addr);

	let middleware = tower::ServiceBuilder::new();
	// .layer(
	// 	TraceLayer::new_for_http()
	// 		.on_request(
	// 			|request: &hyper::Request<hyper::body::Body>, _span: &tracing::Span| tracing::info!(request = ?request, "on_request"),
	// 		)
	// 		.on_body_chunk(|chunk: &Bytes, latency: Duration, _: &tracing::Span| {
	// 			tracing::info!(size_bytes = chunk.len(), latency = ?latency, "sending body chunk")
	// 		})
	// 		.make_span_with(DefaultMakeSpan::new().include_headers(true))
	// 		.on_response(DefaultOnResponse::new().include_headers(true).latency_unit(LatencyUnit::Micros)),
	// );

	let client = HttpClientBuilder::default().set_http_middleware(middleware).build(url)?;
	let params = rpc_params![1_u64, 2, 3];
	let response: Result<String, _> = client.request("say_hello", params).await;
	tracing::info!("r: {:?}", response);

	Ok(())
}

async fn run_server() -> anyhow::Result<SocketAddr> {
	let server = Server::builder().build("127.0.0.1:0".parse::<SocketAddr>()?).await?;
	let mut module = RpcModule::new(());
	module.register_method("say_hello", |_, _| "lo")?;

	let addr = server.local_addr()?;
	let handle = server.start(module);

	// In this example we don't care about doing shutdown so let's it run forever.
	// You may use the `ServerHandle` to shut it down or manage it yourself.
	tokio::spawn(handle.stopped());

	Ok(addr)
}