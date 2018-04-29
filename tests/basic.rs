mod common;
// use common::*;
use common::common::*;

#[test]
fn create_basic_project_test() {
    create_basic_project("basic").expect("Creating project failed.");

    assert_file_exists("basic");
    assert_file_exists("basic/config.toml");
    assert_file_exists("basic/responses/haxonite.json");
    assert_file_exists("basic/assets/haxonite.png");

    assert_file_contents("basic/config.toml");
    assert_file_contents("basic/responses/haxonite.json");

    remove_test_project("basic")
}

