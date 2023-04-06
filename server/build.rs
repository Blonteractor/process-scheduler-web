fn main() {
    tonic_build::configure()
        .build_client(false)
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile(
            &["../proto/scheduler/scheduler.proto"],
            &["../proto/scheduler"],
        )
        .unwrap_or_else(|e| panic!("Failed to compile protos: {:?}", e));
}
