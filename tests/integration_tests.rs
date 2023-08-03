
use assert_cmd::{
    self,
    Command,
};
use tempfile::NamedTempFile;

fn create_command(args: &[&str]) -> assert_cmd::assert::Assert {
    Command::cargo_bin("hidepng").unwrap().args(args).assert()
}

#[test]
fn test_encode_decode_default_chunk_type() {
    let output_file = NamedTempFile::new().unwrap();
    let output_file = output_file.path().to_str().unwrap();

    let args = [
        "encode",
        "-f",
        "./assets/catgurl.png",
        "-m",
        "secret message 1",
        "-o",
        output_file,
    ];
    let output = create_command(&args);
    output.success();

    let args = ["decode", "-f", output_file];
    let output = create_command(&args);
    output
        .success()
        .stdout(predicates::str::contains("secret message 1"));
}

#[test]
fn test_encode_decode_custom_chunk_type() {
    let custom_chunk_type = "TXTI";
    let output_file = NamedTempFile::new().unwrap();
    let output_file = output_file.path().to_str().unwrap();

    let args = [
        "encode",
        "-f",
        "./assets/catgurl.png",
        "-c",
        custom_chunk_type,
        "-m",
        "secret message 1",
        "-o",
        output_file,
    ];
    let output = create_command(&args);
    output.success();

    let args = ["decode", "-f", output_file, "-c", custom_chunk_type];
    let output = create_command(&args);
    output
        .success()
        .stdout(predicates::str::contains("secret message 1"));
}

#[test]
fn test_encode_invalid_chunk_type() {
    let custom_chunk_type = "TXtI";

    let args = [
        "encode",
        "-f",
        "./assets/catgurl.png",
        "-c",
        custom_chunk_type,
        "-m",
        "secret message 1",
    ];
    let output = create_command(&args);
    output.failure().stderr(predicates::str::contains(
        "Reserve bit in chunk type `TXtI` should be 0.",
    ));
}

#[test]
fn test_encode_remove_default_chunk_type() {
    let output_file = NamedTempFile::new().unwrap();
    let output_file = output_file.path().to_str().unwrap();

    let args = [
        "encode",
        "-f",
        "./assets/catgurl.png",
        "-m",
        "secret message 1",
        "-o",
        output_file,
    ];
    let output = create_command(&args);
    output.success();

    let args = ["remove", "-f", output_file];
    let output = create_command(&args);
    output.success();
}
