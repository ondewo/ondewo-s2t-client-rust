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

//! End-to-end tests for the GENERATED tonic service stubs.
//!
//! The generated `Speech2TextServer` is served over a loopback socket and driven by the generated
//! `Speech2TextClient`, so a request really is encoded, routed by its
//! `/ondewo.s2t.Speech2Text/<Method>` path, decoded, answered and decoded again. That is what
//! catches a service the generator wired to the wrong path, a codec mismatch, or a method that
//! silently went missing.
//!
//! `Speech2Text` carries one streaming RPC, `TranscribeStream` (client stream in, server stream
//! out), so the fake server also has to declare the generated `TranscribeStreamStream` associated
//! type; the round-trip, presence, error and metadata assertions stay on unary RPCs.
//!
//! No network beyond `127.0.0.1` and no ONDEWO server is involved.

use std::collections::HashMap;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use ondewo_s2t_client::api::ondewo::s2t;
use ondewo_s2t_client::api::ondewo::s2t::speech2_text_client::Speech2TextClient;
use ondewo_s2t_client::api::ondewo::s2t::speech2_text_server::{Speech2Text, Speech2TextServer};
use ondewo_s2t_client::auth::{
    BearerTokenInterceptor, AUTHORIZATION_METADATA_KEY, CAI_TOKEN_METADATA_KEY,
};
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;
use tokio_stream::Stream;
use tonic::transport::{Channel, Endpoint, Server};
use tonic::{Code, Request, Response, Status};

/// The pipeline id `get_s2t_pipeline` answers with `not_found` for, so the error path is exercised.
const MISSING_PIPELINE: &str = "does-not-exist";

/// Metadata the fake server captured from the last request it handled.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct SeenMetadata {
    authorization: Option<String>,
    cai_token: Option<String>,
}

/// A minimal in-process implementation of the generated `Speech2Text` service.
///
/// The pipeline RPCs are backed by a real map, so `CreateS2tPipeline` followed by
/// `GetS2tPipeline` is a genuine client -> server -> client round trip of a whole config.
#[derive(Clone, Default)]
struct FakeSpeech2Text {
    seen: Arc<Mutex<SeenMetadata>>,
    pipelines: Arc<Mutex<HashMap<String, s2t::Speech2TextConfig>>>,
}

impl FakeSpeech2Text {
    fn record<T>(&self, request: &Request<T>) {
        let read = |key: &str| {
            request
                .metadata()
                .get(key)
                .map(|value| value.to_str().unwrap().to_string())
        };
        *self.seen.lock().unwrap() = SeenMetadata {
            authorization: read(AUTHORIZATION_METADATA_KEY),
            cai_token: read(CAI_TOKEN_METADATA_KEY),
        };
    }

    fn seen(&self) -> SeenMetadata {
        self.seen.lock().unwrap().clone()
    }
}

/// Transcribe `audio` the way the fake server does: one word per byte value, so the response
/// carries repeated nested messages that the assertions can pin down.
fn transcribe(audio: &[u8]) -> s2t::Transcription {
    s2t::Transcription {
        transcription: format!("{} bytes", audio.len()),
        confidence_score: 0.93,
        words: audio
            .iter()
            .enumerate()
            .map(|(index, byte)| s2t::WordDetail {
                start_time: index as f32,
                end_time: index as f32 + 1.0,
                word: format!("byte-{byte}"),
                confidence: 0.9,
                word_alternatives: Vec::new(),
            })
            .collect(),
        alternatives: Vec::new(),
    }
}

/// A pipeline config whose deeply nested `turn_detection.active` presence field is under test.
fn pipeline(id: &str, turn_detection_active: Option<bool>) -> s2t::Speech2TextConfig {
    s2t::Speech2TextConfig {
        id: id.to_string(),
        description: Some(s2t::S2tDescription {
            language: "de".to_string(),
            pipeline_owner: "ondewo".to_string(),
            domain: "general".to_string(),
            comments: String::new(),
        }),
        active: true,
        streaming_server: Some(s2t::StreamingServer {
            host: "127.0.0.1".to_string(),
            port: 40_015,
            output_style: "word".to_string(),
            streaming_speech_recognition: Some(s2t::StreamingSpeechRecognition {
                sampling_rate: 16_000,
                turn_detection: Some(s2t::TurnDetectionOptions {
                    active: turn_detection_active,
                    ..Default::default()
                }),
                ..Default::default()
            }),
        }),
        ..Default::default()
    }
}

/// Reach the presence field `pipeline` sets, through the three messages it is nested in.
fn turn_detection_active(config: &s2t::Speech2TextConfig) -> Option<bool> {
    config
        .streaming_server
        .as_ref()?
        .streaming_speech_recognition
        .as_ref()?
        .turn_detection
        .as_ref()?
        .active
}

#[tonic::async_trait]
impl Speech2Text for FakeSpeech2Text {
    async fn transcribe_file(
        &self,
        request: Request<s2t::TranscribeFileRequest>,
    ) -> Result<Response<s2t::TranscribeFileResponse>, Status> {
        self.record(&request);
        let request = request.into_inner();
        Ok(Response::new(s2t::TranscribeFileResponse {
            transcriptions: vec![transcribe(&request.audio_file)],
            time: 0.42,
            audio_uuid: request
                .config
                .and_then(|config| config.language)
                .unwrap_or_else(|| "audio-uuid-1".to_string()),
        }))
    }

    type TranscribeStreamStream =
        Pin<Box<dyn Stream<Item = Result<s2t::TranscribeStreamResponse, Status>> + Send>>;

    /// Drains the client stream and answers with one response per audio chunk, so both halves of
    /// the bidirectional RPC are really carried over the socket.
    async fn transcribe_stream(
        &self,
        request: Request<tonic::Streaming<s2t::TranscribeStreamRequest>>,
    ) -> Result<Response<Self::TranscribeStreamStream>, Status> {
        self.record(&request);
        let mut incoming = request.into_inner();
        let mut responses = Vec::new();
        while let Some(chunk) = incoming.message().await? {
            responses.push(Ok(s2t::TranscribeStreamResponse {
                transcriptions: vec![transcribe(&chunk.audio_chunk)],
                time: 0.1,
                r#final: chunk.end_of_stream,
                audio_uuid: "audio-uuid-1".to_string(),
                oneof_config: chunk
                    .config
                    .map(s2t::transcribe_stream_response::OneofConfig::Config),
                ..Default::default()
            }));
        }
        Ok(Response::new(Box::pin(tokio_stream::iter(responses))))
    }

    async fn get_s2t_pipeline(
        &self,
        request: Request<s2t::S2tPipelineId>,
    ) -> Result<Response<s2t::Speech2TextConfig>, Status> {
        self.record(&request);
        let id = request.into_inner().id;
        self.pipelines
            .lock()
            .unwrap()
            .get(&id)
            .cloned()
            .map(Response::new)
            .ok_or_else(|| Status::not_found(format!("no s2t pipeline named {id}")))
    }

    async fn create_s2t_pipeline(
        &self,
        request: Request<s2t::Speech2TextConfig>,
    ) -> Result<Response<s2t::S2tPipelineId>, Status> {
        self.record(&request);
        let config = request.into_inner();
        let id = config.id.clone();
        self.pipelines.lock().unwrap().insert(id.clone(), config);
        Ok(Response::new(s2t::S2tPipelineId { id }))
    }

    async fn delete_s2t_pipeline(
        &self,
        request: Request<s2t::S2tPipelineId>,
    ) -> Result<Response<()>, Status> {
        self.record(&request);
        self.pipelines
            .lock()
            .unwrap()
            .remove(&request.into_inner().id);
        Ok(Response::new(()))
    }

    async fn update_s2t_pipeline(
        &self,
        request: Request<s2t::Speech2TextConfig>,
    ) -> Result<Response<()>, Status> {
        self.record(&request);
        let config = request.into_inner();
        self.pipelines
            .lock()
            .unwrap()
            .insert(config.id.clone(), config);
        Ok(Response::new(()))
    }

    async fn list_s2t_pipelines(
        &self,
        request: Request<s2t::ListS2tPipelinesRequest>,
    ) -> Result<Response<s2t::ListS2tPipelinesResponse>, Status> {
        self.record(&request);
        let mut pipeline_configs: Vec<_> =
            self.pipelines.lock().unwrap().values().cloned().collect();
        pipeline_configs.sort_by(|left, right| left.id.cmp(&right.id));
        Ok(Response::new(s2t::ListS2tPipelinesResponse {
            pipeline_configs,
        }))
    }

    async fn list_s2t_languages(
        &self,
        request: Request<s2t::ListS2tLanguagesRequest>,
    ) -> Result<Response<s2t::ListS2tLanguagesResponse>, Status> {
        self.record(&request);
        Ok(Response::new(s2t::ListS2tLanguagesResponse {
            languages: vec!["de".to_string(), "en".to_string()],
        }))
    }

    async fn list_s2t_domains(
        &self,
        request: Request<s2t::ListS2tDomainsRequest>,
    ) -> Result<Response<s2t::ListS2tDomainsResponse>, Status> {
        self.record(&request);
        Ok(Response::new(s2t::ListS2tDomainsResponse {
            domains: vec!["general".to_string()],
        }))
    }

    async fn get_service_info(
        &self,
        request: Request<()>,
    ) -> Result<Response<s2t::S2tGetServiceInfoResponse>, Status> {
        self.record(&request);
        Ok(Response::new(s2t::S2tGetServiceInfoResponse {
            version: "7.5.0".to_string(),
        }))
    }

    async fn list_s2t_language_models(
        &self,
        request: Request<s2t::ListS2tLanguageModelsRequest>,
    ) -> Result<Response<s2t::ListS2tLanguageModelsResponse>, Status> {
        self.record(&request);
        Ok(Response::new(s2t::ListS2tLanguageModelsResponse {
            lm_pipeline_ids: request
                .into_inner()
                .ids
                .into_iter()
                .map(|pipeline_id| s2t::LanguageModelPipelineId {
                    pipeline_id,
                    model_names: vec!["default".to_string()],
                })
                .collect(),
        }))
    }

    async fn create_user_language_model(
        &self,
        request: Request<s2t::CreateUserLanguageModelRequest>,
    ) -> Result<Response<()>, Status> {
        self.record(&request);
        Ok(Response::new(()))
    }

    async fn delete_user_language_model(
        &self,
        request: Request<s2t::DeleteUserLanguageModelRequest>,
    ) -> Result<Response<()>, Status> {
        self.record(&request);
        Ok(Response::new(()))
    }

    async fn add_data_to_user_language_model(
        &self,
        request: Request<s2t::AddDataToUserLanguageModelRequest>,
    ) -> Result<Response<()>, Status> {
        self.record(&request);
        Ok(Response::new(()))
    }

    async fn train_user_language_model(
        &self,
        request: Request<s2t::TrainUserLanguageModelRequest>,
    ) -> Result<Response<()>, Status> {
        self.record(&request);
        Ok(Response::new(()))
    }

    async fn list_s2t_normalization_pipelines(
        &self,
        request: Request<s2t::ListS2tNormalizationPipelinesRequest>,
    ) -> Result<Response<s2t::ListS2tNormalizationPipelinesResponse>, Status> {
        self.record(&request);
        Ok(Response::new(s2t::ListS2tNormalizationPipelinesResponse {
            s2t_normalization_pipelines: vec![request.into_inner().language],
        }))
    }
}

/// Start the generated server on an ephemeral loopback port and return it with its address.
///
/// The server task is detached; it ends when the test process does.
async fn start_server() -> (FakeSpeech2Text, SocketAddr) {
    let service = FakeSpeech2Text::default();
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
    let addr = listener.local_addr().expect("local_addr");

    let served = service.clone();
    tokio::spawn(async move {
        Server::builder()
            .add_service(Speech2TextServer::new(served))
            .serve_with_incoming(TcpListenerStream::new(listener))
            .await
            .expect("the in-process gRPC server must not fail");
    });

    (service, addr)
}

async fn connect(addr: SocketAddr) -> Channel {
    Endpoint::from_shared(format!("http://{addr}"))
        .expect("endpoint")
        .connect_timeout(Duration::from_secs(10))
        .connect()
        .await
        .expect("the in-process gRPC server must accept a connection")
}

#[tokio::test]
async fn a_unary_call_round_trips_through_the_generated_client_and_server() {
    let (_service, addr) = start_server().await;
    let mut client = Speech2TextClient::new(connect(addr).await);

    let response = client
        .transcribe_file(s2t::TranscribeFileRequest {
            audio_file: vec![0x01, 0x02, 0x03],
            config: Some(s2t::TranscribeRequestConfig {
                s2t_pipeline_id: "de_DE_ondewo".to_string(),
                decoding: s2t::Decoding::BeamSearch as i32,
                ..Default::default()
            }),
        })
        .await
        .expect("TranscribeFile must succeed")
        .into_inner();

    assert_eq!(response.transcriptions.len(), 1);
    assert_eq!(response.transcriptions[0].transcription, "3 bytes");
    assert_eq!(response.transcriptions[0].words.len(), 3);
    assert_eq!(response.transcriptions[0].words[2].word, "byte-3");
    assert_eq!(response.time, 0.42);
    assert_eq!(response.audio_uuid, "audio-uuid-1");
}

/// The explicit-presence guarantee of `tests/generated_messages.rs`, but over a real hop: a config
/// whose `turn_detection.active` is explicitly `false` is stored by `CreateS2tPipeline` and read
/// back by `GetS2tPipeline` as `Some(false)`, never as `None`.
#[tokio::test]
async fn an_explicit_presence_field_survives_a_real_grpc_hop() {
    let (_service, addr) = start_server().await;
    let mut client = Speech2TextClient::new(connect(addr).await);

    for (id, active) in [("explicitly-off", Some(false)), ("unset", None)] {
        let created = client
            .create_s2t_pipeline(pipeline(id, active))
            .await
            .expect("CreateS2tPipeline must succeed")
            .into_inner();
        assert_eq!(created.id, id);

        let read_back = client
            .get_s2t_pipeline(created)
            .await
            .expect("GetS2tPipeline must succeed")
            .into_inner();

        assert_eq!(
            turn_detection_active(&read_back),
            active,
            "the presence of turn_detection.active must survive the hop unchanged"
        );
        assert_eq!(read_back, pipeline(id, active));
    }
}

/// The one streaming RPC of the service: the client streams audio chunks in and reads a response
/// stream back, so both directions of `TranscribeStream` are really carried over the socket.
#[tokio::test]
async fn the_streaming_rpc_carries_every_chunk_in_both_directions() {
    let (_service, addr) = start_server().await;
    let mut client = Speech2TextClient::new(connect(addr).await);

    let requests = vec![
        s2t::TranscribeStreamRequest {
            audio_chunk: vec![0x01, 0x02],
            end_of_stream: false,
            config: Some(s2t::TranscribeRequestConfig {
                s2t_pipeline_id: "de_DE_ondewo".to_string(),
                ..Default::default()
            }),
            mute_audio: false,
        },
        s2t::TranscribeStreamRequest {
            audio_chunk: vec![0x03],
            end_of_stream: true,
            config: None,
            mute_audio: false,
        },
    ];

    let mut responses = client
        .transcribe_stream(tokio_stream::iter(requests))
        .await
        .expect("TranscribeStream must succeed")
        .into_inner();

    let mut transcriptions = Vec::new();
    let mut finals = Vec::new();
    while let Some(chunk) = responses
        .message()
        .await
        .expect("the response stream must not fail")
    {
        transcriptions.push(chunk.transcriptions[0].transcription.clone());
        finals.push(chunk.r#final);
    }

    assert_eq!(
        transcriptions,
        vec!["2 bytes".to_string(), "1 bytes".to_string()]
    );
    assert_eq!(
        finals,
        vec![false, true],
        "the end-of-stream flag must reach the server and come back on the right chunk"
    );
}

/// A server-side `Status` has to reach the caller as that same status, not as a transport error.
#[tokio::test]
async fn a_server_error_reaches_the_client_as_its_status() {
    let (_service, addr) = start_server().await;
    let mut client = Speech2TextClient::new(connect(addr).await);

    let error = client
        .get_s2t_pipeline(s2t::S2tPipelineId {
            id: MISSING_PIPELINE.to_string(),
        })
        .await
        .expect_err("GetS2tPipeline must report the missing pipeline");

    assert_eq!(error.code(), Code::NotFound);
    assert_eq!(error.message(), "no s2t pipeline named does-not-exist");
}

/// Every RPC the `Speech2Text` proto declares must exist on the generated client and be routable -
/// a method the generator dropped, or wired to the wrong path, fails here with `Unimplemented`.
///
/// `GetServiceInfo` takes `google.protobuf.Empty`, which prost models as `()`.
#[tokio::test]
async fn every_declared_service_method_exists_and_is_routable() {
    let (_service, addr) = start_server().await;
    let mut client = Speech2TextClient::new(connect(addr).await);

    client
        .transcribe_file(s2t::TranscribeFileRequest::default())
        .await
        .expect("TranscribeFile");
    client
        .transcribe_stream(tokio_stream::iter(vec![
            s2t::TranscribeStreamRequest::default(),
        ]))
        .await
        .expect("TranscribeStream");
    client
        .create_s2t_pipeline(pipeline("routable", None))
        .await
        .expect("CreateS2tPipeline");
    client
        .get_s2t_pipeline(s2t::S2tPipelineId {
            id: "routable".to_string(),
        })
        .await
        .expect("GetS2tPipeline");
    client
        .update_s2t_pipeline(pipeline("routable", Some(true)))
        .await
        .expect("UpdateS2tPipeline");
    client
        .list_s2t_pipelines(s2t::ListS2tPipelinesRequest::default())
        .await
        .expect("ListS2tPipelines");
    client
        .delete_s2t_pipeline(s2t::S2tPipelineId {
            id: "routable".to_string(),
        })
        .await
        .expect("DeleteS2tPipeline");
    client
        .list_s2t_languages(s2t::ListS2tLanguagesRequest::default())
        .await
        .expect("ListS2tLanguages");
    client
        .list_s2t_domains(s2t::ListS2tDomainsRequest::default())
        .await
        .expect("ListS2tDomains");
    client.get_service_info(()).await.expect("GetServiceInfo");
    client
        .list_s2t_language_models(s2t::ListS2tLanguageModelsRequest::default())
        .await
        .expect("ListS2tLanguageModels");
    client
        .create_user_language_model(s2t::CreateUserLanguageModelRequest::default())
        .await
        .expect("CreateUserLanguageModel");
    client
        .add_data_to_user_language_model(s2t::AddDataToUserLanguageModelRequest::default())
        .await
        .expect("AddDataToUserLanguageModel");
    client
        .train_user_language_model(s2t::TrainUserLanguageModelRequest::default())
        .await
        .expect("TrainUserLanguageModel");
    client
        .delete_user_language_model(s2t::DeleteUserLanguageModelRequest::default())
        .await
        .expect("DeleteUserLanguageModel");
    client
        .list_s2t_normalization_pipelines(s2t::ListS2tNormalizationPipelinesRequest::default())
        .await
        .expect("ListS2tNormalizationPipelines");
}

/// The pipeline RPCs share one store, so the sequence has to behave: what `Create` wrote, `List`
/// reports and `Delete` removes.
#[tokio::test]
async fn the_pipeline_rpcs_operate_on_the_same_state() {
    let (_service, addr) = start_server().await;
    let mut client = Speech2TextClient::new(connect(addr).await);

    for id in ["de_DE_ondewo", "en_US_ondewo"] {
        client
            .create_s2t_pipeline(pipeline(id, Some(true)))
            .await
            .expect("CreateS2tPipeline");
    }

    let listed = client
        .list_s2t_pipelines(s2t::ListS2tPipelinesRequest::default())
        .await
        .expect("ListS2tPipelines")
        .into_inner();
    assert_eq!(
        listed
            .pipeline_configs
            .iter()
            .map(|config| config.id.as_str())
            .collect::<Vec<_>>(),
        vec!["de_DE_ondewo", "en_US_ondewo"]
    );

    client
        .delete_s2t_pipeline(s2t::S2tPipelineId {
            id: "de_DE_ondewo".to_string(),
        })
        .await
        .expect("DeleteS2tPipeline");

    let error = client
        .get_s2t_pipeline(s2t::S2tPipelineId {
            id: "de_DE_ondewo".to_string(),
        })
        .await
        .expect_err("the deleted pipeline must be gone");
    assert_eq!(error.code(), Code::NotFound);
}

/// The hand-written [`BearerTokenInterceptor`] has to put its metadata on the wire, where the
/// server can actually read it - asserting on the `Request` it returns would not prove that.
#[tokio::test]
async fn the_bearer_interceptor_reaches_the_server() {
    let (service, addr) = start_server().await;
    let interceptor = BearerTokenInterceptor::new("access-token-abc")
        .expect("a plain ASCII token is valid")
        .with_cai_token("cai-token-xyz")
        .expect("a plain ASCII cai token is valid");
    let mut client = Speech2TextClient::with_interceptor(connect(addr).await, interceptor);

    client.get_service_info(()).await.expect("GetServiceInfo");

    assert_eq!(
        service.seen(),
        SeenMetadata {
            authorization: Some("Bearer access-token-abc".to_string()),
            cai_token: Some("cai-token-xyz".to_string()),
        }
    );
}

/// Without the interceptor the client must send no credentials at all - the unauthenticated path
/// (plaintext server, or an ingress that injects the bearer token) has to stay usable.
#[tokio::test]
async fn a_client_without_an_interceptor_sends_no_credentials() {
    let (service, addr) = start_server().await;
    let mut client = Speech2TextClient::new(connect(addr).await);

    client.get_service_info(()).await.expect("GetServiceInfo");

    assert_eq!(service.seen(), SeenMetadata::default());
}

/// A client built against an address nothing listens on must surface a transport error rather
/// than panic or hang - `connect_lazy` defers the connect to the first call.
#[tokio::test]
async fn a_call_to_an_unreachable_target_fails_as_a_status() {
    let channel = Endpoint::from_static("http://127.0.0.1:1")
        .connect_timeout(Duration::from_secs(2))
        .connect_lazy();
    let mut client = Speech2TextClient::new(channel);

    let error = client
        .get_service_info(())
        .await
        .expect_err("nothing listens on port 1");

    assert_eq!(error.code(), Code::Unavailable);
}
