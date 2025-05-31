use splix_api::{HelloReply, HelloRequest, SplixApi};
use tonic::{Request, Response};

pub struct SplixService;

impl SplixService {
    pub fn new() -> Self {
        Self
    }
}

#[tonic::async_trait]
impl SplixApi for SplixService {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> tonic::Result<Response<HelloReply>> {
        println!("got request: {request:?}");

        Ok(Response::new(HelloReply {
            message: format!("hello, {}", request.into_inner().name),
        }))
    }
}
