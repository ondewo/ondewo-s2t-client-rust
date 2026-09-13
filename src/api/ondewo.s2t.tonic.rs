// @generated
/// Generated client implementations.
pub mod speech2_text_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct Speech2TextClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl Speech2TextClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> Speech2TextClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> Speech2TextClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            Speech2TextClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn transcribe_file(
            &mut self,
            request: impl tonic::IntoRequest<super::TranscribeFileRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TranscribeFileResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.s2t.Speech2Text/TranscribeFile",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.s2t.Speech2Text", "TranscribeFile"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn transcribe_stream(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::TranscribeStreamRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::TranscribeStreamResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.s2t.Speech2Text/TranscribeStream",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.s2t.Speech2Text", "TranscribeStream"));
            self.inner.streaming(req, path, codec).await
        }
        pub async fn get_s2t_pipeline(
            &mut self,
            request: impl tonic::IntoRequest<super::S2tPipelineId>,
        ) -> std::result::Result<
            tonic::Response<super::Speech2TextConfig>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.s2t.Speech2Text/GetS2tPipeline",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.s2t.Speech2Text", "GetS2tPipeline"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_s2t_pipeline(
            &mut self,
            request: impl tonic::IntoRequest<super::Speech2TextConfig>,
        ) -> std::result::Result<tonic::Response<super::S2tPipelineId>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.s2t.Speech2Text/CreateS2tPipeline",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.s2t.Speech2Text", "CreateS2tPipeline"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_s2t_pipeline(
            &mut self,
            request: impl tonic::IntoRequest<super::S2tPipelineId>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.s2t.Speech2Text/DeleteS2tPipeline",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.s2t.Speech2Text", "DeleteS2tPipeline"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_s2t_pipeline(
            &mut self,
            request: impl tonic::IntoRequest<super::Speech2TextConfig>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.s2t.Speech2Text/UpdateS2tPipeline",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.s2t.Speech2Text", "UpdateS2tPipeline"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_s2t_pipelines(
            &mut self,
            request: impl tonic::IntoRequest<super::ListS2tPipelinesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListS2tPipelinesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.s2t.Speech2Text/ListS2tPipelines",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.s2t.Speech2Text", "ListS2tPipelines"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_s2t_languages(
            &mut self,
            request: impl tonic::IntoRequest<super::ListS2tLanguagesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListS2tLanguagesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.s2t.Speech2Text/ListS2tLanguages",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.s2t.Speech2Text", "ListS2tLanguages"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_s2t_domains(
            &mut self,
            request: impl tonic::IntoRequest<super::ListS2tDomainsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListS2tDomainsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.s2t.Speech2Text/ListS2tDomains",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.s2t.Speech2Text", "ListS2tDomains"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_service_info(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<
            tonic::Response<super::S2tGetServiceInfoResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.s2t.Speech2Text/GetServiceInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.s2t.Speech2Text", "GetServiceInfo"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_s2t_language_models(
            &mut self,
            request: impl tonic::IntoRequest<super::ListS2tLanguageModelsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListS2tLanguageModelsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.s2t.Speech2Text/ListS2tLanguageModels",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("ondewo.s2t.Speech2Text", "ListS2tLanguageModels"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_user_language_model(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateUserLanguageModelRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.s2t.Speech2Text/CreateUserLanguageModel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("ondewo.s2t.Speech2Text", "CreateUserLanguageModel"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_user_language_model(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteUserLanguageModelRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.s2t.Speech2Text/DeleteUserLanguageModel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("ondewo.s2t.Speech2Text", "DeleteUserLanguageModel"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_data_to_user_language_model(
            &mut self,
            request: impl tonic::IntoRequest<super::AddDataToUserLanguageModelRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.s2t.Speech2Text/AddDataToUserLanguageModel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "ondewo.s2t.Speech2Text",
                        "AddDataToUserLanguageModel",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn train_user_language_model(
            &mut self,
            request: impl tonic::IntoRequest<super::TrainUserLanguageModelRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.s2t.Speech2Text/TrainUserLanguageModel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("ondewo.s2t.Speech2Text", "TrainUserLanguageModel"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_s2t_normalization_pipelines(
            &mut self,
            request: impl tonic::IntoRequest<super::ListS2tNormalizationPipelinesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListS2tNormalizationPipelinesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.s2t.Speech2Text/ListS2tNormalizationPipelines",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "ondewo.s2t.Speech2Text",
                        "ListS2tNormalizationPipelines",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod speech2_text_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with Speech2TextServer.
    #[async_trait]
    pub trait Speech2Text: std::marker::Send + std::marker::Sync + 'static {
        async fn transcribe_file(
            &self,
            request: tonic::Request<super::TranscribeFileRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TranscribeFileResponse>,
            tonic::Status,
        >;
        /// Server streaming response type for the TranscribeStream method.
        type TranscribeStreamStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<
                    super::TranscribeStreamResponse,
                    tonic::Status,
                >,
            >
            + std::marker::Send
            + 'static;
        async fn transcribe_stream(
            &self,
            request: tonic::Request<tonic::Streaming<super::TranscribeStreamRequest>>,
        ) -> std::result::Result<
            tonic::Response<Self::TranscribeStreamStream>,
            tonic::Status,
        >;
        async fn get_s2t_pipeline(
            &self,
            request: tonic::Request<super::S2tPipelineId>,
        ) -> std::result::Result<
            tonic::Response<super::Speech2TextConfig>,
            tonic::Status,
        >;
        async fn create_s2t_pipeline(
            &self,
            request: tonic::Request<super::Speech2TextConfig>,
        ) -> std::result::Result<tonic::Response<super::S2tPipelineId>, tonic::Status>;
        async fn delete_s2t_pipeline(
            &self,
            request: tonic::Request<super::S2tPipelineId>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        async fn update_s2t_pipeline(
            &self,
            request: tonic::Request<super::Speech2TextConfig>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        async fn list_s2t_pipelines(
            &self,
            request: tonic::Request<super::ListS2tPipelinesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListS2tPipelinesResponse>,
            tonic::Status,
        >;
        async fn list_s2t_languages(
            &self,
            request: tonic::Request<super::ListS2tLanguagesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListS2tLanguagesResponse>,
            tonic::Status,
        >;
        async fn list_s2t_domains(
            &self,
            request: tonic::Request<super::ListS2tDomainsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListS2tDomainsResponse>,
            tonic::Status,
        >;
        async fn get_service_info(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<
            tonic::Response<super::S2tGetServiceInfoResponse>,
            tonic::Status,
        >;
        async fn list_s2t_language_models(
            &self,
            request: tonic::Request<super::ListS2tLanguageModelsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListS2tLanguageModelsResponse>,
            tonic::Status,
        >;
        async fn create_user_language_model(
            &self,
            request: tonic::Request<super::CreateUserLanguageModelRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        async fn delete_user_language_model(
            &self,
            request: tonic::Request<super::DeleteUserLanguageModelRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        async fn add_data_to_user_language_model(
            &self,
            request: tonic::Request<super::AddDataToUserLanguageModelRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        async fn train_user_language_model(
            &self,
            request: tonic::Request<super::TrainUserLanguageModelRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        async fn list_s2t_normalization_pipelines(
            &self,
            request: tonic::Request<super::ListS2tNormalizationPipelinesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListS2tNormalizationPipelinesResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct Speech2TextServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> Speech2TextServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for Speech2TextServer<T>
    where
        T: Speech2Text,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/ondewo.s2t.Speech2Text/TranscribeFile" => {
                    #[allow(non_camel_case_types)]
                    struct TranscribeFileSvc<T: Speech2Text>(pub Arc<T>);
                    impl<
                        T: Speech2Text,
                    > tonic::server::UnaryService<super::TranscribeFileRequest>
                    for TranscribeFileSvc<T> {
                        type Response = super::TranscribeFileResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TranscribeFileRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Speech2Text>::transcribe_file(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = TranscribeFileSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ondewo.s2t.Speech2Text/TranscribeStream" => {
                    #[allow(non_camel_case_types)]
                    struct TranscribeStreamSvc<T: Speech2Text>(pub Arc<T>);
                    impl<
                        T: Speech2Text,
                    > tonic::server::StreamingService<super::TranscribeStreamRequest>
                    for TranscribeStreamSvc<T> {
                        type Response = super::TranscribeStreamResponse;
                        type ResponseStream = T::TranscribeStreamStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::TranscribeStreamRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Speech2Text>::transcribe_stream(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = TranscribeStreamSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ondewo.s2t.Speech2Text/GetS2tPipeline" => {
                    #[allow(non_camel_case_types)]
                    struct GetS2tPipelineSvc<T: Speech2Text>(pub Arc<T>);
                    impl<
                        T: Speech2Text,
                    > tonic::server::UnaryService<super::S2tPipelineId>
                    for GetS2tPipelineSvc<T> {
                        type Response = super::Speech2TextConfig;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::S2tPipelineId>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Speech2Text>::get_s2t_pipeline(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetS2tPipelineSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ondewo.s2t.Speech2Text/CreateS2tPipeline" => {
                    #[allow(non_camel_case_types)]
                    struct CreateS2tPipelineSvc<T: Speech2Text>(pub Arc<T>);
                    impl<
                        T: Speech2Text,
                    > tonic::server::UnaryService<super::Speech2TextConfig>
                    for CreateS2tPipelineSvc<T> {
                        type Response = super::S2tPipelineId;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Speech2TextConfig>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Speech2Text>::create_s2t_pipeline(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = CreateS2tPipelineSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ondewo.s2t.Speech2Text/DeleteS2tPipeline" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteS2tPipelineSvc<T: Speech2Text>(pub Arc<T>);
                    impl<
                        T: Speech2Text,
                    > tonic::server::UnaryService<super::S2tPipelineId>
                    for DeleteS2tPipelineSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::S2tPipelineId>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Speech2Text>::delete_s2t_pipeline(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = DeleteS2tPipelineSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ondewo.s2t.Speech2Text/UpdateS2tPipeline" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateS2tPipelineSvc<T: Speech2Text>(pub Arc<T>);
                    impl<
                        T: Speech2Text,
                    > tonic::server::UnaryService<super::Speech2TextConfig>
                    for UpdateS2tPipelineSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Speech2TextConfig>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Speech2Text>::update_s2t_pipeline(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = UpdateS2tPipelineSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ondewo.s2t.Speech2Text/ListS2tPipelines" => {
                    #[allow(non_camel_case_types)]
                    struct ListS2tPipelinesSvc<T: Speech2Text>(pub Arc<T>);
                    impl<
                        T: Speech2Text,
                    > tonic::server::UnaryService<super::ListS2tPipelinesRequest>
                    for ListS2tPipelinesSvc<T> {
                        type Response = super::ListS2tPipelinesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListS2tPipelinesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Speech2Text>::list_s2t_pipelines(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListS2tPipelinesSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ondewo.s2t.Speech2Text/ListS2tLanguages" => {
                    #[allow(non_camel_case_types)]
                    struct ListS2tLanguagesSvc<T: Speech2Text>(pub Arc<T>);
                    impl<
                        T: Speech2Text,
                    > tonic::server::UnaryService<super::ListS2tLanguagesRequest>
                    for ListS2tLanguagesSvc<T> {
                        type Response = super::ListS2tLanguagesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListS2tLanguagesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Speech2Text>::list_s2t_languages(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListS2tLanguagesSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ondewo.s2t.Speech2Text/ListS2tDomains" => {
                    #[allow(non_camel_case_types)]
                    struct ListS2tDomainsSvc<T: Speech2Text>(pub Arc<T>);
                    impl<
                        T: Speech2Text,
                    > tonic::server::UnaryService<super::ListS2tDomainsRequest>
                    for ListS2tDomainsSvc<T> {
                        type Response = super::ListS2tDomainsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListS2tDomainsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Speech2Text>::list_s2t_domains(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListS2tDomainsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ondewo.s2t.Speech2Text/GetServiceInfo" => {
                    #[allow(non_camel_case_types)]
                    struct GetServiceInfoSvc<T: Speech2Text>(pub Arc<T>);
                    impl<T: Speech2Text> tonic::server::UnaryService<()>
                    for GetServiceInfoSvc<T> {
                        type Response = super::S2tGetServiceInfoResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Speech2Text>::get_service_info(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetServiceInfoSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ondewo.s2t.Speech2Text/ListS2tLanguageModels" => {
                    #[allow(non_camel_case_types)]
                    struct ListS2tLanguageModelsSvc<T: Speech2Text>(pub Arc<T>);
                    impl<
                        T: Speech2Text,
                    > tonic::server::UnaryService<super::ListS2tLanguageModelsRequest>
                    for ListS2tLanguageModelsSvc<T> {
                        type Response = super::ListS2tLanguageModelsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListS2tLanguageModelsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Speech2Text>::list_s2t_language_models(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListS2tLanguageModelsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ondewo.s2t.Speech2Text/CreateUserLanguageModel" => {
                    #[allow(non_camel_case_types)]
                    struct CreateUserLanguageModelSvc<T: Speech2Text>(pub Arc<T>);
                    impl<
                        T: Speech2Text,
                    > tonic::server::UnaryService<super::CreateUserLanguageModelRequest>
                    for CreateUserLanguageModelSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreateUserLanguageModelRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Speech2Text>::create_user_language_model(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = CreateUserLanguageModelSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ondewo.s2t.Speech2Text/DeleteUserLanguageModel" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteUserLanguageModelSvc<T: Speech2Text>(pub Arc<T>);
                    impl<
                        T: Speech2Text,
                    > tonic::server::UnaryService<super::DeleteUserLanguageModelRequest>
                    for DeleteUserLanguageModelSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::DeleteUserLanguageModelRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Speech2Text>::delete_user_language_model(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = DeleteUserLanguageModelSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ondewo.s2t.Speech2Text/AddDataToUserLanguageModel" => {
                    #[allow(non_camel_case_types)]
                    struct AddDataToUserLanguageModelSvc<T: Speech2Text>(pub Arc<T>);
                    impl<
                        T: Speech2Text,
                    > tonic::server::UnaryService<
                        super::AddDataToUserLanguageModelRequest,
                    > for AddDataToUserLanguageModelSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::AddDataToUserLanguageModelRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Speech2Text>::add_data_to_user_language_model(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = AddDataToUserLanguageModelSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ondewo.s2t.Speech2Text/TrainUserLanguageModel" => {
                    #[allow(non_camel_case_types)]
                    struct TrainUserLanguageModelSvc<T: Speech2Text>(pub Arc<T>);
                    impl<
                        T: Speech2Text,
                    > tonic::server::UnaryService<super::TrainUserLanguageModelRequest>
                    for TrainUserLanguageModelSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TrainUserLanguageModelRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Speech2Text>::train_user_language_model(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = TrainUserLanguageModelSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ondewo.s2t.Speech2Text/ListS2tNormalizationPipelines" => {
                    #[allow(non_camel_case_types)]
                    struct ListS2tNormalizationPipelinesSvc<T: Speech2Text>(pub Arc<T>);
                    impl<
                        T: Speech2Text,
                    > tonic::server::UnaryService<
                        super::ListS2tNormalizationPipelinesRequest,
                    > for ListS2tNormalizationPipelinesSvc<T> {
                        type Response = super::ListS2tNormalizationPipelinesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ListS2tNormalizationPipelinesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Speech2Text>::list_s2t_normalization_pipelines(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListS2tNormalizationPipelinesSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for Speech2TextServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "ondewo.s2t.Speech2Text";
    impl<T> tonic::server::NamedService for Speech2TextServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
