use super::Cli;
use std::fs;
use std::path::PathBuf;
use fs_extra::dir::*;
use std::env::set_current_dir;

//    static test: Arc<Mutex<bool>> = Arc::new(Mutex::new(true));

#[test]
fn consequent_tests() {
    test_install();
    test_deploy();
}

struct TestData {
    dir: PathBuf,
    pub source_dir: PathBuf,
    pub snapshot_dir: PathBuf,
    pub test_root_dir: PathBuf,
    pub test_dir: PathBuf,
}
impl TestData {
    fn setup(source: &str, snapshot: &str, id: usize) -> Self {
        let _self = Self {
            dir: std::env::current_dir().unwrap(),
            source_dir: PathBuf::from(format!("testdata/{}", source)),
            snapshot_dir: PathBuf::from(format!("testdata/{}", snapshot)),
            test_root_dir: PathBuf::from(format!("testdata/tmp.{}", id)),
            test_dir: PathBuf::from(format!("testdata/tmp.{}/{}", id, source))
        };
        let options = CopyOptions::new();
        fs::create_dir(&_self.test_root_dir)
            .expect("create temp dir testdata/tmp");
        copy(&_self.source_dir, &_self.test_root_dir, &options)
            .expect(&format!("setup project in {:?}", _self.test_dir));
        _self
    }
    fn to_test_dir(&self) {
        set_current_dir(&self.test_dir)
            .expect(&format!("change dir to {:?}", &self.test_dir));
    }
    fn restore_dir(&self) {
        set_current_dir(&self.dir)
            .expect("unwind current dir to crate dir");
    }
}
impl Drop for TestData {
    fn drop(&mut self) {
        set_current_dir(&self.dir)
            .expect("unwind current dir to crate dir");
        remove(&self.test_root_dir)
            .expect("remove testdata/tmp");
    }
}



fn test_install() {
    // setup test data
    let testdata = TestData::setup("testcli", "testcli.install", 1);
    // run install
    testdata.to_test_dir();
    Cli::Install.exec()
        .expect(&format!("run `wasiworker install` under {:?}", testdata.test_dir));
    // compare with snapshot
    testdata.restore_dir();
    assert!(!dir_diff::is_different(&testdata.snapshot_dir, &testdata.test_dir).unwrap());
}

fn test_deploy() {
    // setup test data
    let testdata = TestData::setup("testcli.install", "testcli.deploy", 2);
    // run install
    testdata.to_test_dir();
    Cli::Deploy.exec()
        .expect(&format!("run `wasiworker deplot` under {:?}", testdata.test_dir));
    // check that all required files in dist exist
    let files = get_dir_content("./dist")
      .expect("dist dir exist after running `wasiworker deploy`")
      .files.len();
    assert_eq!(files, 2, "should have 2 files in ./dist after `wasiworker deploy`");
}
