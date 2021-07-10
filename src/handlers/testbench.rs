use actix_web::{web,HttpResponse,Error,get};

use crate::MongoDB;

use crate::models::get_testbench_by_id;


#[get("/testbench/{id}")]
pub async fn get_testbench(
    mongo:MongoDB,
    path: web::Path<(u32,)>
)->Result<HttpResponse,Error>{
    
    if let Ok(result)=get_testbench_by_id(mongo,path.into_inner().0){
        return Ok(HttpResponse::Ok().json(result));
    }
    
    Ok(HttpResponse::NotFound().finish())
}



