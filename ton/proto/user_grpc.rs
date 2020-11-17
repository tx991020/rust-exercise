// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

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


// server interface

pub trait Echo {
    fn unary_echo(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::user::EchoRequest>, resp: ::grpc::ServerResponseUnarySink<super::user::EchoResponse>) -> ::grpc::Result<()>;

    fn server_streaming_echo(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::user::EchoRequest>, resp: ::grpc::ServerResponseSink<super::user::EchoResponse>) -> ::grpc::Result<()>;

    fn client_streaming_echo(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequest<super::user::EchoRequest>, resp: ::grpc::ServerResponseUnarySink<super::user::EchoResponse>) -> ::grpc::Result<()>;

    fn bidirectional_streaming_echo(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequest<super::user::EchoRequest>, resp: ::grpc::ServerResponseSink<super::user::EchoResponse>) -> ::grpc::Result<()>;
}

// client

pub struct EchoClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
}

impl ::grpc::ClientStub for EchoClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        EchoClient {
            grpc_client: grpc_client,
        }
    }
}

impl EchoClient {
    pub fn unary_echo(&self, o: ::grpc::RequestOptions, req: super::user::EchoRequest) -> ::grpc::SingleResponse<super::user::EchoResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/grpc.examples.echo.Echo/UnaryEcho"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn server_streaming_echo(&self, o: ::grpc::RequestOptions, req: super::user::EchoRequest) -> ::grpc::StreamingResponse<super::user::EchoResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/grpc.examples.echo.Echo/ServerStreamingEcho"),
            streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_server_streaming(o, req, descriptor)
    }

    pub fn client_streaming_echo(&self, o: ::grpc::RequestOptions) -> impl ::std::future::Future<Output=::grpc::Result<(::grpc::ClientRequestSink<super::user::EchoRequest>, ::grpc::SingleResponse<super::user::EchoResponse>)>> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/grpc.examples.echo.Echo/ClientStreamingEcho"),
            streaming: ::grpc::rt::GrpcStreaming::ClientStreaming,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_client_streaming(o, descriptor)
    }

    pub fn bidirectional_streaming_echo(&self, o: ::grpc::RequestOptions) -> impl ::std::future::Future<Output=::grpc::Result<(::grpc::ClientRequestSink<super::user::EchoRequest>, ::grpc::StreamingResponse<super::user::EchoResponse>)>> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/grpc.examples.echo.Echo/BidirectionalStreamingEcho"),
            streaming: ::grpc::rt::GrpcStreaming::Bidi,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_bidi(o, descriptor)
    }
}

// server

pub struct EchoServer;


impl EchoServer {
    pub fn new_service_def<H : Echo + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/grpc.examples.echo.Echo",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/grpc.examples.echo.Echo/UnaryEcho"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).unary_echo(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/grpc.examples.echo.Echo/ServerStreamingEcho"),
                        streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerServerStreaming::new(move |ctx, req, resp| (*handler_copy).server_streaming_echo(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/grpc.examples.echo.Echo/ClientStreamingEcho"),
                        streaming: ::grpc::rt::GrpcStreaming::ClientStreaming,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerClientStreaming::new(move |ctx, req, resp| (*handler_copy).client_streaming_echo(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/grpc.examples.echo.Echo/BidirectionalStreamingEcho"),
                        streaming: ::grpc::rt::GrpcStreaming::Bidi,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerBidi::new(move |ctx, req, resp| (*handler_copy).bidirectional_streaming_echo(ctx, req, resp))
                    },
                ),
            ],
        )
    }
}
