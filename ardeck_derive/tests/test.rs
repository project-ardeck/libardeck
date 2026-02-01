use ardeck::{
    config::ConfigFile,
    store::{StoreBuilder, StoreTrait},
};
use ardeck_derive::Store;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Store)]
struct MyConfig {
    name: String,
    age: u32,
}

impl Default for MyConfig {
    fn default() -> Self {
        Self {
            name: "John Doe".into(),
            age: 42,
        }
    }
}
impl ConfigFile for MyConfig {
    fn name() -> &'static str {
        "my_config.json"
    }
}

#[test]
fn derive() {
    #[test]
    fn store_builder() {
        StoreBuilder::default().path("./".into()).init();

        let mut my_config = MyConfig::load().unwrap_or_default();
        my_config.age += 1;
        my_config.save().unwrap();
    }
}
