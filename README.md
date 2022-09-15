<p align="center">
  <img src="https://raw.githubusercontent.com/royrustdev/rustymail/main/public/rustymail.svg" alt="rustymail logo" width=200px height=200px>
</p>
<h1 align="center">rustymail</h1>

## Config

### Method to change default port:

```rust
// Axum server is running on 127.0.0.1:8080
// Port 8080 is the default port.
// Pass desired port number as an argument in run_app funtion

#[tokio::main]
async fn main() -> Result<(), hyper::Error> {
    run_app(8080)?.await
}
```

## Available Routes

`/health_check` - check server status. It returns a `OK` status without any body.