use aws_lambda_events::{
    encodings::Body,
    event::apigw::{
        ApiGatewayProxyRequest, ApiGatewayProxyResponse,
    },
};
use http::HeaderMap;
use lambda_runtime::{handler_fn, Context, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    dbg!("cold start");
    let handler_fn = handler_fn(handler);
    lambda_runtime::run(handler_fn).await?;
    Ok(())
}

async fn handler(
    event: ApiGatewayProxyRequest,
    _: Context,
) -> Result<ApiGatewayProxyResponse, Error> {
    dbg!("in main", &event);
    let world = "world".to_string();
    let first_name = event
        .query_string_parameters
        .get("firstName")
        .unwrap_or(&world);

    dbg!(&first_name);
    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        headers: HeaderMap::new(),
        multi_value_headers: HeaderMap::new(),
        body: Some(Body::Text(format!(
            "Hello, {}!",
            first_name
        ))),
        is_base64_encoded: Some(false),
    })
}
