<div align="center">
    <h1>night_worker</h1>
    Ergonimic Cloudflare Workers SDK for <em>nightly</em> Rust
</div>

<br>

- For [fetch](https://developers.cloudflare.com/workers/runtime-apis/handlers/fetch/) use mainly
- Built upon [workers-rs](https://crates.io/crates/worker)

<br>

## Supported Bindings

- [x] KV
- [x] Service Bindings
- [x] D1 ( by `"d1"` feature )
- [x] Queues ( by `"queue"` feature )
- [ ] Durable Objects ( TODO )

<br>

## Example

*Cargo.toml*
```toml
# the same version of 
# `worker` crate is required

[dependencies]
worker       = "0.1.0"
night_worker = "0.1.0-rc"
```

*src/lib.rs*
```rust
use worker::{Request, Env, Context, Response};
use night_worker::{Worker, Error};

#[worker::event(fetch)]
async fn main(
    req: Request,
    env: Env,
    ctx: Context,
) -> Result<Response, worker::Error> {
    let w = Worker::take_over(env, ctx);
    ergonimic_fetch(w, req).await.map_err(Into::into)
}

async fn ergonimic_fetch(
    w:   Worker<'_>,
    req: Request,
) -> Result<Response, night_worker::Error> {
    let kv = w.KV("MY_KV")?;

    kv.put("key1", "value1").await?;
    kv.put("key2", "value2").expiration_ttl(1024).await?;

    let value = kv.get("key1").await?;
    let value = kv.get("key2").cache_ttl(1024).await?;

    let all = kv.list().await?;
    let all = kv.list().prefix("worker").limit(42).await?;
}
```
