/// This is our custom build script which allows us to compile proto files
/// for use with tonic for rpc
fn main() {
    tonic_build::compile_protos("proto/sessions.proto").unwrap();
}
