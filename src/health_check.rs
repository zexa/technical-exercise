use crate::ApplicationState;
use tide::StatusCode;
use tide::{Request, Response};

pub async fn health_check(_: Request<ApplicationState>) -> tide::Result {
    Ok(Response::new(StatusCode::Ok))
}
