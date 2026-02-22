#[cfg(feature = "serde")]
pub use crate::ModelFile;
use crate::define_file;

#[derive(Debug, thiserror::Error)]
pub enum ModelJsonIoError {
    #[cfg(feature = "serde")]
    #[error("Seder Error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Io Error: {0}")]
    Io(#[from] std::io::Error),
}

#[cfg(feature = "serde")]
impl crate::ModelIoError for ModelJsonIoError {}

define_file!(Json, ["json"], b"{}");

#[cfg(feature = "serde")]
impl ModelFile for Json {
    type Error = ModelJsonIoError;
    
    fn bytes_to_model<T: for<'de> serde::Deserialize<'de>>(data: Vec<u8>) -> Result<T, Self::Error> {
        Ok(serde_json::from_slice(&data)?)
    }
    
    fn model_to_bytes(model: &impl serde::Serialize) -> Result<Vec<u8>, Self::Error> {
        Ok(serde_json::to_vec_pretty(model)?)
    }
}

/// =================== TESTS =================== ///
#[cfg(test)]
mod json {
    use std::env::temp_dir;

    use crate::Temporary;

    use super::*;

    #[test]
    fn new_valid_extension() {
        let dir = temp_dir();
        let file_path = dir.join("data1.json");
        // Should not panic
        let _ = Temporary::new(Json::new(&file_path));
    }

    #[test]
    #[should_panic]
    fn new_invalid_extension_panics() {
        let dir = temp_dir();
        let file_path = dir.join("data2.txt");
        let _ = Temporary::new(Json::new(&file_path));
    }

    #[test]
    fn create_file() {
        let dir = temp_dir();
        let file_path = dir.join("test1.json");
        let handler = Temporary::new(Json::new(&file_path));
        
        handler.create().expect("Failed to create file");
        assert!(file_path.exists());
    }

    #[test]
    fn save_and_load() {
        let dir = temp_dir();
        let file_path = dir.join("save_test1.json");
        let handler = Temporary::new(Json::new(&file_path));
        let data = b"{\"key\": \"value\"}";

        handler.save(data).expect("Save failed");
        let loaded = handler.load().expect("Load failed");
        
        assert_eq!(loaded, data);
    }

    #[test]
    fn load_non_existent_initializes() {
        let dir = temp_dir();
        let file_path = dir.join("init_test1.json");
        let handler = Temporary::new(Json::new(&file_path));

        // File doesn't exist yet, load should create it with Json::file_init_bytes()
        let loaded = handler.load().expect("Load failed on new file");
        
        assert_eq!(loaded, Json::file_init_bytes().unwrap());
        assert!(file_path.exists());
    }
}

#[cfg(all(test, feature = "async"))]
mod async_tests {
    use std::env::temp_dir;

    use crate::Temporary;

    use super::*;

    #[tokio::test]
    async fn async_save_load() {
        let dir = temp_dir();
        let file_path = dir.join("async_test.json");
        let handler = Temporary::new(Json::new(&file_path));
        let data = b"async data";

        handler.save_async(&data).await.expect("Async save failed");
        let loaded = handler.load_async().await.expect("Async load failed");

        assert_eq!(loaded, data);
    }
}

#[cfg(test)]
mod json_from {
    use super::*;
    use std::path::{Path, PathBuf};

    #[test]
    fn str() {
        let path = String::from("test.json");
        let _ = Json::new(&path);
    }

    #[test]
    fn string() {
        let path = String::from("test.json");
        let _ = Json::new(path);
    }

    #[test]
    fn path() {
        let path = Path::new("test.json");
        let _ = Json::new(path);
    }

    #[test]
    fn pathbuf() {
        let path = PathBuf::from("test.json");
        let _ = Json::new(path);
    }
}

#[cfg(all(test, feature = "serde"))]
mod json_model {
    use crate::{Temporary, test_assets::{User, get_temp_path}};

    use super::*;

    #[test]
    fn bytes_conversion() {
        let user = User { name: "Alice".into(), age: 30 };
        
        let bytes = Json::model_to_bytes(&user).unwrap();
        let decoded: User = Json::bytes_to_model(bytes).unwrap();
        
        assert_eq!(user, decoded);
    }

    #[test]
    fn save_and_load_model() {
        let path = get_temp_path("usr");
        let handler = Temporary::new(Json::new(&path));
        let model = User { name: "Alice".into(), age: 30 };

        handler.save_model(&model).expect("Save model failed");
        let loaded: User = handler.load_model().expect("Load model failed");
        
        assert_eq!(model, loaded);
    }
}

#[cfg(all(test, feature = "async"))]
mod json_model_async {
    use crate::{Temporary, test_assets::{User, get_temp_path}};

    use super::*;

    #[tokio::test]
    async fn model_lifecycle_async() {
        let path = get_temp_path("async_lifecycle");
        let handler = Temporary::new(Json::new(&path));
        let user = User { name: "Async".into(), age: 10 };

        handler.save_model_async(&user).await.expect("Async save failed");
        let loaded: User = handler.load_model_async().await.expect("Async load failed");
        
        assert_eq!(user, loaded);
        let _ = handler.remove_async().await;
    }
}
