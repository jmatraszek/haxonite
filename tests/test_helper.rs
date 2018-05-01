pub extern crate difference;
extern crate reqwest;

use std::process::{Command, Stdio};
use std::ffi::OsStr;
use std::fmt::Display;
use std::path::Path;
use std::fs;
use std::{thread, time};
pub use self::reqwest::StatusCode;

const BASE_URL: &str = "http://localhost:4000";
static SLEEP_TIME: time::Duration = time::Duration::from_millis(500);

pub fn create_basic_project<P: AsRef<OsStr>>(project_name: P) {
    Command::new("cargo")
        .current_dir("tests/test_projects")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .arg("run")
        .arg("--")
        .arg("new")
        .arg(project_name)
        .status()
        .expect("Creating project failed.");
}

pub fn remove_test_project<P: AsRef<OsStr>>(project_name: P) {
    let path = Path::new("./tests/test_projects");
    let project_name = Path::new(&project_name).file_name().expect("No project_name specified");
    let project_path = path.join(project_name);
    fs::remove_dir_all(project_path).expect("Removing project failed");
}

pub fn start_project<P, F>(project_name: P, closure: F)
    where P: AsRef<OsStr> + Display,
          F: Fn()
{
    let project_path = format!("tests/test_projects/{}", project_name);
    let mut child = Command::new("cargo")
        .current_dir(project_path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .arg("run")
        .arg("--")
        .arg("serve")
        .spawn()
        .expect("Starting server failed");
    thread::sleep(SLEEP_TIME);

    closure();

    child.kill().expect("Server wasn't running.");
}

pub fn get_request<U, F>(url_path: U, closure: F)
    where U: AsRef<str> + Display,
          F: Fn(reqwest::Response)
{
    let url = &format!("{}{}", BASE_URL, url_path);
    let response = reqwest::get(url).expect("Request failed.");
    closure(response);
}

#[macro_export]
macro_rules! assert_header {
    ($response:expr, $header_name:expr, $header_value:expr) => ({
        assert!($response.headers().get_raw($header_name).unwrap() == $header_value);
    })
}

#[macro_export]
macro_rules! assert_status {
    ($response:expr, $status:expr) => ({
            assert!($response.status() == $status)
        })
}

#[macro_export]
macro_rules! assert_file_exists {
    ($file_path:expr) => ({
        use std::path::Path;
        let path = Path::new("./tests/test_projects").join(Path::new(&$file_path));
        assert!(path.exists())
    })
}

#[macro_export]
macro_rules! assert_file_contents {
    ($file_path:expr) => ({
        use std::io::Read;
        use std::fs::File;
        use std::path::Path;
        use test_helper::difference::{diff, print_diff};
        let test_file_path = Path::new("./tests/test_projects").join(Path::new(&$file_path));
        let mut file_contents = String::new();
        File::open(test_file_path)
            .expect("Error opening file")
            .read_to_string(&mut file_contents)
            .expect("Error reading from file");

        let fixture_path = Path::new("./tests/fixtures").join(Path::new(&$file_path));
        let mut fixture_contents = String::new();
        File::open(fixture_path)
            .expect("Error opening file")
            .read_to_string(&mut fixture_contents)
            .expect("Error reading from file");

        let diff_res = diff(&fixture_contents, &file_contents, "").0;
        if diff_res != 0 {
            print_diff(&fixture_contents, &file_contents, "");
        }
        assert!(diff_res == 0);
    })
}