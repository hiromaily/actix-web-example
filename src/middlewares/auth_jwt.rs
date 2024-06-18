use crate::state;
use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    error::ErrorUnauthorized,
    web, Error as ActixErr,
};
use actix_web_lab::middleware::Next;
use log::{debug, info};
use std::collections::HashMap;

// refer to
// - https://crates.io/crates/actix-web-lab
// - https://github.com/actix/examples/tree/master/middleware
// - https://github.com/openobserve/openobserve/blob/27eab898aa5b4dd74592299916c1df483282ea4a/src/common/meta/middleware_data.rs#L79

pub async fn mw_auth_jwt(
    admin_data: web::Data<state::AdminState>,
    _query: web::Query<HashMap<String, String>>,
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, ActixErr> {
    info!("middleware run");

    // retrieve token from request
    let headers = req.headers();
    //debug!("headers: {:?}", headers);

    let token = match headers.get("authorization") {
        Some(value) => value.to_str().unwrap().strip_prefix("Bearer ").unwrap(),
        None => "",
    };
    debug!("token: {}", token);

    if let Err(e) = admin_data.auth_usecase.validate_token(token) {
        // return 401
        debug!("token in invalid: {}", e);
        return Err(ErrorUnauthorized(e));
    }

    // pre-processing
    next.call(req).await
    // post-processing
}
