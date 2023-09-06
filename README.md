# Rust gRPC using tonic

## 1. Get started:

Install `protobuf-compiler` and `libprotobuf-dev`

```
sudo apt install protobuf-compiler
sudo apt install libprotobuf-dev
```

## 2. Run:

```
cargo run --bin helloworld-server
```

## 3. Test

```
grpcurl -plaintext -import-path ./proto -proto helloworld.proto -d '{"name": "Tonic"}' '[::1]:50051' helloworld.Greeter/SayHello
```

Response:
{
"message": "Hello Tonic!"
}
