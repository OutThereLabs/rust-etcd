// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]


// interface

pub trait Election {
    fn campaign(&self, o: ::grpc::RequestOptions, p: super::v3election::CampaignRequest) -> ::grpc::SingleResponse<super::v3election::CampaignResponse>;

    fn proclaim(&self, o: ::grpc::RequestOptions, p: super::v3election::ProclaimRequest) -> ::grpc::SingleResponse<super::v3election::ProclaimResponse>;

    fn leader(&self, o: ::grpc::RequestOptions, p: super::v3election::LeaderRequest) -> ::grpc::SingleResponse<super::v3election::LeaderResponse>;

    fn observe(&self, o: ::grpc::RequestOptions, p: super::v3election::LeaderRequest) -> ::grpc::StreamingResponse<super::v3election::LeaderResponse>;

    fn resign(&self, o: ::grpc::RequestOptions, p: super::v3election::ResignRequest) -> ::grpc::SingleResponse<super::v3election::ResignResponse>;
}

// client

pub struct ElectionClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_Campaign: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::v3election::CampaignRequest, super::v3election::CampaignResponse>>,
    method_Proclaim: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::v3election::ProclaimRequest, super::v3election::ProclaimResponse>>,
    method_Leader: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::v3election::LeaderRequest, super::v3election::LeaderResponse>>,
    method_Observe: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::v3election::LeaderRequest, super::v3election::LeaderResponse>>,
    method_Resign: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::v3election::ResignRequest, super::v3election::ResignResponse>>,
}

impl ::grpc::ClientStub for ElectionClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        ElectionClient {
            grpc_client: grpc_client,
            method_Campaign: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/v3electionpb.Election/Campaign".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_Proclaim: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/v3electionpb.Election/Proclaim".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_Leader: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/v3electionpb.Election/Leader".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_Observe: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/v3electionpb.Election/Observe".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_Resign: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/v3electionpb.Election/Resign".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl Election for ElectionClient {
    fn campaign(&self, o: ::grpc::RequestOptions, p: super::v3election::CampaignRequest) -> ::grpc::SingleResponse<super::v3election::CampaignResponse> {
        self.grpc_client.call_unary(o, p, self.method_Campaign.clone())
    }

    fn proclaim(&self, o: ::grpc::RequestOptions, p: super::v3election::ProclaimRequest) -> ::grpc::SingleResponse<super::v3election::ProclaimResponse> {
        self.grpc_client.call_unary(o, p, self.method_Proclaim.clone())
    }

    fn leader(&self, o: ::grpc::RequestOptions, p: super::v3election::LeaderRequest) -> ::grpc::SingleResponse<super::v3election::LeaderResponse> {
        self.grpc_client.call_unary(o, p, self.method_Leader.clone())
    }

    fn observe(&self, o: ::grpc::RequestOptions, p: super::v3election::LeaderRequest) -> ::grpc::StreamingResponse<super::v3election::LeaderResponse> {
        self.grpc_client.call_server_streaming(o, p, self.method_Observe.clone())
    }

    fn resign(&self, o: ::grpc::RequestOptions, p: super::v3election::ResignRequest) -> ::grpc::SingleResponse<super::v3election::ResignResponse> {
        self.grpc_client.call_unary(o, p, self.method_Resign.clone())
    }
}

// server

pub struct ElectionServer;


impl ElectionServer {
    pub fn new_service_def<H : Election + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/v3electionpb.Election",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/v3electionpb.Election/Campaign".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.campaign(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/v3electionpb.Election/Proclaim".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.proclaim(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/v3electionpb.Election/Leader".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.leader(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/v3electionpb.Election/Observe".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerServerStreaming::new(move |o, p| handler_copy.observe(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/v3electionpb.Election/Resign".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.resign(o, p))
                    },
                ),
            ],
        )
    }
}
