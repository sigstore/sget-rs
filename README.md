# sget

[![CI](https://github.com/sigstore/sget/actions/workflows/main.yml/badge.svg)](https://github.com/sigstore/sget/actions/workflows/main.yml)

> :warning: Not ready for use yet! :warning:  

sget is a safe artifact retrieval and execution tool.

It's purpose is to provide a means to address common insecure download methods, such as using curl operations piped to bash, followed by shell script execution.

The initial work involves the use of an OCI registry, however other storage methods are planned and we are open to suggestions from the community.

> sget is based off the prototype [sget](https://github.com/sigstore/cosign/blob/main/cmd/sget/) repurposed in Rust.

## Security

Should you discover any security issues, please refer to sigstore's [security
process](https://github.com/sigstore/community/blob/main/SECURITY.md).
