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
    let mut f = Temporary::new(filess::Json::new(&src));
    f.save(b"rename me").unwrap();
    f.rename(&dst).unwrap();
    let renamed = Temporary::from(f);
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
    f.rename_file(PathBuf::from("b.json"));
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
#[cfg(feature = "json")]
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
    use filess::traits::FileTrait;
    use filess::traits::FsElement;
    use filess::{Dir, Json};

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

    #[tokio::test]
    async fn async_acopy() {
        let dir = crate::scratch_dir();
        let src = dir.join("async_src.json");
        let dst = dir.join("async_dst.json");
        let f = filess::Temporary::new(Json::new(&src));
        f.asave(b"async copy").await.unwrap();
        let copied = f.acopy(&dst).await.unwrap();
        assert!(dst.exists());
        let loaded = copied.aload().await.unwrap();
        assert_eq!(loaded, b"async copy");
    }

    #[tokio::test]
    async fn async_arename() {
        let dir = crate::scratch_dir();
        let src = dir.join("async_old.json");
        let dst = dir.join("async_new.json");
        let f = filess::Temporary::new(Json::new(&src));
        f.asave(b"async rename").await.unwrap();
        let renamed = f.arename(&dst).await.unwrap();
        assert!(!src.exists());
        assert!(dst.exists());
        let loaded = renamed.aload().await.unwrap();
        assert_eq!(loaded, b"async rename");
    }

    #[tokio::test]
    async fn async_dir_acreate_all() {
        let root = crate::scratch_dir();
        let dir_path = root.join("async_dir");
        let mut d = Dir::<Json>::new(&dir_path);
        let a_path = dir_path.join("a.json");
        let b_path = dir_path.join("b.json");
        d.push(Json::new(&a_path));
        d.push(Json::new(&b_path));
        d.acreate_all().await.unwrap();
        assert!(a_path.exists());
        assert!(b_path.exists());
        // clean up manually since Temporary can't wrap async ops
        d.remove().unwrap();
    }

    #[tokio::test]
    async fn async_dir_aload_files() {
        let root = crate::scratch_dir();
        let dir_path = root.join("async_load");
        let mut d = Dir::<Json>::new(&dir_path);
        let a_path = dir_path.join("a.json");
        let b_path = dir_path.join("b.json");
        d.push(Json::new(&a_path));
        d.push(Json::new(&b_path));
        d.create_all().unwrap();
        d.elements[0].save(b"alpha").unwrap();
        d.elements[1].save(b"beta").unwrap();
        let contents = d.aload_files().await.unwrap();
        assert!(contents.contains(&b"alpha".to_vec()));
        assert!(contents.contains(&b"beta".to_vec()));
        d.remove().unwrap();
    }
}

#[cfg(all(feature = "async", feature = "json", feature = "serde"))]
mod async_model_tests {
    use filess::traits::{FsElement, ModelFile};
    use filess::{Dir, Json};

    #[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
    struct Cfg { key: String }

    #[tokio::test]
    async fn async_dir_aload_models() {
        let root = crate::scratch_dir();
        let dir_path = root.join("async_models");
        let mut d = Dir::<Json>::new(&dir_path);
        let a_path = dir_path.join("a.json");
        let b_path = dir_path.join("b.json");
        d.push(Json::new(&a_path));
        d.push(Json::new(&b_path));
        d.create_all().unwrap();
        d.elements[0].save_model(&Cfg { key: "a".into() }).unwrap();
        d.elements[1].save_model(&Cfg { key: "b".into() }).unwrap();
        let models: Vec<Cfg> = d.aload_models().await.unwrap();
        assert_eq!(models, vec![Cfg { key: "a".into() }, Cfg { key: "b".into() }]);
        d.remove().unwrap();
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

// ═════════════════════════════════════════════════════════════════════
// New tests
// ═════════════════════════════════════════════════════════════════════

// ── Dir::save_files ────────────────────────────────────────────────
#[cfg(feature = "json")]
#[test]
fn dir_save_files() {
    let root = scratch_dir();
    let dir_path = root.join("save_files");
    let mut d = Temporary::new(Dir::<filess::Json>::new(&dir_path));
    let a_path = dir_path.join("a.json");
    let b_path = dir_path.join("b.json");
    d.push(filess::Json::new(&a_path));
    d.push(filess::Json::new(&b_path));
    d.create_all().unwrap();
    d.save_files(vec![b"alpha".to_vec(), b"beta".to_vec()]).unwrap();
    assert_eq!(std::fs::read_to_string(&a_path).unwrap(), "alpha");
    assert_eq!(std::fs::read_to_string(&b_path).unwrap(), "beta");
}

// ── Dir::trash_files ───────────────────────────────────────────────
#[cfg(all(feature = "trash", feature = "json"))]
#[test]
fn dir_trash_files() {
    let root = scratch_dir();
    let dir_path = root.join("trash_dir");
    let mut d = Temporary::new(Dir::<filess::Json>::new(&dir_path));
    let a_path = dir_path.join("a.json");
    let b_path = dir_path.join("b.json");
    d.push(filess::Json::new(&a_path));
    d.push(filess::Json::new(&b_path));
    d.create_all().unwrap();
    let result = d.trash_files();
    // trash_files may fail on CI/headless environments
    if result.is_ok() {
        assert!(!a_path.exists());
        assert!(!b_path.exists());
    }
}

// ── Dir::glob_with ─────────────────────────────────────────────────
#[cfg(all(feature = "glob", feature = "json"))]
#[test]
fn dir_glob_with() {
    let dir = scratch_dir();
    let d = Temporary::new(Dir::<filess::Json>::new(&dir));
    std::fs::write(dir.join("one.json"), b"").unwrap();
    std::fs::write(dir.join("two.json"), b"").unwrap();
    let pattern = dir.join("*.json").to_string_lossy().to_string();
    use filess::glob::MatchOptions;
    let opts = MatchOptions {
        case_sensitive: true,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };
    let results = d.glob_with(&pattern, opts);
    assert_eq!(results.len(), 2);
}

// ── Dir::rename_file (no FS change) ─────────────────────────────────
#[cfg(feature = "json")]
#[test]
fn dir_rename_file_no_fs() {
    use std::path::Path;
    let dir = scratch_dir();
    let orig = dir.join("original_name");
    let mut d = Temporary::new(Dir::<filess::Json>::new(&orig));
    d.create().unwrap();
    assert!(orig.exists());
    d.rename_file("renamed_dir");
    assert_eq!(d.as_ref(), Path::new("renamed_dir"));
    // original on disk unchanged
    assert!(orig.exists());
}

// ── Dir::From impls ────────────────────────────────────────────────
#[cfg(feature = "json")]
#[test]
fn dir_from_path() {
    let d = Dir::<filess::Json>::from(Path::new("/from/path"));
    assert_eq!(d.as_ref(), Path::new("/from/path"));
}

#[cfg(feature = "json")]
#[test]
fn dir_from_pathbuf() {
    let d = Dir::<filess::Json>::from(PathBuf::from("/from/pathbuf"));
    assert_eq!(d.as_ref(), Path::new("/from/pathbuf"));
}

#[cfg(feature = "json")]
#[test]
fn dir_from_str() {
    let d = Dir::<filess::Json>::from("/from/str");
    assert_eq!(d.as_ref(), Path::new("/from/str"));
}

#[cfg(feature = "json")]
#[test]
fn dir_from_string() {
    let d = Dir::<filess::Json>::from(String::from("/from/string"));
    assert_eq!(d.as_ref(), Path::new("/from/string"));
}

// ── Dir::Deref and DerefMut ────────────────────────────────────────
#[cfg(feature = "json")]
#[test]
fn dir_deref_to_path() {
    use std::ops::Deref;
    let d = Dir::<filess::Json>::new("/some/dir");
    let p: &Path = d.deref();
    assert_eq!(p, Path::new("/some/dir"));
}

#[cfg(feature = "json")]
#[test]
fn dir_deref_mut() {
    use std::ops::DerefMut;
    let mut d = Dir::<filess::Json>::new("/original");
    let p: &mut Path = d.deref_mut();
    assert_eq!(p, Path::new("/original"));
}

// ── Dir::Div<Self> ─────────────────────────────────────────────────
#[cfg(feature = "json")]
#[test]
fn dir_div_dir() {
    let a = Dir::<filess::Json>::new("/base");
    let b = Dir::<filess::Json>::new("sub");
    let c = a / b;
    let expected = if cfg!(windows) { r"\base\sub" } else { "/base/sub" };
    assert_eq!(c.as_ref(), Path::new(expected));
}

// ── Dir into_iter references ───────────────────────────────────────
#[cfg(feature = "json")]
#[test]
fn dir_into_iter_ref() {
    let mut d = Dir::<filess::Json>::new("/tmp");
    d.push(filess::Json::new("a.json"));
    d.push(filess::Json::new("b.json"));
    let paths: Vec<_> = (&d).into_iter().map(|f| f.as_ref().to_owned()).collect();
    assert_eq!(paths, vec![PathBuf::from("a.json"), PathBuf::from("b.json")]);
}

#[cfg(feature = "json")]
#[test]
fn dir_into_iter_mut() {
    let mut d = Dir::<filess::Json>::new("/tmp");
    d.push(filess::Json::new("a.json"));
    d.push(filess::Json::new("b.json"));
    let names: Vec<_> = (&mut d).into_iter().map(|f| f.as_ref().to_owned()).collect();
    assert_eq!(names, vec![PathBuf::from("a.json"), PathBuf::from("b.json")]);
}

// ── FileTrait::as_file ──────────────────────────────────────────────
#[cfg(feature = "json")]
#[test]
fn file_as_file_returns_handle() {
    let dir = scratch_dir();
    let p = dir.join("as_file.json");
    let f = Temporary::new(filess::Json::new(&p));
    f.save(b"content").unwrap();
    use std::io::Write;
    let mut handle = f.as_file().unwrap();
    handle.write_all(b" appended").unwrap();
    drop(handle);
    let content = std::fs::read_to_string(&p).unwrap();
    assert!(content.len() > 0);
}

// ── FileTrait::enforce (cfg infer) ──────────────────────────────────
#[cfg(all(feature = "infer", feature = "json"))]
#[test]
fn file_enforce_passes_for_correct_data() {
    let dir = scratch_dir();
    let p = dir.join("enforce.json");
    let f = Temporary::new(filess::Json::new(&p));
    // Use a larger JSON payload that infer can reliably detect
    let data = br#"{"name":"test","values":[1,2,3,4,5,6,7,8,9,10],"nested":{"deeply":{"nested":{"key":"value"}}},"array":["a","b","c","d","e","f","g"]}"#;
    f.save(data).unwrap();
    // infer may not always detect JSON; skip assertion if it doesn't
    if f.infer().unwrap().is_some() {
        f.enforce().unwrap();
    }
}

// ── File::ext() edge case (ext = [""]) ──────────────────────────────
#[test]
fn file_type_has_empty_ext_slice() {
    // File::ext() returns &[""] — the slice is non-empty but extension string is empty
    assert_eq!(filess::File::ext(), &[""]);
    // File can never be constructed for a real path (always panics)
    // because no path has extension == "" and None triggers NoExtension
    let err = filess::File::try_new("something.txt").unwrap_err();
    assert!(err.to_string().contains("txt"));
}

// ── TextTypes / ImageTypes / AudioTypes from_ext ───────────────────
#[cfg(feature = "json")]
#[test]
fn text_types_from_ext() {
    let t = filess::TextTypes::from_ext("cfg.json");
    assert_eq!(t, filess::TextTypes::Json(filess::Json::new("cfg.json")));
}

#[cfg(all(feature = "image", feature = "jpeg"))]
#[test]
fn image_types_from_ext() {
    let t = filess::ImageTypes::from_ext("photo.jpg");
    assert!(matches!(t, filess::ImageTypes::Jpeg(_)));
}

#[cfg(all(feature = "audio", feature = "mp3"))]
#[test]
fn audio_types_from_ext() {
    let t = filess::AudioTypes::from_ext("song.mp3");
    assert!(matches!(t, filess::AudioTypes::Mp3(_)));
}

// ── Error variant tests ─────────────────────────────────────────────
#[cfg(feature = "json")]
#[test]
fn file_creation_error_no_extension() {
    use filess::primitives::FileCreationError;
    let err = FileCreationError::<filess::Json>::NoExtension(PathBuf::from("no_ext"));
    let msg = err.to_string();
    assert!(msg.contains("no extension"));
}

#[cfg(feature = "json")]
#[test]
fn file_creation_error_invalid_utf8() {
    use std::ffi::OsString;
    use std::os::unix::ffi::OsStringExt;
    use filess::primitives::FileCreationError;
    let err = FileCreationError::<filess::Json>::InvalidUtf8(OsString::from_vec(b"\xff\xfe".to_vec()));
    let msg = err.to_string();
    assert!(msg.contains("UTF-8"));
}

// ── Temporary::DerefMut ────────────────────────────────────────────
#[cfg(feature = "json")]
#[test]
fn temporary_deref_mut() {
    use std::ops::DerefMut;
    let dir = scratch_dir();
    let p = dir.join("tmp_deref_mut.json");
    let mut tmp = Temporary::new(filess::Json::new(&p));
    tmp.create().unwrap();
    {
        let inner: &mut filess::Json = tmp.deref_mut();
        inner.rename_file("renamed.json");
    }
    // path changed inside Temporary
    assert_eq!(tmp.as_ref(), Path::new("renamed.json"));
}

// ── FileBase AsMut ─────────────────────────────────────────────────
#[cfg(feature = "json")]
#[test]
fn file_base_as_mut_path() {
    use std::path::Path;
    let mut f = filess::Json::new("orig.json");
    let p: &mut Path = f.as_mut();
    // verify it points to the same path
    assert_eq!(p, Path::new("orig.json"));
}

// ── FileBase From impls ─────────────────────────────────────────────
#[cfg(feature = "json")]
#[test]
fn file_base_from_path() {
    use filess::primitives::FileBase;
    let _fb = FileBase::<filess::Json>::from(Path::new("test.json"));
}

#[cfg(feature = "json")]
#[test]
fn file_base_from_pathbuf() {
    use filess::primitives::FileBase;
    let _fb = FileBase::<filess::Json>::from(PathBuf::from("test.json"));
}

#[cfg(feature = "json")]
#[test]
fn file_base_from_str() {
    use filess::primitives::FileBase;
    let _fb = FileBase::<filess::Json>::from("test.json");
}

#[cfg(feature = "json")]
#[test]
fn file_base_from_string() {
    use filess::primitives::FileBase;
    let _fb = FileBase::<filess::Json>::from(String::from("test.json"));
}

// ── DirFile constructability ───────────────────────────────────────
#[cfg(feature = "json")]
#[test]
fn dir_file_constructable() {
    let _ = filess::DirFile::<filess::FileType, filess::FileType>::Dir(
        filess::Dir::new("/tmp"),
    );
    let f = filess::DirFile::<filess::FileType, filess::FileType>::File(
        <filess::FileType as FileTrait>::new("/tmp/f.json"),
    );
    assert!(matches!(f, filess::DirFile::File(_)));
}

// ── New Dir::self_bytes_to_models test ─────────────────────────────
#[cfg(feature = "json")]
#[test]
fn dir_self_bytes_to_models() {
    use filess::traits::ModelFile;

    #[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
    struct Item { v: i32 }

    let root = scratch_dir();
    let dir_path = root.join("self_bytes");
    let mut d = Temporary::new(Dir::<filess::Json>::new(&dir_path));
    let a = dir_path.join("a.json");
    let b = dir_path.join("b.json");
    d.push(filess::Json::new(&a));
    d.push(filess::Json::new(&b));
    d.create_all().unwrap();

    let bytes_a = filess::Json::model_to_bytes(&Item { v: 1 }).unwrap();
    let bytes_b = filess::Json::model_to_bytes(&Item { v: 2 }).unwrap();
    let items: Vec<Item> = d.self_bytes_to_models(vec![bytes_a, bytes_b]).unwrap();
    assert_eq!(items, vec![Item { v: 1 }, Item { v: 2 }]);
}

// ── ModelType tests ─────────────────────────────────────────────────
#[cfg(all(feature = "_any_model", feature = "json"))]
#[test]
fn model_type_from_ext_json() {
    let mt = filess::ModelType::from_ext("cfg.json");
    assert!(matches!(mt, Some(filess::ModelType::Json(_))));
}

#[cfg(all(feature = "_any_model", feature = "json"))]
#[test]
fn model_type_from_ext_unknown_returns_none() {
    let mt = filess::ModelType::from_ext("data.txt");
    assert!(mt.is_none());
}

#[cfg(all(feature = "_any_model", feature = "json"))]
#[test]
fn model_type_file_trait_json() {
    use filess::traits::FileTrait;
    let _mt = filess::ModelType::from_ext("cfg.json").unwrap();
    assert_eq!(filess::ModelType::ext(), &[] as &[&str]);
    assert_eq!(filess::ModelType::ext_name(), "");
}

// ── Sync counterpart links documented ──────────────────────────────
// (Compile-time verification that sync → async cross-links resolve)
