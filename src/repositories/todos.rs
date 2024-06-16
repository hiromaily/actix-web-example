use anyhow::Context;
use sea_orm;
use serde::{Deserialize, Serialize};
use std::{
    clone::Clone,
    collections::HashMap,
    fmt::Debug,
    marker::{Send, Sync},
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
};
use thiserror::Error;
use validator::Validate;

#[derive(Debug, Error)]
enum RepositoryError {
    #[error("NotFound, id is {0}")]
    NotFound(i32),
}

#[allow(dead_code, unused_variables)]
//pub trait TodoRepository: Debug + Clone + Send + Sync + 'static {
pub trait TodoRepository: Debug + Send + Sync + 'static {
    fn create(&self, payload: CreateTodo) -> Todo;
    fn find(&self, id: i32) -> Option<Todo>;
    fn find_all(&self) -> Vec<Todo>;
    fn update(&self, id: i32, payload: UpdateTodo) -> anyhow::Result<Todo>;
    fn delete(&self, id: i32) -> anyhow::Result<()>;
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Todo {
    pub id: i32,
    pub text: String,
    pub completed: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Validate)]
pub struct CreateTodo {
    #[validate(length(min = 1, max = 100))]
    text: String,
}

// #[cfg(test)]
// impl CreateTodo {
//     pub fn new(text: String) -> Self {
//         Self { text }
//     }
// }

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Validate)]
pub struct UpdateTodo {
    #[validate(length(min = 1, max = 100))]
    text: Option<String>,
    completed: Option<bool>,
}

impl Todo {
    pub fn new(id: i32, text: String) -> Self {
        Self {
            id,
            text,
            completed: false,
        }
    }
}

/*******************************************************************************
 PostgreSQL
*******************************************************************************/
#[derive(Debug, Clone)]
#[allow(dead_code, unused_variables)]
pub struct TodoRepositoryForDB {
    conn: sea_orm::DatabaseConnection,
    store: u16,
}

#[allow(dead_code, unused_variables)]
impl TodoRepositoryForDB {
    pub fn new(conn: sea_orm::DatabaseConnection) -> Self {
        Self { conn, store: 0 }
    }

    fn write_store_ref(&self) -> RwLockWriteGuard<TodoDatas> {
        todo!()
    }

    fn read_store_ref(&self) -> RwLockReadGuard<TodoDatas> {
        todo!()
    }
}

#[allow(dead_code, unused_variables)]
impl TodoRepository for TodoRepositoryForDB {
    fn create(&self, payload: CreateTodo) -> Todo {
        todo!()
    }

    fn find(&self, id: i32) -> Option<Todo> {
        todo!()
    }

    fn find_all(&self) -> Vec<Todo> {
        todo!()
    }

    fn update(&self, id: i32, payload: UpdateTodo) -> anyhow::Result<Todo> {
        todo!()
    }

    fn delete(&self, id: i32) -> anyhow::Result<()> {
        todo!()
    }
}

/*******************************************************************************
 On memory
*******************************************************************************/
type TodoDatas = HashMap<i32, Todo>;

#[derive(Debug, Clone)]
#[allow(dead_code, unused_variables)]
pub struct TodoRepositoryForMemory {
    store: Arc<RwLock<TodoDatas>>,
}

#[allow(dead_code, unused_variables)]
impl TodoRepositoryForMemory {
    pub fn new() -> Self {
        TodoRepositoryForMemory {
            store: Arc::default(),
        }
    }

    fn write_store_ref(&self) -> RwLockWriteGuard<TodoDatas> {
        self.store.write().unwrap()
    }

    fn read_store_ref(&self) -> RwLockReadGuard<TodoDatas> {
        self.store.read().unwrap()
    }
}

#[allow(dead_code, unused_variables)]
impl TodoRepository for TodoRepositoryForMemory {
    fn create(&self, payload: CreateTodo) -> Todo {
        let mut store = self.write_store_ref();
        let id = (store.len() + 1) as i32;
        let todo = Todo::new(id, payload.text.clone());
        store.insert(id, todo.clone());
        todo
    }

    fn find(&self, id: i32) -> Option<Todo> {
        let store = self.read_store_ref();
        //store.get(&id).map(|todo| todo.clone())
        store.get(&id).cloned()
    }

    fn find_all(&self) -> Vec<Todo> {
        let store = self.read_store_ref();
        //Vec::from_iter(store.values().map(|todo| todo.clone()))
        Vec::from_iter(store.values().cloned())
    }

    fn update(&self, id: i32, payload: UpdateTodo) -> anyhow::Result<Todo> {
        let mut store = self.write_store_ref();
        let todo = store.get(&id).context(RepositoryError::NotFound(id))?;
        let text = payload.text.unwrap_or(todo.text.clone());
        let completed = payload.completed.unwrap_or(todo.completed);
        let todo = Todo {
            id,
            text,
            completed,
        };
        store.insert(id, todo.clone());
        Ok(todo)
    }

    fn delete(&self, id: i32) -> anyhow::Result<()> {
        let mut store = self.write_store_ref();
        store.remove(&id).ok_or(RepositoryError::NotFound(id))?;
        Ok(())
    }
}
