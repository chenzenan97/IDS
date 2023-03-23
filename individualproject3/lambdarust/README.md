# Rust-lambda random BBCNEWS generator

```bash
cargo init

cargo build --release --target=x86_64-unknown-linux-gnu

cp target/x86_64-unknown-linux-gnu/release/lambdarust ./bootstrap

serverless deploy

## References

* [Rust-lambda random fact generator](https://github.com/mianwu515/rust-world-spr23/tree/main/lambda-rand-fact-prj3)

* [how-to-install-node-js](https://kinsta.com/blog/how-to-install-node-js/)
* [aws-credentials](https://www.serverless.com/framework/docs/providers/aws/guide/credentials)
* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)