use serde::{Deserialize, Serialize};

use crate::primitives::FileTrait;

pub trait ModelIoError: From<std::io::Error> {}

pub trait ModelFile: FileTrait {
    type Error: ModelIoError;

    fn model_to_bytes(model: &impl Serialize) -> Result<Vec<u8>, Self::Error>;
    fn self_model_to_bytes(&self, model: &impl Serialize) -> Result<Vec<u8>, Self::Error> {
        Self::model_to_bytes(model)
    }

    fn save_model(&self, model: &impl Serialize) -> Result<(), Self::Error> {
        self.save(&self.self_model_to_bytes(model)?)?;
        Ok(())
    }

    #[cfg(feature = "async")]
    async fn asave_model(&self, model: &impl Serialize) -> Result<(), Self::Error> {
        use crate::file_base::FileTraitAsync;

        self.asave(&self.self_model_to_bytes(model)?).await?;
        Ok(())
    }

    fn bytes_to_model<T: for<'de> Deserialize<'de>>(data: Vec<u8>) -> Result<T, Self::Error>;
    fn self_bytes_to_model<T: for<'de> Deserialize<'de>>(
        &self,
        data: Vec<u8>,
    ) -> Result<T, Self::Error> {
        Self::bytes_to_model(data)
    }

    fn load_model<T: for<'de> Deserialize<'de>>(&self) -> Result<T, Self::Error> {
        self.self_bytes_to_model(self.load()?)
    }

    #[cfg(feature = "async")]
    async fn aload_model<T: for<'de> Deserialize<'de>>(&self) -> Result<T, Self::Error> {
        use crate::primitives::FileTraitAsync;

        self.self_bytes_to_model(self.aload().await?)
    }
}

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
