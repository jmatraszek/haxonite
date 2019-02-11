#[macro_use] mod test_helper;
use test_helper::*;

#[test]
fn create_basic_project_test() {
    create_basic_project("basic");

    assert_file_exists!("basic");
    assert_file_exists!("basic/config.toml");
    assert_file_exists!("basic/responses/haxonite.json");
    assert_file_exists!("basic/assets/haxonite.png");

    assert_file_contents!("basic/config.toml");
    assert_file_contents!("basic/responses/haxonite.json");

    remove_test_project("basic");
}

#[test]
fn test_basic_project_test() {
    create_basic_project("basic2");

    start_project("basic2", || {
        get_request("/", |response| {
            assert_response!(response, |response: Response| {
                assert_status!(&response, StatusCode::OK);
                assert_header!(&response, "Content-Type", "application/json");
            });
        });

        get_request("/public/haxonite.png", |response| {
            assert_response!(response, |response: Response| {
                assert_status!(&response, StatusCode::OK);
                assert_header!(&response, "Content-Type", "image/png");
                assert_header!(&response, "Content-Length", "60629");
            });
        });
    });

    remove_test_project("basic2");
}

