use crate::schemas::users as db_users;
use anyhow::Context;
//use serde::{Deserialize, Serialize};
use std::{
    clone::Clone,
    collections::HashMap,
    fmt::Debug,
    marker::{Send, Sync},
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
};
use thiserror::Error;
//use validator::Validate;

#[derive(Debug, Error)]
enum RepositoryError {
    #[error("NotFound, id is {0}")]
    NotFound(i32),
}

#[allow(dead_code, unused_variables)]
//pub trait UserRepository: Debug + Clone + Send + Sync + 'static {
pub trait UserRepository: Debug + Send + Sync + 'static {
    fn create(&self, payload: db_users::Model) -> db_users::Model;
    fn find(&self, email: &String, password: &String) -> Option<db_users::Model>;
    fn findById(&self, id: i32) -> Option<db_users::Model>;
    fn find_all(&self) -> Vec<db_users::Model>;
    fn update(&self, id: i32, payload: db_users::Model) -> anyhow::Result<db_users::Model>; // FIXME: type of payload must be changed
    fn delete(&self, id: i32) -> anyhow::Result<()>;
}

// #[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
// pub struct User {
//     pub id: i32,
//     pub text: String,
//     pub completed: bool,
// }

// #[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Validate)]
// pub struct CreateUser {
//     #[validate(length(min = 1, max = 100))]
//     text: String,
// }

// #[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Validate)]
// pub struct UpdateUser {
//     #[validate(length(min = 1, max = 100))]
//     text: Option<String>,
//     completed: Option<bool>,
// }
// impl User {
//     pub fn new(id: i32, text: String) -> Self {
//         Self {
//             id,
//             text,
//             completed: false,
//         }
//     }
// }

/*******************************************************************************
 PostgreSQL
*******************************************************************************/
#[derive(Debug, Clone)]
#[allow(dead_code, unused_variables)]
pub struct UserRepositoryForDB {
    store: u16,
}

#[allow(dead_code, unused_variables)]
impl UserRepositoryForDB {
    pub fn new() -> Self {
        todo!()
    }

    fn write_store_ref(&self) -> RwLockWriteGuard<UserDatas> {
        todo!()
    }

    fn read_store_ref(&self) -> RwLockReadGuard<UserDatas> {
        todo!()
    }
}

#[allow(dead_code, unused_variables)]
impl UserRepository for UserRepositoryForDB {
    fn create(&self, payload: db_users::Model) -> db_users::Model {
        todo!()
    }

    fn find(&self, email: &String, password: &String) -> Option<db_users::Model> {
        todo!()
    }

    fn findById(&self, id: i32) -> Option<db_users::Model> {
        todo!()
    }

    fn find_all(&self) -> Vec<db_users::Model> {
        todo!()
    }

    fn update(&self, id: i32, payload: db_users::Model) -> anyhow::Result<db_users::Model> {
        todo!()
    }

    fn delete(&self, id: i32) -> anyhow::Result<()> {
        todo!()
    }
}

/*******************************************************************************
 On memory
*******************************************************************************/
type UserDatas = HashMap<i32, db_users::Model>;

#[derive(Debug, Clone)]
#[allow(dead_code, unused_variables)]
pub struct UserRepositoryForMemory {
    store: Arc<RwLock<UserDatas>>,
}

#[allow(dead_code, unused_variables)]
impl UserRepositoryForMemory {
    pub fn new() -> Self {
        UserRepositoryForMemory {
            store: Arc::default(),
        }
    }

    fn write_store_ref(&self) -> RwLockWriteGuard<UserDatas> {
        self.store.write().unwrap()
    }

    fn read_store_ref(&self) -> RwLockReadGuard<UserDatas> {
        self.store.read().unwrap()
    }
}

#[allow(dead_code, unused_variables)]
impl UserRepository for UserRepositoryForMemory {
    fn create(&self, payload: db_users::Model) -> db_users::Model {
        todo!()
        // let mut store = self.write_store_ref();
        // let id = (store.len() + 1) as i32;
        // let user = db_users::Model::new(id, payload.text.clone());
        // store.insert(id, user.clone());
        // user
    }

    fn find(&self, email: &String, password: &String) -> Option<db_users::Model> {
        todo!()
    }

    fn findById(&self, id: i32) -> Option<db_users::Model> {
        todo!()
        // let store = self.read_store_ref();
        // store.get(&id).cloned()
    }

    fn find_all(&self) -> Vec<db_users::Model> {
        todo!()
        // let store = self.read_store_ref();
        // Vec::from_iter(store.values().cloned())
    }

    fn update(&self, id: i32, payload: db_users::Model) -> anyhow::Result<db_users::Model> {
        todo!()
        // let mut store = self.write_store_ref();
        // let user = store.get(&id).context(RepositoryError::NotFound(id))?;
        // let text = payload.text.unwrap_or(user.text.clone());
        // let completed = payload.completed.unwrap_or(user.completed);
        // let user = User {
        //     id,
        //     text,
        //     completed,
        // };
        // store.insert(id, user.clone());
        // Ok(user)
    }

    fn delete(&self, id: i32) -> anyhow::Result<()> {
        todo!()
        // let mut store = self.write_store_ref();
        // store.remove(&id).ok_or(RepositoryError::NotFound(id))?;
        // Ok(())
    }
}
