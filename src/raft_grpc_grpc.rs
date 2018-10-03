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

const METHOD_RAFT_SERVICE_APPEND_ENTRIES_PIPELINE: ::grpcio::Method<super::raft_grpc::AppendEntriesRequest, super::raft_grpc::AppendEntriesPipelineResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/RaftService/AppendEntriesPipeline",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_RAFT_SERVICE_APPEND_ENTRIES: ::grpcio::Method<super::raft_grpc::AppendEntriesRequest, super::raft_grpc::AppendEntriesResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/RaftService/AppendEntries",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_RAFT_SERVICE_REQUEST_VOTE: ::grpcio::Method<super::raft_grpc::RequestVoteRequest, super::raft_grpc::RequestVoteResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/RaftService/RequestVote",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_RAFT_SERVICE_INSTALL_SNAPSHOT: ::grpcio::Method<super::raft_grpc::InstallSnapshotRequest, super::raft_grpc::InstallSnapshotResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/RaftService/InstallSnapshot",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

pub struct RaftServiceClient {
    client: ::grpcio::Client,
}

impl RaftServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        RaftServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn append_entries_pipeline_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::raft_grpc::AppendEntriesRequest>, ::grpcio::ClientDuplexReceiver<super::raft_grpc::AppendEntriesPipelineResponse>)> {
        self.client.duplex_streaming(&METHOD_RAFT_SERVICE_APPEND_ENTRIES_PIPELINE, opt)
    }

    pub fn append_entries_pipeline(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::raft_grpc::AppendEntriesRequest>, ::grpcio::ClientDuplexReceiver<super::raft_grpc::AppendEntriesPipelineResponse>)> {
        self.append_entries_pipeline_opt(::grpcio::CallOption::default())
    }

    pub fn append_entries_opt(&self, req: &super::raft_grpc::AppendEntriesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::raft_grpc::AppendEntriesResponse> {
        self.client.unary_call(&METHOD_RAFT_SERVICE_APPEND_ENTRIES, req, opt)
    }

    pub fn append_entries(&self, req: &super::raft_grpc::AppendEntriesRequest) -> ::grpcio::Result<super::raft_grpc::AppendEntriesResponse> {
        self.append_entries_opt(req, ::grpcio::CallOption::default())
    }

    pub fn append_entries_async_opt(&self, req: &super::raft_grpc::AppendEntriesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::raft_grpc::AppendEntriesResponse>> {
        self.client.unary_call_async(&METHOD_RAFT_SERVICE_APPEND_ENTRIES, req, opt)
    }

    pub fn append_entries_async(&self, req: &super::raft_grpc::AppendEntriesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::raft_grpc::AppendEntriesResponse>> {
        self.append_entries_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn request_vote_opt(&self, req: &super::raft_grpc::RequestVoteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::raft_grpc::RequestVoteResponse> {
        self.client.unary_call(&METHOD_RAFT_SERVICE_REQUEST_VOTE, req, opt)
    }

    pub fn request_vote(&self, req: &super::raft_grpc::RequestVoteRequest) -> ::grpcio::Result<super::raft_grpc::RequestVoteResponse> {
        self.request_vote_opt(req, ::grpcio::CallOption::default())
    }

    pub fn request_vote_async_opt(&self, req: &super::raft_grpc::RequestVoteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::raft_grpc::RequestVoteResponse>> {
        self.client.unary_call_async(&METHOD_RAFT_SERVICE_REQUEST_VOTE, req, opt)
    }

    pub fn request_vote_async(&self, req: &super::raft_grpc::RequestVoteRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::raft_grpc::RequestVoteResponse>> {
        self.request_vote_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn install_snapshot_opt(&self, req: &super::raft_grpc::InstallSnapshotRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::raft_grpc::InstallSnapshotResponse> {
        self.client.unary_call(&METHOD_RAFT_SERVICE_INSTALL_SNAPSHOT, req, opt)
    }

    pub fn install_snapshot(&self, req: &super::raft_grpc::InstallSnapshotRequest) -> ::grpcio::Result<super::raft_grpc::InstallSnapshotResponse> {
        self.install_snapshot_opt(req, ::grpcio::CallOption::default())
    }

    pub fn install_snapshot_async_opt(&self, req: &super::raft_grpc::InstallSnapshotRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::raft_grpc::InstallSnapshotResponse>> {
        self.client.unary_call_async(&METHOD_RAFT_SERVICE_INSTALL_SNAPSHOT, req, opt)
    }

    pub fn install_snapshot_async(&self, req: &super::raft_grpc::InstallSnapshotRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::raft_grpc::InstallSnapshotResponse>> {
        self.install_snapshot_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait RaftService {
    fn append_entries_pipeline(&self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::raft_grpc::AppendEntriesRequest>, sink: ::grpcio::DuplexSink<super::raft_grpc::AppendEntriesPipelineResponse>);
    fn append_entries(&self, ctx: ::grpcio::RpcContext, req: super::raft_grpc::AppendEntriesRequest, sink: ::grpcio::UnarySink<super::raft_grpc::AppendEntriesResponse>);
    fn request_vote(&self, ctx: ::grpcio::RpcContext, req: super::raft_grpc::RequestVoteRequest, sink: ::grpcio::UnarySink<super::raft_grpc::RequestVoteResponse>);
    fn install_snapshot(&self, ctx: ::grpcio::RpcContext, req: super::raft_grpc::InstallSnapshotRequest, sink: ::grpcio::UnarySink<super::raft_grpc::InstallSnapshotResponse>);
}

pub fn create_raft_service<S: RaftService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let instance = s.clone();
    builder = builder.add_duplex_streaming_handler(&METHOD_RAFT_SERVICE_APPEND_ENTRIES_PIPELINE, move |ctx, req, resp| {
        instance.append_entries_pipeline(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_RAFT_SERVICE_APPEND_ENTRIES, move |ctx, req, resp| {
        instance.append_entries(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_RAFT_SERVICE_REQUEST_VOTE, move |ctx, req, resp| {
        instance.request_vote(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_RAFT_SERVICE_INSTALL_SNAPSHOT, move |ctx, req, resp| {
        instance.install_snapshot(ctx, req, resp)
    });
    builder.build()
}
