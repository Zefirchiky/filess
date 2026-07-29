use crate::define_file;

define_file!(Txt, "txt", ["text/plain", "image/ascii-art", "text/vnd.ascii-art", "text/x-ascii-art", "text/json", "application/octet-stream", "text/xml", "text/asciidoc"], ["txt"]);

#[cfg(test)]
mod txt_tests {
    use std::env::temp_dir;

    use crate::Temporary;

    use super::*;

    #[test]
    fn no_init_bytes() {
        let dir = temp_dir();
        let p = dir.join("plain.txt");
        let f = Temporary::new(Txt::new(&p));
        let loaded = f.load().unwrap();
        assert!(loaded.is_empty());
    }
}