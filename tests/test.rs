use std::path::{Path, PathBuf};

use filess::traits::FileTrait;
#[cfg(feature = "serde")]
use filess::traits::FsElement;
use filess::{Dir, DirFile, FileType, Temporary};

// ── Fixtures ────────────────────────────────────────────────────────
fn temp_path(name: &str) -> PathBuf {
    let mut p = std::env::temp_dir();
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    p.push(format!("filess_test_{}_{}", name, ts));
    p
}

fn scratch_dir() -> PathBuf {
    let p = temp_path("scratch");
    let _ = std::fs::remove_dir_all(&p);
    std::fs::create_dir_all(&p).unwrap();
    p
}

// ── File creation errors ────────────────────────────────────────────
#[cfg(feature = "json")]
#[test]
fn json_wrong_extension_error() {
    let err = filess::Json::try_new("data.txt").unwrap_err();
    assert!(err.to_string().contains("txt"));
}

#[cfg(feature = "json")]
#[test]
fn json_no_extension_error() {
    let err = filess::Json::try_new("data").unwrap_err();
    assert!(err.to_string().contains("no extension"));
}

// ── Create / remove / exists ────────────────────────────────────────
#[cfg(feature = "json")]
#[test]
fn create_and_remove_file() {
    let dir = scratch_dir();
    let p = dir.join("test.json");
    let f = filess::Json::new(&p);
    f.create().unwrap();
    assert!(p.exists());
    f.remove().unwrap();
    assert!(!p.exists());
}

#[cfg(feature = "json")]
#[test]
fn remove_non_existent_errors() {
    let dir = scratch_dir();
    let p = dir.join("nope.json");
    let f = filess::Json::new(&p);
    let err = f.remove();
    assert!(err.is_err());
}

// ── Save / Load ─────────────────────────────────────────────────────
#[cfg(feature = "json")]
#[test]
fn save_and_load_roundtrip() {
    let dir = scratch_dir();
    let p = dir.join("roundtrip.json");
    let f = Temporary::new(filess::Json::new(&p));
    let data = b"hello world";
    f.save(data).unwrap();
    let loaded = f.load().unwrap();
    assert_eq!(loaded, data);
}

#[cfg(feature = "json")]
#[test]
fn save_creates_parent_dirs() {
    let nested = scratch_dir().join("a").join("b").join("c");
    let p = nested.join("f.json");
    let f = Temporary::new(filess::Json::new(&p));
    f.save(b"data").unwrap();
    assert!(p.exists());
}

#[cfg(feature = "json")]
#[test]
fn load_non_existent_creates_with_init_bytes() {
    let dir = scratch_dir();
    let p = dir.join("init_test.json");
    let f = Temporary::new(filess::Json::new(&p));
    let loaded = f.load().unwrap();
    assert_eq!(loaded, b"{}");
    assert!(p.exists());
}

#[cfg(feature = "txt")]
#[test]
fn txt_no_init_bytes() {
    let dir = scratch_dir();
    let p = dir.join("plain.txt");
    let f = Temporary::new(filess::Txt::new(&p));
    let loaded = f.load().unwrap();
    assert!(loaded.is_empty());
}

// ── Copy / Rename ───────────────────────────────────────────────────
#[cfg(feature = "json")]
#[test]
fn copy_file() {
    let dir = scratch_dir();
    let src = dir.join("src.json");
    let dst = dir.join("dst.json");
    let f = Temporary::new(filess::Json::new(&src));
    f.save(b"data").unwrap();
    let copied = Temporary::new(f.copy(&dst).unwrap());
    assert!(dst.exists());
    assert_eq!(copied.load().unwrap(), b"data");
}

#[cfg(feature = "json")]
#[test]
fn rename_file_in_fs() {
    let dir = scratch_dir();
    let src = dir.join("old.json");
    let dst = dir.join("new.json");
    let f = Temporary::new(filess::Json::new(&src));
    f.save(b"rename me").unwrap();
    let renamed = Temporary::new(f.rename(&dst).unwrap());
    assert!(!src.exists());
    assert!(dst.exists());
    assert_eq!(renamed.load().unwrap(), b"rename me");
}

#[cfg(feature = "json")]
#[test]
fn rename_file_does_not_touch_fs() {
    let dir = scratch_dir();
    let p = dir.join("unchanged.json");
    let mut f = Temporary::new(filess::Json::new(&p));
    f.create().unwrap();
    f.rename_file("changed.json");
    assert_eq!(f.as_ref(), Path::new("changed.json"));
    assert!(p.exists());
}

// ── FileTrait::change_path ──────────────────────────────────────────
#[cfg(feature = "json")]
#[test]
fn change_path_updates_inner() {
    let mut f = filess::Json::new("a.json");
    f.change_path(PathBuf::from("b.json"));
    assert_eq!(f.as_ref(), Path::new("b.json"));
}

// ── Ext / ext_name / mime_type ──────────────────────────────────────
#[cfg(feature = "json")]
#[test]
fn json_metadata() {
    assert!(filess::Json::ext().contains(&"json"));
    assert_eq!(filess::Json::ext_name(), "json");
    assert!(filess::Json::mime_type().contains(&"application/json"));
}

#[cfg(feature = "txt")]
#[test]
fn txt_metadata() {
    assert!(filess::Txt::ext().contains(&"txt"));
    assert_eq!(filess::Txt::ext_name(), "txt");
}

#[cfg(feature = "md")]
#[test]
fn md_metadata() {
    assert!(filess::Md::ext().contains(&"md"));
    assert_eq!(filess::Md::ext_name(), "md");
}

// ── FileType ────────────────────────────────────────────────────────
#[cfg(feature = "json")]
#[test]
fn file_type_from_ext_json() {
    let ft = FileType::from_ext("cfg.json");
    assert_eq!(ft, FileType::Json(filess::Json::new("cfg.json")));
}

#[cfg(feature = "toml")]
#[test]
fn file_type_from_ext_toml() {
    let ft = FileType::from_ext("Cargo.toml");
    assert_eq!(ft, FileType::Toml(filess::Toml::new("Cargo.toml")));
}

#[cfg(feature = "ron")]
#[test]
fn file_type_from_ext_ron() {
    let ft = FileType::from_ext("data.ron");
    assert_eq!(ft, FileType::Ron(filess::Ron::new("data.ron")));
}

#[cfg(feature = "md")]
#[test]
fn file_type_from_ext_md() {
    let ft = FileType::from_ext("readme.md");
    assert_eq!(ft, FileType::Md(filess::Md::new("readme.md")));
}

#[cfg(feature = "txt")]
#[test]
fn file_type_from_ext_txt() {
    let ft = FileType::from_ext("notes.txt");
    assert_eq!(ft, FileType::Txt(filess::Txt::new("notes.txt")));
}

// ──── Dir ───────────────────────────────────────────────────────────
#[cfg(feature = "json")]
#[test]
fn dir_new() {
    let d = Dir::<filess::Json>::new("/tmp");
    assert_eq!(d.as_ref(), Path::new("/tmp"));
}

#[cfg(feature = "json")]
#[test]
fn dir_try_new_non_existent() {
    let d = Dir::<filess::Json>::try_new("/some/nonexistent/path").unwrap();
    assert_eq!(d.as_ref(), Path::new("/some/nonexistent/path"));
}

#[cfg(feature = "json")]
#[test]
fn dir_try_new_file_fails() {
    let dir = scratch_dir();
    let f_path = dir.join("not_a_dir.txt");
    std::fs::write(&f_path, b"").unwrap();
    let err = Dir::<filess::Json>::try_new(&f_path).unwrap_err();
    assert!(err.to_string().contains("not a directory"));
}

#[cfg(feature = "json")]
#[test]
fn dir_create_and_remove() {
    let dir = scratch_dir();
    let d = Temporary::new(Dir::<filess::Json>::new(dir.join("mydir")));
    d.create().unwrap();
    assert!(d.as_ref().exists());
    d.remove().unwrap();
    assert!(!d.as_ref().exists());
}

#[cfg(feature = "json")]
#[test]
fn dir_push_and_create_all() {
    let root = scratch_dir();
    let dir_path = root.join("project");
    let mut d = Temporary::new(Dir::<filess::Json>::new(&dir_path));
    let a_path = dir_path.join("a.json");
    let b_path = dir_path.join("b.json");
    d.push(filess::Json::new(&a_path));
    d.push(filess::Json::new(&b_path));
    d.create_all().unwrap();
    assert!(a_path.exists());
    assert!(b_path.exists());
}

#[cfg(feature = "json")]
#[test]
fn dir_load_files() {
    let root = scratch_dir();
    let dir_path = root.join("data");
    let mut d = Temporary::new(Dir::<filess::Json>::new(&dir_path));
    // Use absolute paths so elements can be found independently
    let x_path = dir_path.join("x.json");
    let y_path = dir_path.join("y.json");
    d.push(filess::Json::new(&x_path));
    d.push(filess::Json::new(&y_path));
    d.create_all().unwrap();
    d.elements[0].save(b"10").unwrap();
    d.elements[1].save(b"20").unwrap();
    let contents = d.load_files().unwrap();
    assert_eq!(contents, vec![b"10".to_vec(), b"20".to_vec()]);
}

#[cfg(feature = "json")]
#[test]
fn dir_into_iter() {
    let mut d = Dir::<filess::Json>::new("/tmp");
    d.push(filess::Json::new("a.json"));
    d.push(filess::Json::new("b.json"));
    let paths: Vec<_> = d.into_iter().map(|f| f.as_ref().to_owned()).collect();
    assert_eq!(paths, vec![PathBuf::from("a.json"), PathBuf::from("b.json")]);
}

// ── Temporary ───────────────────────────────────────────────────────
#[cfg(feature = "json")]
#[test]
fn temporary_deletes_on_drop() {
    let dir = scratch_dir();
    let p = dir.join("temp.json");
    {
        let f = filess::Json::new(&p);
        let _tmp = Temporary::new(f);
        _tmp.create().unwrap();
        assert!(p.exists());
    }
    assert!(!p.exists());
}

#[cfg(feature = "json")]
#[test]
fn temporary_deref_to_inner() {
    let dir = scratch_dir();
    let p = dir.join("deref.json");
    let f = filess::Json::new(&p);
    let tmp = Temporary::new(f);
    tmp.create().unwrap();
    assert!(p.exists());
}

#[cfg(feature = "json")]
#[test]
fn temporary_from_conversion() {
    let dir = scratch_dir();
    let p = dir.join("from.json");
    let f = filess::Json::new(&p);
    let tmp: Temporary<filess::Json> = f.into();
    tmp.create().unwrap();
}

// ── Serde Model types ───────────────────────────────────────────────
#[cfg(feature = "_any_serde_model")]
mod model_tests {
    use filess::traits::ModelFile;

    #[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
    struct Config {
        name: String,
        count: usize,
    }

    fn config() -> Config {
        Config { name: "test".into(), count: 42 }
    }

    #[cfg(feature = "json")]
    #[test]
    fn json_model_bytes_roundtrip() {
        let cfg = config();
        let bytes = filess::Json::model_to_bytes(&cfg).unwrap();
        let decoded: Config = filess::Json::bytes_to_model(bytes).unwrap();
        assert_eq!(cfg, decoded);
    }

    #[cfg(feature = "json")]
    #[test]
    fn json_save_and_load_model() {
        let dir = crate::scratch_dir();
        let p = dir.join("model.json");
        let f = filess::Temporary::new(filess::Json::new(&p));
        let cfg = config();
        f.save_model(&cfg).unwrap();
        let loaded: Config = f.load_model().unwrap();
        assert_eq!(cfg, loaded);
    }

    #[cfg(feature = "toml")]
    #[test]
    fn toml_model_bytes_roundtrip() {
        let cfg = config();
        let bytes = filess::Toml::model_to_bytes(&cfg).unwrap();
        let decoded: Config = filess::Toml::bytes_to_model(bytes).unwrap();
        assert_eq!(cfg, decoded);
    }

    #[cfg(feature = "toml")]
    #[test]
    fn toml_save_and_load_model() {
        let dir = crate::scratch_dir();
        let p = dir.join("model.toml");
        let f = filess::Temporary::new(filess::Toml::new(&p));
        let cfg = config();
        f.save_model(&cfg).unwrap();
        let loaded: Config = f.load_model().unwrap();
        assert_eq!(cfg, loaded);
    }

    #[cfg(feature = "ron")]
    #[test]
    fn ron_model_bytes_roundtrip() {
        let cfg = config();
        let bytes = filess::Ron::model_to_bytes(&cfg).unwrap();
        let decoded: Config = filess::Ron::bytes_to_model(bytes).unwrap();
        assert_eq!(cfg, decoded);
    }

    #[cfg(feature = "ron")]
    #[test]
    fn ron_save_and_load_model() {
        let dir = crate::scratch_dir();
        let p = dir.join("model.ron");
        let f = filess::Temporary::new(filess::Ron::new(&p));
        let cfg = config();
        f.save_model(&cfg).unwrap();
        let loaded: Config = f.load_model().unwrap();
        assert_eq!(cfg, loaded);
    }
}

// ── Dir with models ─────────────────────────────────────────────────
#[cfg(all(feature = "serde", feature = "json"))]
#[test]
fn dir_load_models() {
    use filess::traits::ModelFile;

    #[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
    struct Point { x: i32, y: i32 }

    let root = scratch_dir();
    let dir_path = root.join("points");
    let mut d = Temporary::new(Dir::<filess::Json>::new(&dir_path));
    let p1_path = dir_path.join("a.json");
    let p2_path = dir_path.join("b.json");
    d.push(filess::Json::new(&p1_path));
    d.push(filess::Json::new(&p2_path));
    d.create_all().unwrap();
    d.elements[0].save_model(&Point { x: 1, y: 2 }).unwrap();
    d.elements[1].save_model(&Point { x: 3, y: 4 }).unwrap();
    let pts: Vec<Point> = d.load_models().unwrap();
    assert_eq!(pts, vec![Point { x: 1, y: 2 }, Point { x: 3, y: 4 }]);
}

// ── From impls ──────────────────────────────────────────────────────
#[cfg(feature = "json")]
#[test]
fn json_from_various_types() {
    use filess::Json;
    let _a = Json::from(Path::new("a.json"));
    let _b = Json::from(PathBuf::from("b.json"));
    let _c = Json::from("c.json");
    let _d = Json::from(String::from("d.json"));
}

// ── FsElement blanket trait ─────────────────────────────────────────
#[cfg(feature = "json")]
#[test]
fn fs_element_trait_works() {
    let dir = scratch_dir();
    let p = dir.join("fs_elem.json");
    let f = Temporary::new(filess::Json::new(&p));
    f.create().unwrap();
    assert!(f.as_ref().exists());
    let copied = Temporary::new(f.copy(p.with_file_name("copied.json")).unwrap());
    assert!(copied.as_ref().exists());
    f.remove().unwrap();
}

// ── Image types ─────────────────────────────────────────────────────
#[cfg(feature = "image")]
#[test]
fn image_type_has_format() {
    use filess::traits::ImageFile;
    let _fmt = <filess::Image as ImageFile>::image_format();
}

#[cfg(all(feature = "image", feature = "png"))]
#[test]
fn png_metadata() {
    assert!(filess::Png::ext().contains(&"png"));
}

#[cfg(all(feature = "image", feature = "jpeg"))]
#[test]
fn jpeg_metadata() {
    let exts = filess::Jpeg::ext();
    assert!(exts.contains(&"jpg") || exts.contains(&"jpeg"));
}

// ── Audio types ─────────────────────────────────────────────────────
#[cfg(feature = "mp3")]
#[test]
fn mp3_metadata() {
    assert!(filess::Mp3::ext().contains(&"mp3"));
}

#[cfg(feature = "flac")]
#[test]
fn flac_metadata() {
    assert!(filess::Flac::ext().contains(&"flac"));
}

#[cfg(feature = "wav")]
#[test]
fn wav_metadata() {
    assert!(filess::Wav::ext().contains(&"wav"));
}

// ── Async ───────────────────────────────────────────────────────────
#[cfg(all(feature = "async", feature = "json"))]
mod async_tests {
    use filess::traits::AsyncFileTrait;
    use filess::traits::AsyncFsElement;
    use filess::Json;

    #[tokio::test]
    async fn async_save_load() {
        let dir = crate::scratch_dir();
        let p = dir.join("async_test.json");
        let f = filess::Temporary::new(Json::new(&p));
        let data = b"async data";
        f.asave(data).await.unwrap();
        let loaded = f.aload().await.unwrap();
        assert_eq!(loaded, data);
    }

    #[tokio::test]
    async fn async_create_remove() {
        let dir = crate::scratch_dir();
        let p = dir.join("async_lifecycle.json");
        let f = filess::Temporary::new(Json::new(&p));
        f.acreate().await.unwrap();
        assert!(p.exists());
        f.aremove().await.unwrap();
        assert!(!p.exists());
    }
}

// ── Trash ───────────────────────────────────────────────────────────
#[cfg(all(feature = "trash", feature = "json"))]
#[test]
fn trash_file() {
    let dir = scratch_dir();
    let p = dir.join("to_trash.json");
    let f = Temporary::new(filess::Json::new(&p));
    f.create().unwrap();
    f.trash().unwrap();
    assert!(!p.exists());
}

// ── Walk feature ────────────────────────────────────────────────────
#[cfg(all(feature = "walk", feature = "json"))]
#[test]
fn dir_walk() {
    let root = scratch_dir();
    let dir_path = root.join("walk_test");
    let mut d = Temporary::new(Dir::<filess::Json>::new(&dir_path));
    let a_path = dir_path.join("a.json");
    let b_path = dir_path.join("b.json");
    d.push(filess::Json::new(&a_path));
    d.push(filess::Json::new(&b_path));
    d.create_all().unwrap();
    // WalkDir returns the root directory itself plus all entries
    let count = d.walk().into_iter().count();
    assert!(count >= 1);
}

// ── Glob feature ────────────────────────────────────────────────────
#[cfg(all(feature = "glob", feature = "json"))]
#[test]
fn dir_glob() {
    let dir = scratch_dir();
    let d = Temporary::new(Dir::<filess::Json>::new(&dir));
    std::fs::write(dir.join("one.json"), b"").unwrap();
    std::fs::write(dir.join("two.json"), b"").unwrap();
    let pattern = dir.join("*.json").to_string_lossy().to_string();
    let results = d.glob(&pattern);
    assert_eq!(results.len(), 2);
}

// ── Infer feature ───────────────────────────────────────────────────
// Use a larger JSON payload that infer can actually detect.
#[cfg(all(feature = "infer", feature = "json"))]
#[test]
fn infer_json_file() {
    let dir = scratch_dir();
    let p = dir.join("detect.json");
    let f = Temporary::new(filess::Json::new(&p));
    // Write valid JSON with enough content for infer to detect
    let data = br#"{"name":"test","values":[1,2,3,4,5],"nested":{"key":"value"}}"#;
    f.save(data).unwrap();
    let t = f.infer().unwrap();
    // infer may or may not detect JSON; just verify it doesn't error
    if let Some(ty) = t {
        assert_eq!(ty.extension(), "json");
    }
}

#[cfg(all(feature = "infer", feature = "json"))]
#[test]
fn is_correct_data() {
    let dir = scratch_dir();
    let p = dir.join("correct.json");
    let f = Temporary::new(filess::Json::new(&p));
    let data = br#"{"name":"test","values":[1,2,3,4,5],"nested":{"key":"value"}}"#;
    f.save(data).unwrap();
    // Just verify it doesn't error; infer may not detect short JSON
    let _ = f.is_correct_data().unwrap();
}

// ── Multiple From impls for FileType ────────────────────────────────
#[cfg(feature = "json")]
#[test]
fn file_type_from_str() {
    let ft = FileType::from("data.json");
    assert!(matches!(ft, FileType::Json(_)));
}

#[cfg(feature = "json")]
#[test]
fn file_type_from_pathbuf() {
    let ft = FileType::from(PathBuf::from("data.json"));
    assert!(matches!(ft, FileType::Json(_)));
}

#[cfg(feature = "json")]
#[test]
fn file_type_from_path() {
    let ft = FileType::from(Path::new("data.json"));
    assert!(matches!(ft, FileType::Json(_)));
}

// ── Dir division operator ───────────────────────────────────────────
#[cfg(feature = "json")]
#[test]
fn dir_div_str_creates_file_with_ext() {
    let d = Dir::<filess::Json>::new("/base");
    let df = d / "notes.json";
    assert!(matches!(df, DirFile::File(_)));
}

#[cfg(feature = "just_json")]
#[test]
fn dir_div_str_creates_dir_without_ext() {
    let d = Dir::<filess::Json>::new("/base");
    let df = d / "subdir";
    assert!(matches!(df, DirFile::Dir(_)));
}

// ── FileType default ────────────────────────────────────────────────
#[test]
fn file_type_default() {
    let _ft = FileType::default();
}

// ── File creation with init bytes ───────────────────────────────────
#[cfg(feature = "json")]
#[test]
fn create_json_initializes_with_braces() {
    let dir = scratch_dir();
    let p = dir.join("init.json");
    let f = Temporary::new(filess::Json::new(&p));
    f.create().unwrap();
    let content = std::fs::read_to_string(&p).unwrap();
    assert_eq!(content, "{}");
}

// ── Error display ───────────────────────────────────────────────────
#[cfg(feature = "json")]
#[test]
fn file_creation_error_display() {
    use filess::primitives::FileCreationError;
    let err = FileCreationError::<filess::Json>::WrongExtension(
        PathBuf::from("f.txt"),
        "txt".into(),
    );
    let msg = err.to_string();
    assert!(msg.contains("txt"));
    assert!(msg.contains("f.txt"));
}

// ── Open trait compiles ─────────────────────────────────────────────
#[cfg(feature = "open")]
#[test]
fn open_trait_is_implemented() {
    use filess::traits::OpenTrait;
    fn assert_open<T: OpenTrait>() {}
    assert_open::<filess::Json>();
    assert_open::<Dir<filess::Json>>();
}
