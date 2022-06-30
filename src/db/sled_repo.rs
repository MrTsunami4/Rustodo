// #![allow(unused)]

// use sled::Db;
// use uuid::Uuid;

// use crate::models::Todo;

// pub fn init_db() -> Db {
//     sled::open("axum_db").expect("Failed to open sled database")
// }

// pub async fn write_todo_db(db: &Db, todo: &Todo) {
//     let (id, content): (&Uuid, &str) = (todo.id(), todo.content());
//     db.insert(id, content).expect("Fto write todo to database");
//     flush_db(db).await;
// }

// pub async fn read_todo_db(db: &Db, id: &Uuid) -> Option<Todo> {
//     let content = db.get(id).expect("Failed to read todo from database");
//     let content = content
//         .as_ref()
//         .map(|content| Todo::new(&ivec_to_string(content)));
//     flush_db(db).await;
//     content
// }

// pub async fn read_all_todo_db(db: &Db) -> Vec<Todo> {
//     let mut todos = Vec::new();

//     for kv_result in db.iter() {
//         let kv = kv_result.expect("Failed to read todo from database");
//         let id = Uuid::from_slice(kv.0.as_ref()).expect("Failed to parse UUID");
//         let content = ivec_to_string(kv.1.as_ref());
//         todos.push(Todo::from_db(id, content));
//     }
//     flush_db(db).await;
//     todos
// }

// pub async fn delete_todo_db(db: &Db, id: &Uuid) -> bool {
//     let status = db.remove(id).expect("Failed to delete todo from database");
//     flush_db(db).await;
//     status.is_some()
// }

// async fn flush_db(db: &Db) {
//     db.flush_async().await.expect("Failed to flush database");
// }

// fn ivec_to_string(ivec: &[u8]) -> String {
//     String::from_utf8(ivec.to_vec()).unwrap()
// }
