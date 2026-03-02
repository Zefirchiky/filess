use serde::{Deserialize, Serialize};

use crate::FileTrait;

pub trait ModelIoError: From<std::io::Error> {}

pub trait ModelFile: FileTrait {
    type Error: ModelIoError;

    fn model_to_bytes(model: &impl Serialize) -> Result<Vec<u8>, Self::Error>;
    fn save_model(&self, model: &impl Serialize) -> Result<(), Self::Error> {
        self.save(&Self::model_to_bytes(model)?)?;
        Ok(())
    }

    #[cfg(feature = "async")]
    async fn save_model_async(&self, model: &impl Serialize) -> Result<(), Self::Error> {
        self.save_async(&Self::model_to_bytes(model)?).await?;
        Ok(())
    }

    fn bytes_to_model<T: for<'de> Deserialize<'de>>(data: Vec<u8>) -> Result<T, Self::Error>;
    fn load_model<T: for<'de> Deserialize<'de>>(&self) -> Result<T, Self::Error> {
        Self::bytes_to_model(self.load()?)
    }

    #[cfg(feature = "async")]
    async fn load_model_async<T: for<'de> Deserialize<'de>>(&self) -> Result<T, Self::Error> {
        Self::bytes_to_model(self.load_async().await?)
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
