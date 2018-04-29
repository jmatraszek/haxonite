#[cfg(test)]
pub mod common {
    extern crate difference;

    use std::process::{Command, ExitStatus, Stdio};
    use std::ffi::OsStr;
    use std::io::{Error, Read};
    use std::path::Path;
    use std::fs;
    use std::fs::File;
    use self::difference::{diff, print_diff};

    pub fn create_basic_project<P: AsRef<OsStr>>(project_name: P) -> Result<ExitStatus, Error> {
        Command::new("cargo")
            .current_dir("tests/test_projects")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .arg("run")
            .arg("--")
            .arg("new")
            .arg(project_name)
            .status()
    }

    pub fn remove_test_project<P: AsRef<OsStr>>(project_name: P) {
        let path = Path::new("./tests/test_projects");
        let project_name = Path::new(&project_name).file_name().expect("No project_name specified");
        let project_path = path.join(project_name);
        fs::remove_dir_all(project_path).expect("Removing project failed");
    }

    pub fn assert_file_exists<P: AsRef<OsStr>>(file_path: P) {
        let path = Path::new("./tests/test_projects").join(Path::new(&file_path));
        assert!(path.exists())
    }

    pub fn assert_file_contents<P: AsRef<OsStr>>(file_path: P) {
        let test_file_path = Path::new("./tests/test_projects").join(Path::new(&file_path));
        let mut file_contents = String::new();
        File::open(test_file_path)
            .expect("Error opening file")
            .read_to_string(&mut file_contents)
            .expect("Error reading from file");

        let fixture_path = Path::new("./tests/fixtures").join(Path::new(&file_path));
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
    }
}
