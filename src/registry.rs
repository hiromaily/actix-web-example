// use crate::repositories::todo_repository;
// use crate::state;
use crate::toml;

#[allow(dead_code)]
pub struct Registry {
    pub conf: toml::Config,
    //pub app_repo: Box<dyn todo_repository::TodoRepository>,
}

#[allow(dead_code)]
impl Registry {
    pub fn new(conf: toml::Config) -> Self {
        Self { conf }
    }

    // Refer to `dynを利用してトレイトを返す`
    // https://doc.rust-jp.rs/rust-by-example-ja/trait/dyn.html

    // `todo_repository::TodoRepository`` traitがcloneを継承するとobject-safetyが失われるため、エラーが出る
    // #[allow(dead_code)]
    // fn new_repository(&self) -> Box<dyn todo_repository::TodoRepository> {
    //     if self.conf.pg.enabled {
    //         return Box::new(todo_repository::TodoRepositoryForDB::new());
    //     } else {
    //         return Box::new(todo_repository::TodoRepositoryForMemory::new());
    //     }
    // }

    // pub fn create_server_data(&self) -> state::AppState {
    //     state::AppState {
    //         app_name: String::from("Actix Web"),
    //         repository: self.new_repository(),
    //     }
    // }
}
