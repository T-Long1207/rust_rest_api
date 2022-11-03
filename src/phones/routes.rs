use crate::error_handler::CustomError;
use actix_web::{delete, get, post, put, web, HttpResponse};
use crate::phones::{PhoneReport, PhoneReportImpl, PhoneSearch, PhoneSearchImpl, PhoneComment, PhoneCommentImpl};
use serde_json::json;


#[get("/phoneReports")]
async fn phone_reports_find_all() -> Result<HttpResponse, CustomError> {
    let phone = PhoneReportImpl::find_all()?;
    Ok(HttpResponse::Ok().json(phone))
}

#[get("/phoneReports/{id}")]
async fn phone_reports_find(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let phone = PhoneReportImpl::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(phone))
}

#[post("/phoneReports")]
async fn phone_reports_create(phone_input: web::Json<PhoneReport>) -> Result<HttpResponse, CustomError> {
    let phone = PhoneReportImpl::create(phone_input.into_inner())?;
    Ok(HttpResponse::Ok().json(phone))
}

#[put("/phoneReports/{id}")]
async fn phone_reports_update(
    id: web::Path<i32>,
    phone_input: web::Json<PhoneReport>,
) -> Result<HttpResponse, CustomError> {
    let phone = PhoneReportImpl::update(id.into_inner(), phone_input.into_inner())?;
    Ok(HttpResponse::Ok().json(phone))
}

#[delete("/phoneReports/{id}")]
async fn phone_reports_delete(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_phone = PhoneReportImpl::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_phone })))
}


#[get("/phoneSearch")]
async fn phone_search_find_all() -> Result<HttpResponse, CustomError> {
    let phone = PhoneSearchImpl::find_all()?;
    Ok(HttpResponse::Ok().json(phone))
}

#[get("/phoneSearch/{id}")]
async fn phone_search_find(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let phone = PhoneSearchImpl::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(phone))
}

#[post("/phoneSearch")]
async fn phone_search_create(phone_input: web::Json<PhoneSearch>) -> Result<HttpResponse, CustomError> {
    let phone = PhoneSearchImpl::create(phone_input.into_inner())?;
    Ok(HttpResponse::Ok().json(phone))
}

#[put("/phoneSearch/{id}")]
async fn phone_search_update(
    id: web::Path<i32>,
    phone_input: web::Json<PhoneSearch>,
) -> Result<HttpResponse, CustomError> {
    let phone = PhoneSearchImpl::update(id.into_inner(), phone_input.into_inner())?;
    Ok(HttpResponse::Ok().json(phone))
}

#[delete("/phoneSearch/{id}")]
async fn phone_search_delete(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_phone = PhoneSearchImpl::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_phone })))
}


#[get("/phoneComment")]
async fn phone_comment_find_all() -> Result<HttpResponse, CustomError> {
    let phone = PhoneCommentImpl::find_all()?;
    Ok(HttpResponse::Ok().json(phone))
}

#[get("/phoneSearch/{id}")]
async fn phone_comment_find(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let phone = PhoneCommentImpl::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(phone))
}

#[post("/phoneSearch")]
async fn phone_comment_create(phone_input: web::Json<PhoneComment>) -> Result<HttpResponse, CustomError> {
    let phone = PhoneCommentImpl::create(phone_input.into_inner())?;
    Ok(HttpResponse::Ok().json(phone))
}

#[put("/phoneSearch/{id}")]
async fn phone_comment_update(
    id: web::Path<i32>,
    phone_input: web::Json<PhoneComment>,
) -> Result<HttpResponse, CustomError> {
    let phone = PhoneCommentImpl::update(id.into_inner(), phone_input.into_inner())?;
    Ok(HttpResponse::Ok().json(phone))
}

#[delete("/phoneSearch/{id}")]
async fn phone_comment_delete(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_phone = PhoneCommentImpl::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_phone })))
}

pub fn init_routes(comfig: &mut web::ServiceConfig) {
    comfig.service(phone_reports_find_all);
    comfig.service(phone_reports_find);
    comfig.service(phone_reports_create);
    comfig.service(phone_reports_update);
    comfig.service(phone_reports_delete);

    comfig.service(phone_search_find_all);
    comfig.service(phone_search_find);
    comfig.service(phone_search_create);
    comfig.service(phone_search_update);
    comfig.service(phone_search_delete);

    comfig.service(phone_comment_find_all);
    comfig.service(phone_comment_find);
    comfig.service(phone_comment_create);
    comfig.service(phone_comment_update);
    comfig.service(phone_comment_delete);
}
