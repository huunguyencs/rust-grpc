# Rust gRPC using tonic

## 1. Get started:

Install `protobuf-compiler` and `libprotobuf-dev`

```
sudo apt install protobuf-compiler
sudo apt install libprotobuf-dev
```

## 2. Run:

```
cargo run --bin calculate-server
```

## 3. Test

```
grpcurl -plaintext -import-path ./proto -proto calculate.proto -d '{"operand_first": 1, "operand_second": 2}' '127.0.0.1:50051' calculate.Calculate/Add
```

Response:
{
"result": 3
}

## 4. Build docker

```
docker build -t calculate-server .
```

```
docker run -dp 0.0.0.0:50051:50051 calculate-server
```
