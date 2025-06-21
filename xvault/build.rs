use std::{env, fs, io::Write, path::Path};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    build_xfile_tests(out_dir.clone());
    build_device_tests(out_dir.clone());
    build_volume_tests(out_dir);
}

fn build_xfile_tests(out_dir: String) {
    let dest_path = Path::new(&out_dir).join("generated_xfile_tests.rs");
    let mut f = fs::File::create(&dest_path).unwrap();
    let assets_dir = Path::new("../assets");

    let mut test_id = 0;

    for entry in walkdir::WalkDir::new(assets_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        let relative_path = path.strip_prefix(assets_dir).unwrap();
        let relative_path_str = relative_path.to_string_lossy().replace('\\', "/");

        let test_name = format!("xfile_generated_test_{}", test_id);

        writeln!(
            f,
            r#"
    #[test]
    fn {test_name}() {{
    test_file("{relative_path_str}");
    }}
    "#,
            test_name = test_name,
            relative_path_str = relative_path_str,
        )
        .unwrap();

        test_id += 1;
    }
}


fn build_device_tests(out_dir: String) {
    let dest_path = Path::new(&out_dir).join("generated_device_tests.rs");
    let mut f = fs::File::create(&dest_path).unwrap();
    let assets_dir = Path::new("../assets");

    let mut test_id = 0;

    for entry in walkdir::WalkDir::new(assets_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        let relative_path = path.strip_prefix(assets_dir).unwrap();
        let relative_path_str = relative_path.to_string_lossy().replace('\\', "/");

        let test_name = format!("device_generated_test_{}", test_id);

        writeln!(
            f,
            r#"
    #[test]
    fn {test_name}() {{
    test_file("{relative_path_str}");
    }}
    "#,
            test_name = test_name,
            relative_path_str = relative_path_str,
        )
        .unwrap();

        test_id += 1;
    }
}

fn build_volume_tests(out_dir: String) {
    let dest_path = Path::new(&out_dir).join("generated_volume_tests.rs");
    let mut f = fs::File::create(&dest_path).unwrap();
    let assets_dir = Path::new("../assets");

    let mut test_id = 0;

    for entry in walkdir::WalkDir::new(assets_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        let relative_path = path.strip_prefix(assets_dir).unwrap();
        let relative_path_str = relative_path.to_string_lossy().replace('\\', "/");

        let test_name = format!("volume_read_and_write_offsets_generated_test_{}", test_id);

        writeln!(
            f,
            r#"
    #[test]
    fn {test_name}() {{
    volume_test_read_and_write_offsets("{relative_path_str}".into(), {test_id});
    }}
    "#,
            test_name = test_name,
            relative_path_str = relative_path_str,
            test_id = test_id
        )
        .unwrap();

        test_id += 1;
    }
}

