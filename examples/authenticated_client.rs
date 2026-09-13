// Copyright 2021-2026 ONDEWO GmbH
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Transcribe an audio file on an ONDEWO S2T server with a Keycloak bearer token.
//!
//! This is the crate's usage snippet. It lives here rather than in a doc comment because doctests
//! are disabled crate-wide (see the `doctest = false` note in `Cargo.toml`); as an example it is
//! still compiled by `cargo test` and `cargo build --examples`, so it cannot rot.
//!
//! ```sh
//! ONDEWO_S2T_HOST=https://s2t.example.com:443 \
//! ONDEWO_S2T_ACCESS_TOKEN=<keycloak access token> \
//! ONDEWO_S2T_CAI_TOKEN=<cai token> \
//! ONDEWO_S2T_PIPELINE_ID=de_DE_ondewo \
//! ONDEWO_S2T_AUDIO_FILE=./utterance.wav \
//!   cargo run --example authenticated_client
//! ```

use std::env;
use std::error::Error;
use std::fs;

use ondewo_s2t_client::api::ondewo::s2t;
use ondewo_s2t_client::api::ondewo::s2t::speech2_text_client::Speech2TextClient;
use ondewo_s2t_client::auth::BearerTokenInterceptor;
use tonic::transport::Endpoint;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let host = env::var("ONDEWO_S2T_HOST")?;
    let access_token = env::var("ONDEWO_S2T_ACCESS_TOKEN")?;

    let mut interceptor = BearerTokenInterceptor::new(&access_token)?;
    if let Ok(cai_token) = env::var("ONDEWO_S2T_CAI_TOKEN") {
        interceptor = interceptor.with_cai_token(&cai_token)?;
    }

    let channel = Endpoint::from_shared(host)?.connect().await?;
    let mut client = Speech2TextClient::with_interceptor(channel, interceptor);

    let response = client
        .transcribe_file(s2t::TranscribeFileRequest {
            audio_file: fs::read(env::var("ONDEWO_S2T_AUDIO_FILE")?)?,
            config: Some(s2t::TranscribeRequestConfig {
                s2t_pipeline_id: env::var("ONDEWO_S2T_PIPELINE_ID")?,
                // `language` is a proto3 `optional`: leaving it `None` means "not requested",
                // which is a different message on the wire from requesting an empty language.
                language: env::var("ONDEWO_S2T_LANGUAGE").ok(),
                ..Default::default()
            }),
        })
        .await?
        .into_inner();

    for transcription in response.transcriptions {
        println!(
            "{} (confidence {:.2})",
            transcription.transcription, transcription.confidence_score
        );
    }
    Ok(())
}
