use serde::{Deserialize, Serialize};

/// Marker trait for errors that can wrap [std::io::Error] in a model context.
pub trait ModelIoError: From<std::io::Error> + std::error::Error {}

/// Trait for files that serialize/deserialize typed models.
pub trait ModelFile: crate::traits::FileTrait {
    type Error: ModelIoError;

    /// Serializes a model to bytes using the file's format.
    fn model_to_bytes(model: &impl Serialize) -> Result<Vec<u8>, Self::Error>;
    /// Like [Self::model_to_bytes], but allows format-specific dispatch.
    fn self_model_to_bytes(&self, model: &impl Serialize) -> Result<Vec<u8>, Self::Error> {
        Self::model_to_bytes(model)
    }

    /// Serializes and writes a model to the file.
    fn save_model(&self, model: &impl Serialize) -> Result<(), Self::Error> {
        self.save(&self.self_model_to_bytes(model)?)?;
        Ok(())
    }

    /// Deserializes bytes into a model using the file's format.
    fn bytes_to_model<T: for<'de> Deserialize<'de>>(data: Vec<u8>) -> Result<T, Self::Error>;
    /// Like [Self::bytes_to_model], but allows format-specific dispatch.
    fn self_bytes_to_model<T: for<'de> Deserialize<'de>>(
        &self,
        data: Vec<u8>,
    ) -> Result<T, Self::Error> {
        Self::bytes_to_model(data)
    }

    /// Reads the file and deserializes its content into a model.
    fn load_model<T: for<'de> Deserialize<'de>>(&self) -> Result<T, Self::Error> {
        self.self_bytes_to_model(self.load()?)
    }
}

/// Async counterpart of [ModelFile].
#[cfg(all(feature = "async"))]
pub trait AsyncModelFile: ModelFile + crate::traits::AsyncFileTrait {
    /// Async version of [ModelFile::save_model].
    async fn asave_model(&self, model: &impl Serialize) -> Result<(), Self::Error> {
        self.asave(&self.self_model_to_bytes(model)?).await?;
        Ok(())
    }

    /// Async version of [ModelFile::load_model].
    async fn aload_model<T: for<'de> Deserialize<'de>>(&self) -> Result<T, Self::Error> {
        self.self_bytes_to_model(self.aload().await?)
    }
}

#[cfg(all(feature = "async"))]
impl<T: ModelFile + crate::traits::AsyncFileTrait> AsyncModelFile for T {}

// #[macro_export]
// macro_rules! define_model_file {
//     ($name:ident, $error:ident) => {
//         #[cfg(feature = "serde")]
//         const _: () = {
//             impl ModelFile for $name {
//                 type Error = $error;

//                 fn bytes_to_model<T: for<'de> serde::Deserialize<'de>>(data: Vec<u8>) -> Result<T, Self::Error> {
//                     Ok(serde_json::from_slice(&data)?)
//                 }

//                 fn model_to_bytes(model: &impl serde::Serialize) -> Result<Vec<u8>, Self::Error> {
//                     Ok(serde_json::to_vec_pretty(model)?)
//                 }
//             }
//         };
//     };
// }
