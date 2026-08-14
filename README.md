# Filess

Simplify file management with file primitives.

Use `filess` as you would use `String` or `Vec`.

Each file format is now a separate type, if your function needs json, you can put `filess::Json` as a type, enforcing the proper path.

`Filess` simplifies saving and loading of data, with `serde`, `image` and `symphonia` optional integration.

```rust
let file1: Json = Json::new("path/to/file.json");   // Create new Json file. Filess will ensure that it's a valid path
let data: Vec<u8> = file1.load()?;                  // Load data from a file
let model = file1.load_model::<YourModel>()?;       // `Serde` integration: load model from the file
let model: YourModel = file1.load_model()?;         // Or like this
file1.save(&data)?;                                 // Save anything with `impl AsRef<[u8]>`
file1.save_model(&model)?;                          // `Serde` integration: save a model into file

let file2: Jpeg = Temporary::new(Jpeg::new("path/to/image.jpg")); // Temporary file will be deleted together with it's empty parent dirs at `drop()`
let image: DynamicImage = file2.load_image()?;      // `Image` integration: load image of jpeg format from file
file2.save_image(&image)?;                          // `Image` integration: save `DynamicImage` with default compression parameters
// `Image` integration: save `DynamicImage` with custom quality parameters (only available if supports quality settings)
file2.save_image_custom(&image, JpegConfig { quality: 40 })?;

let file3 = Ogg::new("path/to/audio.opus");         // `Symphonia` integration
let audio: DecodedStream<OggReader, DynamicDecoder> // DecodedStream gives you everything you need for use with `symphonia` and is served as source for `rodio`
    = file3.load_audio()?;                          // Saving is not support (yet). You can still use `file3.save(&data)` if you have compressed audio

let some_image = ImageType::new("path/to/image.webp"); // `File`, `Text`, `Model`, `Image` and `Audio` Types abstract away exact file types without overhead ov `Box<dyn>`
let img = some_image.load_image()?;
some_image.save_image(&img)?;

// `Dirs` integration: access user directories
let config_dir = Dir::config().unwrap();           // User's config directory
let cache_dir = DirWith::<Json>::cache().unwrap(); // User's cache directory with Json files

// Each function have their async variants (prefixed with `a`) if `async` feature is on
let image2 = file2.aload_image().await?;
Jpeg::new("another/path/image.jpeg").asave_image(&image2).await?;

// Enforce that file given should be able to save/load `serde` models. `ModelType` can also be passed
fn needs_model(file: impl filess::traits::ModelFile);
// OR
fn needs_model(file: filess::ModelType);
```

## Features

| Feature      | Description
|--------------|------------
| `base`       | (Default) Includes basic small features, like `rayon`, `open` and `infer`
| `all-files`  | (Default) All currently supported files, includes `all-text`, `all-images` and `all-audio`
| `all-text`   | (Default) All currently supported text files: `all-models`, `Md`, `Txt`
| `all-models` | (Default) All currently supported model files: `Json`, `Toml`, `Ron` (this can be heavy on compilation, as it also activates `serde` and specific serde file support)
| `just-all-models` | All currently supported model files, without serde: `Json`, `Toml`, `Ron` (features `just_*`)
| `all-images` | (Default) All currently supported image files: `Jpeg`, `Png`, `WebP`, `Avif`, `Tiff`, `Gif`, `Bmp`, `Exr`, `Ff`, `Hdr`, `Ico`, `Pnm`, `Qoi`, `Tga`
| `all-audio`  | (Default) All currently supported audio files: `Ogg`, `Mkv`, `Wav`, `Flac`, `Mp4`, `Mp3`, `Mp2`, `Mp1`, `Mpa`, `Alac`
| `serde`      | (Default) `serde` integration, adds `save_model()` and `load_model()` for `all-models`.
| `image`      | `image` integration, adds `save_image()` and `load_image()` to all image formats, and `save_image_custom()` to formats where `image` supports custom quality
| `image-nasm` | Turns on `nasm` feature of `image`
| `audio`      | `symphonia` integration, adds `load_audio()` to all audio formats. Due to audio being complicated, `DecodedStream` is returned, which contains reader, decoder, track_id and helper methods
| `symphonia-simd`| Turns on `opt-simd` feature of `symphonia`
| `rodio`      | Adds minimal rodio integration, allowing `DecodedStream` to be directly passed as source
| `async`      | Add async versions of all methods. Uses minimal `tokio` for fs. Adds `asave_image_custom_offload()` to offload image encoding
| `rayon`      | (Default) Turns on all of `rayon` features in crates that support it
| `open`       | (Default) `open` integration. Open files or directories in default or arbitrary programs (`open()`, `open_with()`, etc.)
| `walk`       | (Default) `walkdir` integration. Walk through directory (`walk()`)
| `glob`       | (Default) `glob` integration. Use glob pattern to find files in `Dir`. Also converts paths into `F` (`DirWith<F>::glob()`, `DirWith<F>::glob_with()`)
| `infer`      | (Default) `infer` integration. Enforces that file data is of proper format, especially useful for images, audio, etc. (`enforce()`, `aenforce()`)
| `trash`      | (Default) `trash` integration. Move files/dirs to trash, aka trash them (`trash()`)
| `dirs`       | (Default) `dirs` integration. Access user directories like home, config, cache, etc. (`Dir::home()`, `Dir::config()`, `DirWith::cache()`, etc.) |

Also exposes `open`, `walk`, `glob`, `infer`, `trash` and `dirs`.

All files have their separate features. It is recommended to turn off default features and add only formats you use, if you wish to publish.

Adding `json`, `toml` or `ron` without serde support doesn't really make sense, as they are used specifically for it. But activating all of their features is not good either, as it will add serde and additional crate for each format, which will increase binary and compilation speed.
I tried doing conditional feature, so only if both `serde` and `json` are active, `serde_json` will be added, but unfortunately cargo doesn't support that yet.
Current approach is to just activate both `serde` and `serde_json` when `json` feature is active, but this will hopefully change.
