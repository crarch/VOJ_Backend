pub mod user;
pub mod user_id;
pub mod pass;

pub use pass::*;
pub use user_id::UserId;

use serde::{Deserialize,Serialize};
use mongodb::bson::doc;

use crate::MongoDB;


#[derive(Debug,Serialize,Deserialize)]
pub struct Pass{
    pass:Vec<i32>,
}




pub fn get_question_by_id(mongo:MongoDB,question_id:u32)->Result<Document,()>{
    let collection=mongo.collection::<Document>("questions");

    if let Ok(cursor)=collection.find_one(
        doc!{"_id":question_id},
        mongodb::options::FindOneOptions::builder()
            .projection(Some(doc!{"_id":0,"update":1}))
            .build()
    ){
        if let Some(result)=cursor{
            return Ok(result);
        }
    }
        
    Err(())
    
}

pub fn get_question_update_by_id(mongo:MongoDB,question_id:u32)->Result<u32,()>{
    let collection=mongo.collection::<Document>("questions");

    if let Ok(cursor)=collection.find_one(doc!{"_id":question_id},None){
        if let Some(result)=cursor{
            if let Ok(result)=result.get_i32("update"){
                return Ok(result as u32);
            }
        }
    }
        
    Err(())
    
}

use crate::utils::time::get_unix_timestamp;

pub fn create_new_record(mongo:MongoDB,user_id:u32,question_id:u32,code:&str)->Result<Document,()>{
    
    let collection=mongo.collection::<Document>("records");
    
    let doc=doc!{
        "user_id":user_id,
        "question_id":question_id,
        "submit_time":get_unix_timestamp(),
        "code":code
    };
    
    let result=collection.insert_one(doc,None);
    
    match result{
        Ok(insert_result)=>{
            let object_id=insert_result.inserted_id
                .as_object_id()
                .unwrap()
                .to_hex();
            
            let result=doc!{
                "$oid":object_id,
            };
            
            Ok(result)
        },
        Err(_)=>Err(()),
    }
    
}

pub fn queue_add_job(
    mongo:MongoDB,
    object_id:&str,
    question_id:u32,
    update:u32,
    code:&str
)->Result<(),()>{
    let collection=mongo.collection::<Document>("queue");
    
    let doc=doc!{
        "_id":ObjectId::parse_str(object_id).unwrap(),
        "question_id":question_id,
        "update":update,
        "submit_time":get_unix_timestamp(),
        "code":code
    };
    
    let result=collection.insert_one(doc,None);
    
    match result{
        Ok(_)=>{
            Ok(())
        },
        Err(_)=>Err(()),
    }
}

// pub fn update_judge_result();

use bson::oid::ObjectId;


//only owner can access record
pub fn get_record_by_object_id(mongo:MongoDB,object_id:&str,user_id:u32)->Result<Document,()>{
    
    let collection=mongo.collection::<Document>("records");
    
    if let Ok(object_id)=ObjectId::parse_str(object_id){
        if let Ok(result)=collection.find_one(
            doc!{"_id":object_id,"user_id":user_id},
            None
        ){
            if let Some(result)=result{
                return Ok(result);
            }
        }
    }    
    
    Err(())
}

use bson::document::Document;


#[derive(Debug,Serialize,Deserialize)]
pub struct CodeJson{
    pub question_id:u32,
    pub code:String
}
