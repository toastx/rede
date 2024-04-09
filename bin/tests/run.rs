use assert_cmd::Command;
use predicates::boolean::PredicateBooleanExt;
use predicates::prelude::predicate::str::contains;

// Set of integration tests for `rede run`, they are all ignored to run them only manually.
// The majority of these tests are built against a custom test API

macro_rules! test_request {
    ($name:ident -> $assert:expr) => {
        #[test]
        #[ignore]
        fn $name() {
            let file = format!("tests/inputs/{}", stringify!($name));
            Command::cargo_bin("rede")
                .unwrap()
                .arg("run")
                .arg(file)
                .assert()
                .success()
                .stdout($assert);
        }
    };
}

macro_rules! test_error {
    // Runs the test using the file matching the test name
    ($(#[$m:meta])*$name:ident $(, $arg:literal)* -> $assert:expr) => {
        test_error!($(#[$m])*$name <$name>, $($arg),* -> $assert);
    };
    // Runs the test using the `get_simple` file, for test not dependent on the request file
    ($(#[$m:meta])*$name:ident <>, $($arg:literal),* -> $assert:expr) => {
        test_error!($(#[$m])*$name <get_simple>, $($arg),* -> $assert);
    };
    // Runs the test using the given file
    ($(#[$m:meta])*$name:ident <$file:ident>, $($arg:literal),* -> $assert:expr) => {
        $(#[$m])*
        #[test]
        fn $name() {
            let file = format!("tests/inputs/{}", stringify!($file));
            Command::cargo_bin("rede")
                .unwrap()
                .arg("run")
                $(.arg($arg))*
                .arg(file)
                .assert()
                .failure()
                .stderr($assert);
        }
    };
}

test_request!(get_simple -> contains(r#"{"hello":"world"}"#));
test_request!(http_version -> contains(r#""http_version":"HTTP/1.0""#));
test_request!(headers -> contains(r#""content-type":"application/json""#).and(contains(r#""num_headers":5"#)));
test_request!(query_params -> contains(r#""name":["Robert","Edward"]"#).and(contains(r#""num_query_params":3"#)));
test_request!(body_raw -> contains("rede,request").and(contains(r#""content-type":"text/plain"#)));
test_request!(body_binary -> contains(r#""size":8"#).and(contains(r#""content-type":"application/octet-stream""#)));
test_request!(override_content_type -> contains(r#""content-type":"application/json""#));
// todo -no-redirect, requires --verbose

test_error!(missing_file -> contains("invalid [REQUEST]").and(contains("No such file or directory")));
test_error!(invalid_url -> contains("invalid url").and(contains("http://128.0.0.256")));
test_error!(failed_connection -> contains("failed connection").and(contains("completelymadeupurl")));
test_error!(bad_url_scheme -> contains("failed request building").and(contains("htt:/www.url.com")));
test_error!(wrong_binary -> contains("invalid file").and(contains("no_exists.zip")));
test_error!(timeout<>, "--timeout", "0ms" -> contains("timeout"));

test_error!(#[ignore] unsupported_http_version -> contains("wrong http version"));
test_error!(#[ignore] redirect_loop, "--max-redirects", "5" -> contains("redirect"));
