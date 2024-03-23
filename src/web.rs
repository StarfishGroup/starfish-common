use axum::{
    extract::{FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json as WebJson, RequestExt, Router,
};
use derive_more::Display;
use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use std::borrow::Cow;

#[derive(Debug, Display)]
#[display("错误码({_0}), 错误消息({_1})")]
pub struct ErrorMsg(u64, Cow<'static, str>);

impl From<(u64, Cow<'static, str>)> for ErrorMsg {
    fn from(value: (u64, Cow<'static, str>)) -> Self {
        Self(value.0, value.1)
    }
}

pub struct ErrorResult(pub anyhow::Error);

impl IntoResponse for ErrorResult {
    fn into_response(self) -> Response {
        let data = if let Some(err_msg) = self.0.downcast_ref::<ErrorMsg>() {
            WebJson(json!({
                "code" : err_msg.0,
                "msg" :err_msg.1,
                "data" : Value::Null,
            }))
        } else {
            WebJson(json!({
                "code" : 1,
                "msg" :"server error",
                "data" : Value::Null,
            }))
        };
        (StatusCode::OK, data).into_response()
    }
}

impl From<anyhow::Error> for ErrorResult {
    fn from(err: anyhow::Error) -> Self {
        Self(err)
    }
}

pub struct Json<T>(pub T);

impl<T: serde::Serialize> IntoResponse for Json<T> {
    fn into_response(self) -> Response {
        (
            StatusCode::OK,
            WebJson(json!({
                "code":0,
                "msg":"success",
                "data":self.0,
            })),
        )
            .into_response()
    }
}

#[async_trait::async_trait]
impl<T, S> FromRequest<S> for Json<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
    T: 'static,
{
    type Rejection = Response;

    async fn from_request(req: Request, _state: &S) -> core::result::Result<Self, Self::Rejection> {
        match req.extract().await {
            Ok(WebJson(payload)) => return Ok(Self(payload)),
            Err(err) => Err((
                StatusCode::OK,
                WebJson(json!({
                    "code":2,
                    "msg":err.to_string(),
                    "data":Value::Null,
                })),
            )
                .into_response()),
        }
    }
}

pub type Result<T> = core::result::Result<Json<T>, ErrorResult>;

pub async fn init<A: tokio::net::ToSocketAddrs>(
    bind: A,
    register: fn(Router) -> Router,
) -> anyhow::Result<()> {
    let router = Router::new();
    let router = register(router);
    let router = router.layer(
        tower_http::trace::TraceLayer::new_for_http().make_span_with(tracing::Span::current()),
    );
    let listener = tokio::net::TcpListener::bind(bind).await?;
    axum::serve(listener, router).await?;
    Ok(())
}
