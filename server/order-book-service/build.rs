
fn main() ->  Result<(), Box<dyn std::error::Error>>{
    let proto_file = "proto/order-book.proto";
    tonic_build::compile_protos(proto_file)?;

    Ok(())
}