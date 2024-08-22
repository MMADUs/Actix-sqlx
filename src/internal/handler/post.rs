use actix_web::{web, HttpResponse, Responder, Error};
use actix_web::error::{ErrorInternalServerError};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DeleteResult};
use sea_orm::{EntityTrait, Set};
use validator::{Validate};

use crate::internal::model::post::{PostRequest};
use crate::internal::entity::post::{ActiveModel as PostActiveModel, Entity as PostEntity, Model as PostModel, Model};

pub async fn create_post(
    db: web::Data<DatabaseConnection>,
    post: web::Json<PostRequest>,
) -> Result<impl Responder, Error> {
    if let Err(errors) = post.validate() {
        return Ok(HttpResponse::BadRequest().json(errors));
    }

    let new_post = PostActiveModel {
        title: Set(post.title.to_owned()),
        text: Set(post.text.to_owned()),
        ..Default::default()
    }
        .insert(db.get_ref())
        .await;

    match new_post {
        Ok(post) => Ok(HttpResponse::Created().json(post)),
        Err(err) => Err(ErrorInternalServerError(err)),
    }
}

pub async fn get_posts(
    db: web::Data<DatabaseConnection>,
) -> impl Responder {
    let posts_result = PostEntity::find()
        .all(db.get_ref())
        .await;

    match posts_result {
        Ok(posts) => Ok(HttpResponse::Ok().json(posts)),
        Err(err) => Err(ErrorInternalServerError(err)),
    }
}

pub async fn get_post(
    db: web::Data<DatabaseConnection>,
    id: web::Path<i32>,
) -> impl Responder {
    let post_result = PostEntity::find_by_id(id.into_inner())
        .one(db.get_ref())
        .await;

    match post_result {
        Ok(Some(post)) => HttpResponse::Ok().json(post),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn update_post(
    db: web::Data<DatabaseConnection>,
    id: web::Path<i32>,
    post: web::Json<PostRequest>,
) -> impl Responder {
    let post_result = PostEntity::find_by_id(id.into_inner())
        .one(db.get_ref())
        .await;

    match post_result {
        Ok(Some(existing_post)) => {
            let mut new_post: PostActiveModel = existing_post.into();
            new_post.title = Set(post.title.to_owned());
            new_post.text = Set(post.text.to_owned());

            let updated_post = new_post
                .update(db.get_ref())
                .await;

            match updated_post {
                Ok(post) => HttpResponse::Ok().json(post),
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }
        },
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn delete_post(
    db: web::Data<DatabaseConnection>,
    id: web::Path<i32>,
) -> impl Responder {
    let delete_result = PostEntity::delete_by_id(id.into_inner())
        .exec(db.get_ref())
        .await;

    match delete_result {
        Ok(DeleteResult { rows_affected: 1 }) => HttpResponse::Ok().finish(),
        Ok(_) => HttpResponse::NotFound().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}