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

//! Call an ONDEWO S2T server with an auto-refreshed Keycloak token.
//!
//! With `ONDEWO_S2T_OFFLINE_TOKEN` set, the client starts up from that offline refresh token and
//! never sends a password; otherwise it logs in with `ONDEWO_S2T_PASSWORD`.
//!
//! ```sh
//! ONDEWO_S2T_HOST=s2t.example.com ONDEWO_S2T_PORT=443 \
//! ONDEWO_KEYCLOAK_URL=https://keycloak.example.com/auth \
//! ONDEWO_KEYCLOAK_REALM=ondewo-ccai-platform \
//! ONDEWO_KEYCLOAK_CLIENT_ID=ondewo-s2t-cai-sdk-public \
//! ONDEWO_S2T_USERNAME=technical-user@example.com \
//! ONDEWO_S2T_OFFLINE_TOKEN=<offline refresh token> \
//! ONDEWO_S2T_AUDIO_FILE=./utterance.wav \
//! ONDEWO_S2T_PIPELINE_ID=de_DE_ondewo \
//!   cargo run --example keycloak_client
//! ```

use std::env;
use std::error::Error;
use std::fs;

use ondewo_s2t_client::api::ondewo::s2t;
use ondewo_s2t_client::api::ondewo::s2t::speech2_text_client::Speech2TextClient;
use ondewo_s2t_client::channel::ClientConfig;
use ondewo_s2t_client::keycloak::{KeycloakConfig, KeycloakInterceptor};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = KeycloakConfig::new(
        env::var("ONDEWO_KEYCLOAK_URL")?,
        env::var("ONDEWO_KEYCLOAK_REALM")?,
        env::var("ONDEWO_KEYCLOAK_CLIENT_ID")?,
        env::var("ONDEWO_S2T_USERNAME").unwrap_or_default(),
        env::var("ONDEWO_S2T_PASSWORD").unwrap_or_default(),
    )
    .with_refresh_token(env::var("ONDEWO_S2T_OFFLINE_TOKEN").unwrap_or_default());
    // Validates the config, then acquires the first token; the provider keeps it fresh.
    let interceptor = KeycloakInterceptor::new(&config)?;

    let channel = ClientConfig::new(
        env::var("ONDEWO_S2T_HOST")?,
        env::var("ONDEWO_S2T_PORT")?.parse()?,
    )
    .connect()
    .await?;
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
