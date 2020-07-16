# rust-async-std-server-issue-example

Example of async std tcp server problem(http2 server test).

# Run example 

- First start the server by running: `cargo test`.
- In another terminal create a curl request like so: `curl http://localhost:8080 --http2-prior-knowledge --verbose`

## Expected behaviour

- I read tcp stream bytes which represent http2 preflight.
- I read tcp stream bytes which represent http2 settings frame.
- I write tcp stream bytes which represent http2 settings frame.
- I read tcp stream bytes which represent http2 headers frame.
.
.
.

## Actual behaviour

- I read tcp stream bytes which represent http2 preflight.
- I read tcp stream bytes which represent http2 settings frame.
- I write tcp stream bytes which represent http2 settings frame.
- I read tcp stream bytes which represent http2 settings frame that I just wrote.
- I write tcp stream bytes which represent http2 settings frame.
- I read tcp stream bytes which represent http2 settings frame that I just wrote.
- I write tcp stream bytes which represent http2 settings frame.
- I read tcp stream bytes which represent http2 settings frame that I just wrote.
- I write tcp stream bytes which represent http2 settings frame.
- I read tcp stream bytes which represent http2 settings frame that I just wrote.

Basically loop myself.

How can I read and write in the same tcp stream?