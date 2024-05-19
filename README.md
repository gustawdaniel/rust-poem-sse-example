# Rust poem sse example

This is experimental repo created during learning poem and SSE.

How to run:

1. Start server:

```bash
cargo run --bin rust-poem
```

2. Open openapi

```
http http://localhost:3000
```

3. Open client in browser

```
PORT=5000 npx serve .
```

then inspect element and see events

4. Open client in terminal

```
cargo run --bin client
```

Sources:

console sse client
https://github.com/launchdarkly/rust-eventsource-client/blob/main/eventsource-client/examples/tail.rs

cors
https://github.com/poem-web/poem/discussions/175

openapi sse
https://github.com/poem-web/poem/blob/master/examples/openapi/sse/src/main.rs

poem openapi
https://github.com/poem-web/poem/blob/master/poem-openapi/README.md

list of poem features
https://github.com/poem-web/poem/blob/master/poem/README.md

poem readme
https://github.com/poem-web/poem?tab=readme-ov-file

other rust openapi tools
https://www.reddit.com/r/rust/comments/xannze/web_frameworks_with_integrated_open_api/

