fn main() {
    tonic_build::configure()
        .build_client(false)
        .compile(&["../proto/scheduler/scheduler.proto"], &["../proto/scheduler"])
        .unwrap_or_else(|e| panic!("Failed to compile protos: {:?}", e));
}
