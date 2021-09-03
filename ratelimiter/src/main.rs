// Some implementation of the gRPC service using the shared library.

pub mod pb {
    tonic::include_proto!("nova.ratelimit.v1.proto");
}

fn main () {
    println!("Hello world!")
}