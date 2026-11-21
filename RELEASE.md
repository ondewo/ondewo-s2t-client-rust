# Release History

*****************

## Release ONDEWO S2T Rust Client 7.6.2

### New Features

* [[OND236-120]](https://ondewo.atlassian.net/browse/OND236-120) Keycloak authentication with offline-token start-up, matching ondewo-s2t-client-python 7.6.2 (this release carries the python SDK's 7.6.1 and 7.6.2 Keycloak work at once, so every language shares one version for one feature set): `KeycloakConfig`, `KeycloakTokenProvider` and `KeycloakInterceptor`, which attaches `authorization: Bearer <token>` to every call; the default transport is one blocking HTTP/1.1 POST over std sockets and rustls (built without its default features).
* [[OND236-120]](https://ondewo.atlassian.net/browse/OND236-120) A handed-off Keycloak *offline* refresh token starts the client with one `grant_type=refresh_token` exchange, and the client then never sends a password grant, not even as a fallback; a refused token is never answered with a password login. A Keycloak config needs `password` OR `refresh_token`, all Keycloak fields are validated all-or-nothing, and the password and the token are redacted in every rendering.
* [[OND236-120]](https://ondewo.atlassian.net/browse/OND236-120) Resilient refresh: a failed refresh never ends the refresh loop; the proactive refresh and the per-call (lazy) refresh share one full-jitter backoff (5 s doubling to 300 s, measured from the recorded failure) and send no request while it runs; concurrent callers share one in-flight refresh. A still-valid token keeps being served while a refresh fails, and only an expired token raises `KeycloakAuthenticationError` (with its source).
* [[OND236-120]](https://ondewo.atlassian.net/browse/OND236-120) One shared provider per credential set, keyed by a SHA-256 of the credentials, so two configs of different tenants never share one. The token request has a 10 s timeout, quoted Keycloak error bodies are cut at 500 characters, and a token response is reported by its field names only, never its values.

### Improvements

* [[OND236-120]](https://ondewo.atlassian.net/browse/OND236-120) Tests: the T01-T62 catalogue plus HTTP and TLS transport cases; the hand-written code stays at 100 % line coverage.

API unchanged: tracking API Version [7.6.0](https://github.com/ondewo/ondewo-s2t-api/releases/tag/7.6.0) ( [Documentation](https://ondewo.github.io/ondewo-s2t-api/) )

*****************

## Release ONDEWO S2T Rust Client 7.6.0

### Improvements

* Tracking API Version [7.6.0](https://github.com/ondewo/ondewo-s2t-api/releases/tag/7.6.0) ( [Documentation](https://ondewo.github.io/ondewo-s2t-api/) )
* Generated with ondewo-proto-compiler 5.17.0 (protoc 35.1). New runtime floors: `prost` / `prost-types` 0.14.6 and `tonic` / `tonic-prost` 0.14.9.

*****************

## Release ONDEWO S2T Rust Client 7.5.1

### New Features

* TLS, mutual TLS and plaintext channels: the new hand-written module `channel` builds the tonic
  `Endpoint` / `Channel` from a `ClientConfig` (`endpoint()`, `connect()`, `connect_lazy()`).
  `grpc_cert`, `grpc_client_cert` and `grpc_client_key` take PEM **content**, not file paths.
  Without `grpc_cert` the platform trust store is used (tonic `tls-native-roots`).
* Refused with a `ChannelError` before tonic sees the config: half a client identity
  (`IncompleteClientIdentity`), a client identity on a plaintext channel
  (`ClientIdentityWithoutTls`) and a certificate field that holds no PEM certificate
  (`NotAPemCertificate`).
* A plaintext channel logs a `tracing` warning naming `host:port`. `Debug` redacts the client key
  and renders no PEM, and error messages name only the field and `host:port`. A bare IPv6 host is
  bracketed, and `tls_domain_name` overrides the name the server certificate is checked against.
* README section "TLS, mutual TLS and certificates": the modes, a test PKI built with openssl, TLS
  security notes and the channel defaults. Of the python SDK channel options, the message size
  limit is applied per client (`MAX_MESSAGE_LENGTH`); keepalive pings are not set (TCP keepalive
  detects a dropped idle connection instead), and tonic has no reconnect backoff or retry policy.

### Tests

* `tests/tls_channel.rs` runs real handshakes against an in-process tonic server with an
  openssl-generated test PKI: plain TLS, mutual TLS, missing and foreign client identities, wrong
  CA, platform roots, CRLF PEMs, SAN override and IPv6 loopback.
* `tests/release_notes.rs` pins the RELEASE.md slice the GitHub release body is cut from.

### Build

* `ondewo-proto-compiler` is pinned to 5.15.4 (5.15.2 before this release), which pre-warms tonic
  `tls-native-roots` for the offline stub generation that `src/channel.rs` needs.
  `make generate_ondewo_protos` now also stages `README.md`, which the image's `cargo package`
  requires. The regenerated stubs under `src/api` are byte-identical.
* Tracking API Version [7.5.0](https://github.com/ondewo/ondewo-s2t-api/releases/tag/7.5.0) (unchanged)

*****************

## Release ONDEWO S2T Rust Client 7.5.0

### New Features

* Initial release of the ONDEWO S2T (Speech-to-Text) gRPC client for rust. The crate
  ships the complete client surface of the [ondewo-s2t-api](https://github.com/ondewo/ondewo-s2t-api)
  protocol buffer definitions: [prost](https://crates.io/crates/prost) message types and
  [tonic](https://crates.io/crates/tonic) service clients for every RPC, generated by the
  `ondewo-rust-proto-compiler` image of
  [ondewo-proto-compiler](https://github.com/ondewo/ondewo-proto-compiler) 5.15.1.
* The generated module tree is confined to `src/api` and is re-exported from the crate root, so a
  message declared in `package ondewo.s2t;` is reachable as
  `ondewo_s2t_client::ondewo::s2t::<Message>` and each service as
  `ondewo::s2t::<service>_client::<Service>Client`. Hand-written modules live beside it
  under `src/` and are declared in the hand-written crate barrel `src/lib.rs`, which the
  generator leaves untouched.
* `make build` runs the whole pipeline - submodule checkout, compiler image build, stub
  generation and `cargo build` - and `make check_build` asserts that a generated stub exists for
  every proto package before a release is cut.
* The generated stubs are COMMITTED under `src/api`, so the crate compiles straight from a
  checkout - no docker, no compiler image and no submodule are needed to build or test it.
  Regenerating them stays a `make build` concern.
* A hand-written `auth` module (`ondewo_s2t_client::auth::BearerTokenInterceptor`) attaches the
  Keycloak `authorization: Bearer <token>` and `cai-token` metadata to every outgoing request.
  Its `Debug` output redacts both credentials. `examples/authenticated_client.rs` shows the flow.
* An integration suite under `tests/` exercises the generated code: wire-level encode/parse
  round trips over the prost messages, pinned enum discriminants, and the generated
  `Speech2TextServer` served over a loopback socket and driven by the generated `Speech2TextClient`, so
  every declared RPC is really routed by its `/ondewo.s2t.Speech2Text/<Method>` path. `make coverage`
  gates the hand-written sources at 100% line coverage, as does CI.

*****************
