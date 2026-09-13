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

//! Wire-level tests for the GENERATED prost messages under `src/api`.
//!
//! These are the cases that catch a broken generator: a dropped field, a shifted tag number, a
//! presence field silently coerced to its zero value, an enum whose discriminants moved, a map or
//! a oneof that lost its shape. They are pure encode/decode - no runtime, no socket. The gRPC
//! plumbing is covered by `tests/generated_grpc.rs`.
//!
//! The NLU client's cross-package case has no counterpart here: the S2T API compiles to exactly
//! one package, `ondewo.s2t` - it vendors no `google.*` proto and no second ONDEWO package.

use std::collections::HashMap;

use ondewo_s2t_client::api::ondewo::s2t;
use prost::Message;

/// A fully populated [`s2t::Transcription`] - scalars plus two levels of repeated nesting.
fn sample_transcription() -> s2t::Transcription {
    s2t::Transcription {
        transcription: "guten tag".to_string(),
        confidence_score: 0.93,
        words: vec![
            s2t::WordDetail {
                start_time: 0.0,
                end_time: 0.4,
                word: "guten".to_string(),
                confidence: 0.95,
                word_alternatives: vec![s2t::WordAlternative {
                    word: "guetn".to_string(),
                    confidence: 0.05,
                }],
            },
            s2t::WordDetail {
                start_time: 0.4,
                end_time: 0.8,
                word: "tag".to_string(),
                confidence: 0.91,
                word_alternatives: Vec::new(),
            },
        ],
        alternatives: vec![s2t::TranscriptionAlternative {
            transcript: "guten tach".to_string(),
            confidence: 0.31,
            words: Vec::new(),
        }],
    }
}

#[test]
fn a_transcription_survives_a_serialize_parse_round_trip() {
    let original = sample_transcription();

    let bytes = original.encode_to_vec();
    assert!(
        !bytes.is_empty(),
        "a populated Transcription must not encode to zero bytes"
    );
    assert_eq!(
        bytes.len(),
        original.encoded_len(),
        "encoded_len must agree with the bytes actually written"
    );

    let parsed =
        s2t::Transcription::decode(bytes.as_slice()).expect("re-parsing our own bytes must work");
    assert_eq!(parsed, original);

    // Spot-check the individual fields too: a PartialEq on two identically broken values would
    // still pass above.
    assert_eq!(parsed.transcription, "guten tag");
    assert_eq!(parsed.confidence_score, 0.93);
    assert_eq!(parsed.words.len(), 2);
    assert_eq!(parsed.words[0].word, "guten");
    assert_eq!(parsed.words[0].word_alternatives[0].word, "guetn");
    assert!(
        parsed.words[1].word_alternatives.is_empty(),
        "an empty repeated field must stay empty, not gain the sibling's entries"
    );
    assert_eq!(parsed.alternatives[0].transcript, "guten tach");
}

#[test]
fn a_default_transcription_round_trips_to_zero_bytes() {
    let empty = s2t::Transcription::default();

    assert_eq!(empty.transcription, "");
    assert_eq!(empty.confidence_score, 0.0);
    assert!(empty.words.is_empty());

    let bytes = empty.encode_to_vec();
    assert!(
        bytes.is_empty(),
        "proto3 must not put unset fields on the wire, got {bytes:?}"
    );
    assert_eq!(s2t::Transcription::decode(bytes.as_slice()).unwrap(), empty);
}

/// The S2T API declares 66 scalar proto3 `optional` (explicit presence) fields; the cloud-provider
/// configs are the ones a caller hits first. An unset field and a field explicitly set to its zero
/// value are two DIFFERENT values and must stay distinguishable across the wire - a generator that
/// collapses them makes `false` and `0` unsendable, which is exactly how a provider feature gets
/// silently re-enabled after a caller turned it off.
#[test]
fn an_explicit_presence_field_distinguishes_unset_from_zero() {
    let unset = s2t::S2tCloudProviderConfigGoogle {
        enable_automatic_punctuation: None,
        max_alternatives: None,
        ..Default::default()
    };
    let explicit_zero = s2t::S2tCloudProviderConfigGoogle {
        enable_automatic_punctuation: Some(false),
        max_alternatives: Some(0),
        ..Default::default()
    };

    let unset_bytes = unset.encode_to_vec();
    let zero_bytes = explicit_zero.encode_to_vec();
    assert!(
        unset_bytes.is_empty(),
        "an unset presence field must not occupy the wire, got {unset_bytes:?}"
    );
    assert!(
        !zero_bytes.is_empty(),
        "an explicitly zeroed presence field must occupy the wire"
    );

    let parsed_unset = s2t::S2tCloudProviderConfigGoogle::decode(unset_bytes.as_slice()).unwrap();
    assert_eq!(parsed_unset.enable_automatic_punctuation, None);
    assert_eq!(parsed_unset.max_alternatives, None);

    let parsed_zero = s2t::S2tCloudProviderConfigGoogle::decode(zero_bytes.as_slice()).unwrap();
    assert_eq!(parsed_zero.enable_automatic_punctuation, Some(false));
    assert_eq!(parsed_zero.max_alternatives, Some(0));
}

/// A presence field nested three messages deep has to behave the same way - this is the shape the
/// pipeline configs actually use.
#[test]
fn a_nested_explicit_presence_field_distinguishes_unset_from_zero() {
    let config = |active: Option<bool>| s2t::Speech2TextConfig {
        id: "de_DE_ondewo".to_string(),
        streaming_server: Some(s2t::StreamingServer {
            streaming_speech_recognition: Some(s2t::StreamingSpeechRecognition {
                turn_detection: Some(s2t::TurnDetectionOptions {
                    active,
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        }),
        ..Default::default()
    };

    let turn_detection_active = |config: &s2t::Speech2TextConfig| -> Option<bool> {
        config
            .streaming_server
            .as_ref()?
            .streaming_speech_recognition
            .as_ref()?
            .turn_detection
            .as_ref()?
            .active
    };

    let unset_bytes = config(None).encode_to_vec();
    let zero_bytes = config(Some(false)).encode_to_vec();
    assert_ne!(unset_bytes, zero_bytes);

    assert_eq!(
        turn_detection_active(&s2t::Speech2TextConfig::decode(unset_bytes.as_slice()).unwrap()),
        None
    );
    assert_eq!(
        turn_detection_active(&s2t::Speech2TextConfig::decode(zero_bytes.as_slice()).unwrap()),
        Some(false)
    );
}

/// Map fields have to survive as maps, keys and values intact - including an empty value, which a
/// generator that skipped "empty" entries would drop.
#[test]
fn a_map_field_round_trips() {
    let mut default_headers = HashMap::new();
    default_headers.insert("X-Ondewo-Tenant".to_string(), "acme".to_string());
    default_headers.insert("X-Ondewo-Trace".to_string(), String::new());
    let mut logit_bias = HashMap::new();
    logit_bias.insert("1734".to_string(), -100);
    logit_bias.insert("9021".to_string(), 42);

    let options = s2t::OpenaiLlmOptions {
        model: "gpt-4o-mini".to_string(),
        default_headers: default_headers.clone(),
        logit_bias: logit_bias.clone(),
        max_retries: Some(0),
        ..Default::default()
    };

    let parsed = s2t::OpenaiLlmOptions::decode(options.encode_to_vec().as_slice()).unwrap();
    assert_eq!(parsed, options);
    assert_eq!(parsed.default_headers, default_headers);
    assert_eq!(parsed.logit_bias, logit_bias);
    assert_eq!(
        parsed.default_headers["X-Ondewo-Trace"], "",
        "a map entry with an empty value must not be dropped"
    );
    assert_eq!(parsed.max_retries, Some(0));
}

/// Decoding tolerates fields it does not know: an unknown tag is skipped, not an error.
#[test]
fn decoding_skips_an_unknown_field() {
    let mut bytes = s2t::S2tPipelineId {
        id: "de_DE_ondewo".to_string(),
    }
    .encode_to_vec();
    // tag 999, wire type 0 (varint), value 1
    bytes.extend_from_slice(&[0xB8, 0x3E, 0x01]);

    let parsed = s2t::S2tPipelineId::decode(bytes.as_slice())
        .expect("an unknown field must be skipped, not rejected");
    assert_eq!(parsed.id, "de_DE_ondewo");
}

#[test]
fn decoding_rejects_a_truncated_message() {
    let bytes = s2t::S2tDescription {
        language: "de".to_string(),
        pipeline_owner: "ondewo".to_string(),
        domain: "general".to_string(),
        comments: "the default german pipeline".to_string(),
    }
    .encode_to_vec();
    let truncated = &bytes[..bytes.len() - 1];

    assert!(
        s2t::S2tDescription::decode(truncated).is_err(),
        "a truncated message must not decode silently"
    );
}

/// The zero value of an enum is the one a default-constructed message carries, so it must be the
/// variant the proto declares as `= 0`. For `Decoding` that is `DEFAULT` - "let the pipeline
/// config decide" - not an `…_UNSPECIFIED` variant.
#[test]
fn the_enum_zero_value_is_the_variant_the_proto_declares_as_zero() {
    assert_eq!(s2t::Decoding::Default as i32, 0);
    assert_eq!(s2t::Decoding::try_from(0), Ok(s2t::Decoding::Default));
    assert_eq!(
        s2t::TranscribeRequestConfig::default().decoding,
        s2t::Decoding::Default as i32,
        "a default message must carry the enum's zero value"
    );

    assert_eq!(s2t::Decoding::Default.as_str_name(), "DEFAULT");
    assert_eq!(
        s2t::Decoding::from_str_name("DEFAULT"),
        Some(s2t::Decoding::Default)
    );
    assert_eq!(s2t::Decoding::from_str_name("NOT_A_VARIANT"), None);
    assert!(
        s2t::Decoding::try_from(9_999).is_err(),
        "an out-of-range discriminant must not map to a variant"
    );
}

/// A non-zero enum value has to travel as its discriminant, not as the zero value.
#[test]
fn a_non_zero_enum_value_round_trips() {
    let request = s2t::TranscribeRequestConfig {
        s2t_pipeline_id: "de_DE_ondewo".to_string(),
        decoding: s2t::Decoding::BeamSearchWithLm as i32,
        language: Some("de".to_string()),
        ..Default::default()
    };

    let parsed = s2t::TranscribeRequestConfig::decode(request.encode_to_vec().as_slice()).unwrap();
    assert_eq!(parsed, request);
    assert_eq!(
        s2t::Decoding::try_from(parsed.decoding),
        Ok(s2t::Decoding::BeamSearchWithLm)
    );
    assert_eq!(
        s2t::Decoding::BeamSearchWithLm.as_str_name(),
        "BEAM_SEARCH_WITH_LM"
    );
}

/// Repeated and nested message fields have to nest, not flatten.
#[test]
fn a_nested_and_repeated_message_round_trips() {
    let response = s2t::TranscribeFileResponse {
        transcriptions: vec![
            sample_transcription(),
            s2t::Transcription {
                transcription: "second".to_string(),
                ..Default::default()
            },
        ],
        time: 0.42,
        audio_uuid: "audio-uuid-1".to_string(),
    };

    let parsed = s2t::TranscribeFileResponse::decode(response.encode_to_vec().as_slice()).unwrap();
    assert_eq!(parsed, response);
    assert_eq!(parsed.transcriptions.len(), 2);
    assert_eq!(parsed.transcriptions[1].transcription, "second");
    assert_eq!(
        parsed.transcriptions[0].words.len(),
        2,
        "the nested repeated field must survive being nested"
    );
    assert_eq!(parsed.audio_uuid, "audio-uuid-1");
}

/// A oneof carries exactly the variant that was set, and nothing when none was.
#[test]
fn a_oneof_round_trips_as_the_variant_that_was_set() {
    let with_config = s2t::TranscribeStreamResponse {
        audio_uuid: "audio-uuid-1".to_string(),
        r#final: true,
        oneof_config: Some(s2t::transcribe_stream_response::OneofConfig::Config(
            s2t::TranscribeRequestConfig {
                s2t_pipeline_id: "de_DE_ondewo".to_string(),
                language: Some(String::new()),
                ..Default::default()
            },
        )),
        ..Default::default()
    };

    let parsed =
        s2t::TranscribeStreamResponse::decode(with_config.encode_to_vec().as_slice()).unwrap();
    assert_eq!(parsed, with_config);
    match parsed.oneof_config {
        Some(s2t::transcribe_stream_response::OneofConfig::Config(config)) => {
            assert_eq!(config.s2t_pipeline_id, "de_DE_ondewo");
            assert_eq!(
                config.language,
                Some(String::new()),
                "a presence field inside a oneof must keep its presence too"
            );
        }
        None => panic!("the oneof variant that was set must come back set"),
    }

    assert_eq!(
        s2t::TranscribeStreamResponse::decode(
            s2t::TranscribeStreamResponse::default()
                .encode_to_vec()
                .as_slice()
        )
        .unwrap()
        .oneof_config,
        None
    );
}
