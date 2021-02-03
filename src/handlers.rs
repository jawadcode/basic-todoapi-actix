use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};

use crate::structs::*;

#[get("/")]
pub async fn hello() -> impl Responder {
    "greetings earthling"
}

#[post("/todos")]
pub async fn post_todo(todo: web::Json<Todo>, store: web::Data<Store>) -> impl Responder {
    let mut write = store.todos.write().unwrap();
    write.push(todo.into_inner());

    HttpResponse::Created().body(format!("Created todo #{}", write.len()))
}

#[get("/todos/{id}")]
pub async fn get_todo(todo_id: web::Path<usize>, store: web::Data<Store>) -> impl Responder {
    let todos = store.todos.read().unwrap().clone();
    let id = todo_id.into_inner();
    let todo = todos.get(id);

    if let None = todo {
        return HttpResponse::NotFound().body(format!("Todo #{} not found", id));
    }

    HttpResponse::Ok().json(todo)
}

#[get("/todos")]
pub async fn get_todos(store: web::Data<Store>) -> impl Responder {
    let read = store.todos.read().unwrap();

    web::Json(read.clone())
}

#[patch("/todos/{id}")]
pub async fn patch_todos(
    new_todo: web::Json<Todo>,
    todo_id: web::Path<usize>,
    store: web::Data<Store>,
) -> impl Responder {
    let mut write = store.todos.write().unwrap();
    let id = todo_id.into_inner();
    let todo = write.get_mut(id);

    if let None = todo {
        return HttpResponse::NotFound().body(format!("Todo #{} not found", id));
    }

    *todo.unwrap() = new_todo.into_inner();
    HttpResponse::Ok().body(format!("Updated todo #{}", id))
}

#[patch("/todos/{id}/toggle")]
pub async fn toggle_todo(todo_id: web::Path<usize>, store: web::Data<Store>) -> impl Responder {
    let mut write = store.todos.write().unwrap();
    let id = todo_id.into_inner();
    let todo = write.get_mut(id);

    if let None = todo {
        return HttpResponse::NotFound().body(format!("Todo #{} not found", id));
    }
    let todo = todo.unwrap();
    todo.completed = !todo.completed;
    HttpResponse::Ok().json(todo)
}

#[delete("/todos/{id}")]
pub async fn delete_todo(todo_id: web::Path<usize>, store: web::Data<Store>) -> impl Responder {
    let mut write = store.todos.write().unwrap();
    let id = todo_id.into_inner();
    let todo = write.get(id);

    if let None = todo {
        return HttpResponse::NotFound().body(format!("Todo #{} not found", id));
    }

    write.remove(id);
    HttpResponse::Ok().body(format!("Deleted todo #{}", id))
}

#[get("/todos/filter/{filter}")]
pub async fn filter_todos(filter: web::Path<String>, store: web::Data<Store>) -> impl Responder {
    let todos = store.todos.read().unwrap().clone();

    match filter.into_inner().as_str() {
        "completed" => {
            HttpResponse::Ok().json(todos.into_iter().filter(|t| t.completed).collect::<Todos>())
        }
        "incomplete" => HttpResponse::Ok().json(
            todos
                .into_iter()
                .filter(|t| !t.completed)
                .collect::<Todos>(),
        ),
        _ => HttpResponse::BadRequest().body(format!("Invalid filter")),
    }
}

#[get("/todos/search")]
pub async fn search_todos(
    query_params: web::Query<QueryParams>,
    store: web::Data<Store>,
) -> impl Responder {
    let params = query_params.into_inner();
    if let None = params.query {
        return HttpResponse::BadRequest().body("Missing query parameter");
    }

    let query = params.query.unwrap().to_lowercase();
    let filter = params.filter.unwrap_or(String::from(""));
    let limit = params.limit.unwrap_or(std::usize::MAX);

    let todos = store.todos.read().unwrap().clone();

    HttpResponse::Ok().json(
        todos
            .into_iter()
            .filter(|t| {
                (t.title.to_lowercase().contains(&query)
                    || t.description.to_lowercase().contains(&query))
                    && match filter.as_str() {
                        "completed" => t.completed,
                        "incomplete" => !t.completed,
                        _ => true,
                    }
            })
            .take(limit)
            .collect::<Todos>(),
    )
}
