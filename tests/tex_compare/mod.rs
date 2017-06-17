extern crate wait_timeout;

use std::env;
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::time::Duration;
use std::process::{Stdio, Command};
use self::wait_timeout::ChildExt;
use std::io::prelude::*;


use tex::parser::Parser;
use tex_format;

const INPUT_FOLDER: &'static str = "assets/";
// TODO: handle windows.
const OUTPUT_FOLDER: &'static str = "/tmp/tex-test";
const PARSER_OUTPUT_FILE_POSTFIX: &'static str = "test";

pub fn test_parser(tex_file: &str) {
    fs::create_dir_all(OUTPUT_FOLDER).unwrap();

    generate_parsed_tex(tex_file);
    println!("tex file parsed and reformatted.");

    execute_tex(&get_input_tex_path(tex_file));
    execute_tex(&get_output_file(
        &get_parsed_tex_filename(tex_file),
        Some("tex"),
    ));

    let dvi = [
        read_dvi_output(&get_output_file(tex_file, Some("dvi"))),
        read_dvi_output(&get_output_file(
            &get_parsed_tex_filename(tex_file),
            Some("dvi"),
        )),
    ];

    println!("{:?}", dvi);
    assert_eq!(dvi[0], dvi[1]);
}

// Execute the real `tex` engine.
fn execute_tex(tex_path: &str) {
    let mut child = Command::new("tex")
        .args(&[tex_path])
        .current_dir(OUTPUT_FOLDER)
        .stdout(Stdio::null())
        .spawn()
        .expect("Can't execute tex");

    match child.wait_timeout(Duration::from_secs(2)).unwrap() {
        Some(status) => assert!(status.success()),
        None => {
            child.kill().unwrap();
            panic!("tex killed when executing: {}", tex_path);
        }
    };
}

fn generate_parsed_tex(input_filename: &str) {
    let input = read_source_tex(input_filename);

    let mut parser = Parser::new(&input);
    let result = parser.parse().unwrap();

    write_parsed_tex(&tex_format::format(&result), input_filename);
}

fn read_source_tex(name: &str) -> String {
    let tex_path = get_input_tex_path(name);

    let mut file = File::open(tex_path).unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    input
}

fn write_parsed_tex(output: &str, input_filename: &str) {
    let path = get_output_file(&get_parsed_tex_filename(input_filename), Some("tex"));

    let mut file = File::create(path).unwrap();
    file.write(output.as_bytes()).unwrap();
}

fn read_dvi_output(dvi_name: &str) -> Vec<u8> {
    let dvi_path = get_output_file(dvi_name, Some("dvi"));
    let mut dvi_file = File::open(dvi_path).unwrap();
    let mut buffer = vec![];

    dvi_file.read_to_end(&mut buffer).unwrap();

    // Make dvi reproducible by removing the first line which contains
    // the compilation time.
    buffer
        .into_iter()
        .skip_while(|x| *x != ('\n' as u8))
        .skip(1)
        .collect()
}

fn get_input_tex_path(test_name: &str) -> String {
    let mut input_path = env::current_dir().unwrap();
    input_path.push(INPUT_FOLDER);
    input_path.push(test_name);
    input_path.set_extension("tex");

    input_path.to_str().unwrap().into()
}

fn get_output_file(file: &str, extensions: Option<&str>) -> String {
    let mut output_path = PathBuf::from(OUTPUT_FOLDER);
    output_path.push(file);

    if let Some(ext) = extensions {
        output_path.set_extension(ext);
    }

    output_path.to_str().unwrap().into()
}

fn get_parsed_tex_filename(source_name: &str) -> String {
    format!("{}-{}", source_name, PARSER_OUTPUT_FILE_POSTFIX)
}
