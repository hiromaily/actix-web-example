use crate::state;
use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    web, Error as ActixErr,
};
use actix_web_lab::middleware::Next;
use log::info;
use std::collections::HashMap;

pub async fn mw_auth_jwt(
    _admin_data: web::Data<state::AdminState>,
    _query: web::Query<HashMap<String, String>>,
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, ActixErr> {
    info!("middleware run");

    // TODO: retrieve token from req

    // TODO: validate token

    // TODO: If invalid, return 401 error

    // pre-processing
    next.call(req).await
    // post-processing
}
