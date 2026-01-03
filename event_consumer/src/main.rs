mod line;
mod executor;

use aws_lambda_events::event::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use lambda_runtime::{service_fn, LambdaEvent};
use log::{debug, error};
use serde_json::from_str;
use crate::executor::Executor;
use crate::executor::parroting::Parroting;
use crate::line::messaging_api::request::EventObjectRequest;

async fn handler(event: LambdaEvent<ApiGatewayProxyRequest>)
                 -> Result<ApiGatewayProxyResponse, lambda_runtime::Error>
{
    debug!("Event: {:?}", event);

    // parse
    if let Some(body) = event.payload.body {
        let request: EventObjectRequest = from_str(body.as_str())?;
        debug!("LINE Request body: {:?}", request);

        // todo 署名検証

        // 処理
        let executor = Parroting {};
        for event in request.events {
            executor.execute(&event).await?
        }
    } else {
        error!("Request Body not found.");
    }

    // レスポンス作成
    let mut response = ApiGatewayProxyResponse::default();
    response.status_code = 200;
    response.body = None;
    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    dotenv::dotenv().ok();
    env_logger::init();
    lambda_runtime::run(service_fn(handler)).await?;
    Ok(())
}
