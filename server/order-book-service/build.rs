
fn main() ->  Result<(), Box<dyn std::error::Error>>{
    let proto_file = "proto/order-book.proto";
    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .out_dir("src/")
        .compile(&[proto_file], &["."])
        .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));
    Ok(())
}