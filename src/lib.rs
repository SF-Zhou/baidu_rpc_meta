mod gen {
    include!(concat!(env!("OUT_DIR"), concat!("/", "baidu_rpc_meta.rs")));
}

pub use gen::baidu_rpc_meta::*;

#[cfg(test)]
mod test {
    use crate::brpc::policy::*;
    use pilota::prost::Message;

    #[test]
    fn encode_and_decode() {
        let mut rpc_meta = RpcMeta::default();
        assert_eq!(rpc_meta.encoded_len(), 0);

        let mut request_meta = RpcRequestMeta::default();
        request_meta.service_name = "test.service".into();
        request_meta.method_name = "test_method".into();
        rpc_meta.request = Some(request_meta);
        assert_ne!(rpc_meta.encoded_len(), 0);

        let mut buf = bytes::BytesMut::with_capacity(rpc_meta.encoded_len());
        let encode_result = rpc_meta.encode(&mut buf);
        assert_eq!(encode_result.is_ok(), true);

        let decode_result = RpcMeta::decode(buf);
        if let Some(request_meta) = decode_result.ok().and_then(|rpc_meta| rpc_meta.request) {
            assert_eq!(request_meta.service_name, "test.service");
        } else {
            assert!(false);
        }
    }

    struct S;
    impl super::brpc::policy::DummyService for S {
        async fn dummy(
            &self,
            req: &RpcMeta,
        ) -> std::result::Result<RpcMeta, Box<dyn std::error::Error>> {
            Ok(req.clone())
        }
    }

    #[test]
    fn demo_service() {
        let server = DummyServiceServer::new(S);

        let mut rpc_meta = RpcMeta::default();
        assert_eq!(rpc_meta.encoded_len(), 0);

        let mut request_meta = RpcRequestMeta::default();
        request_meta.service_name = "test.service".into();
        request_meta.method_name = "test_method".into();
        rpc_meta.request = Some(request_meta);
        assert_ne!(rpc_meta.encoded_len(), 0);

        let mut buf = bytes::BytesMut::with_capacity(rpc_meta.encoded_len());
        let encode_result = rpc_meta.encode(&mut buf);
        assert_eq!(encode_result.is_ok(), true);

        let result = futures::executor::block_on(
            server.call("/brpc.policy.DummyService/dummy", buf.as_ref()),
        );
        assert_eq!(result.is_ok(), true);
    }
}
