#![feature(no_sanitize)]
#![feature(no_coverage)]

#[no_coverage]
#[no_sanitize(address, memory, thread)]
pub mod f0_http_generator;

#[no_coverage]
#[no_sanitize(address, memory, thread)]
pub mod f0_httpresponse_generator;

#[no_coverage]
#[no_sanitize(address, memory, thread)]
pub mod f0_url_generator;

#[no_sanitize(address, memory, thread)]
pub mod http2;


#[no_coverage]
#[no_sanitize(address, memory, thread)]
pub fn setup_tracing() -> Option<()> {
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();
    Some(())
}
