# Filess

Simplify file management with file primitives.

Use filess as you would use String or Vec.

Each file format is now a separate type, if your function needs json, you can put 'filess::Json' as file type, enforcing the proper path.

`Filess` simplifies saving and loading of data, with `serde` and `image` optional integration.

```rust
let file1: Json = Json::new("path");            // Create new Json file. Filess will ensure that it's a valid path
let data: String = file1.load();                // Load data from a file as a String
let model = file1.load_model::<YourModel>();    // `Serde` integration: load model from the file
file1.save(&data);                              // Save anything with `impl AsRef<[u8]>`
file1.save_model(&model);                       // `Serde` integration: save a model into file easily

let file2: Jpeg = Jpeg::new("path");
let image: DynamicImage = file2.load_image();   // `Image` integration: load image of jpeg format from file
file2.save_image(&image);                       // `Image` integration: save `DynamicImage` with default compression parameters
file2.save_image_custom(&image, 80);            // `Image` integration: save `DynamicImage` with custom quality parameters (only available if supports quality settings)
```

## Features

`all-files`: All currently supported files, includes `all-text` and `all-images`
`all-text`: All currently supported text files: `Json`, `Toml`, `Md`, `Txt`
`all-images`: All currently supported image files: `Jpeg`, `Png`, `WebP`, `Avif`, `Tiff`
`serde`: `Serde` integration, adds `save_model` and `load_model` for `Json` and `Toml`
`image`: `Image` integration, adds `save_image` and `load_image` to all image formats, and `save_image_custom` to formats where `image` supports custom quality
`async`: Add async versions of all methods. Uses minimal `tokio` for fs. Adds `save_image_custom_async_offload` to offload image encoding

All files have their separate features. It is recommended to turn off default features and add only formats you use, if you wish to publish.

> [!NOTE]
> In beta, adding new file formats is a bit bothersome
