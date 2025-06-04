mod extract_manager;
mod ini_reader;
 use tanzhenhui_code_lib::interaction_helper;

fn main() {
    // test();
    run_code();
}

fn run_code() {
    let mut extract = extract_manager::ExtractManager::new();
    extract.extract_videos_from_compressed_files();
    interaction_helper::final_wait();
}

#[allow(dead_code)]
pub fn test() {
    // test_ini();
    extract_manager::test_infer_crate();
}
