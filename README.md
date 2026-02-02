# Filess

Simplify file management with file primitives.

Use filess as you would use String or Vec.

Each file format is now a separate type, if your function needs json, you can put 'filess::Json' as file type, enforcing the proper path.

Filess simplifies saving and loading of data, with `serde` and `image` optional integration.

```rust
let file1: Json = Json::new("path");            // Create new Json file. Filess will ensure that it's a valid path
let data: String = file1.load();                // Load data from a file as a String
let model = file1.load_model::\<YourModel\>();  // `Serde` integration: load model from the file
file1.save(&data);                              // Save anything with `impl AsRef<[u8]>`
file1.save_model(&model);                       // `Serde` integration: save a model into file easily

let file2: Jpeg = Jpeg::new("path");
let image: DynamicImage = file2.load_image();   // `Image` integration: load image of jpeg format from file
file2.save_image(&image);                       // `Image` integration: save `DynamicImage` with default compression parameters
file2.save_image_custom(&image, 80);            // `Image` integration: save `DynamicImage` with custom quality parameters (only available if supports quality settings)
```

> [!WARN]
> In beta, adding new file formats is a bit bothersome
