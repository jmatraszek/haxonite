pub extern crate difference;
extern crate reqwest;

use std::process::{Command, Stdio};
use std::ffi::OsStr;
use std::fmt::Display;
use std::path::Path;
use std::{fs, thread, time};
pub use self::reqwest::{Response, Result, StatusCode};

const BASE_URL: &str = "http://localhost:4000";
pub const TEST_PROJECTS_PATH: &str = "./tests/test_projects";
pub const FIXTURES_PATH: &str = "./tests/fixtures";
static SLEEP_TIME: time::Duration = time::Duration::from_millis(500);

pub fn create_basic_project<P: AsRef<OsStr>>(project_name: P) {
    Command::new("cargo")
        .current_dir(TEST_PROJECTS_PATH)
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
    let path = Path::new(TEST_PROJECTS_PATH);
    let project_name = Path::new(&project_name).file_name().expect("No project_name specified");
    let project_path = path.join(project_name);
    fs::remove_dir_all(project_path).expect("Removing project failed");
}

pub fn start_project<P, F>(project_name: P, closure: F)
    where P: AsRef<OsStr> + Display,
          F: Fn()
{
    let project_path = format!("{}/{}", TEST_PROJECTS_PATH, project_name);
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
          F: Fn(Result<Response>)
{
    let url = &format!("{}{}", BASE_URL, url_path);
    get_request_url(url, closure);
}

pub fn get_request_url<U, F>(url: U, closure: F)
    where U: AsRef<str> + reqwest::IntoUrl,
          F: Fn(Result<Response>)
{
    let response = reqwest::get(url);
    closure(response);
}

#[macro_export]
macro_rules! assert_response {
    ($result:expr, $closure:expr) => ({
        match ($result, $closure) {
            (result, closure) => {
                assert!(result.is_ok());
                closure(result.unwrap());
            }
        }
    })
}

#[macro_export]
macro_rules! assert_header {
    ($response:expr, $header_name:expr, $header_value:expr) => ({
        match ($response, $header_name, $header_value) {
            (response, header_name, header_value) => {
                assert!(response.headers().get(header_name).unwrap() == header_value);
            }
        }
    })
}

#[macro_export]
macro_rules! assert_status {
    ($response:expr, $status:expr) => ({
        match ($response, $status) {
            (response, status) => {
                assert!(response.status() == status)
            }
        }
    })
}

#[macro_export]
macro_rules! assert_file_exists {
    ($file_path:expr) => ({
        match $file_path {
            file_path => {
                let path = $crate::std::path::Path::new(TEST_PROJECTS_PATH).join($crate::std::path::Path::new(&file_path));
                assert!(path.exists())
            }
        }
    })
}

#[macro_export]
macro_rules! assert_file_contents {
    ($file_path:expr) => ({
        match $file_path {
            file_path => {
                let test_file_path = $crate::std::path::Path::new(TEST_PROJECTS_PATH).join($crate::std::path::Path::new(&file_path));
                let mut file_contents = String::new();
                let mut file = $crate::std::fs::File::open(test_file_path)
                    .expect("Error opening file");
                $crate::std::io::Read::read_to_string(&mut file, &mut file_contents)
                    .expect("Error reading from file");

                let fixture_path = $crate::std::path::Path::new(FIXTURES_PATH).join($crate::std::path::Path::new(&file_path));
                let mut fixture_contents = String::new();
                let mut file = $crate::std::fs::File::open(fixture_path)
                    .expect("Error opening file");
                $crate::std::io::Read::read_to_string(&mut file, &mut fixture_contents)
                    .expect("Error reading from file");

                let diff_res = $crate::difference::diff(&fixture_contents, &file_contents, "").0;
                if diff_res != 0 {
                    $crate::difference::print_diff(&fixture_contents, &file_contents, "");
                }
                assert_eq!(diff_res, 0);
            }
        }
    })
}
