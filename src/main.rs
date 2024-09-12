mod extract_manager;
mod ini_reader;

fn main() {
    // test();
    run_code();
}

fn run_code() {
    let extract = extract_manager::ExtractManager::new();
    extract.extract_videos_from_compressed_files();
}

#[allow(dead_code)]
pub fn test() {
    // test_ini();
    extract_manager::test_infer_crate();
}
