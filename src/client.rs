//CODEGEN BELOW - DO NOT TOUCH ME
pub mod aura_sender {
    use proto_rs::{proto_message, proto_rpc};
    use crate::TxnData;
    use solana_signature::Signature;

    use crate::TxnAuthInterceptor;

    #[proto_rpc(rpc_package = "aura_sender_rpc", rpc_server = false, rpc_client = true, rpc_client_ctx = "TxnAuthInterceptor")]
    pub trait AuraSenderRpc {
        async fn send_transaction(
            &self,
            request: ::tonic::Request<TxnData>,
        ) -> ::core::result::Result<::tonic::Response<Done>, ::tonic::Status>;

        async fn ping(
            &self,
            request: ::tonic::Request<Ping>,
        ) -> ::core::result::Result<::tonic::Response<Pong>, ::tonic::Status>;

    }

    #[proto_message]
    pub struct Done {
        pub sigs: ::proto_rs::alloc::vec::Vec<Signature>,
    }

    #[proto_message]
    pub struct Ping {
        pub id: u32,
    }

    #[proto_message]
    pub struct Pong {
        pub id: u32,
    }

}
