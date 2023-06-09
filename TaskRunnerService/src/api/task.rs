use actix_web::{
    get,
    post,
    put,
    error::ResponseError,
    web::Path,
    web::Json,
    web::Data,
    HttpResponse,
    http::{header::ContentType, StatusCode}
};

use serde::{ Serialize, Deserialize};
use derive_more::{Display};

#[derive(Serialize, Deserialize)]
pub struct TaskIdentifier{
    task_global_id: String
}

#[get("/task/{task_global_id}")]
pub async fn get_task(task_identifier: Path<TaskIdentifier>) -> Json<String>{
    Json("Hello World!").to_string();
}