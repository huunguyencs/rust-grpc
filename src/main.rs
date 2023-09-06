use tonic::{transport::Server, Request, Response, Status};

use calculate::calculate_server::{Calculate, CalculateServer};
use calculate::{BinaryRequest, ResultResponse};

pub mod calculate {
    tonic::include_proto!("calculate");
}

#[derive(Debug, Default)]
pub struct MyCalculate {}

#[tonic::async_trait]
impl Calculate for MyCalculate {
    async fn add(
        &self,
        request: Request<BinaryRequest>,
    ) -> Result<Response<ResultResponse>, Status> {
        println!("Got a request: {:?}", request);

        // request.get_ref().operant_first;

        let first = request.get_ref().operant_first;
        let second = request.get_ref().operant_second;

        let reply = calculate::ResultResponse {
            result: first + second,
        };

        Ok(Response::new(reply))
    }

    async fn minus(
        &self,
        request: Request<BinaryRequest>,
    ) -> Result<Response<ResultResponse>, Status> {
        println!("Got a request: {:?}", request);

        let first = request.get_ref().operant_first;
        let second = request.get_ref().operant_second;

        let reply = calculate::ResultResponse {
            result: first - second,
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let calculator = MyCalculate::default();

    Server::builder()
        .add_service(CalculateServer::new(calculator))
        .serve(addr)
        .await?;

    Ok(())
}
