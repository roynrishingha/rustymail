use rustymail::run_app;

/// Port 8080 is the default port.
/// Pass desired port number as an argument in run_app funtion
#[tokio::main]
async fn main() -> Result<(), hyper::Error> {
    run_app(8080)?.await
}
