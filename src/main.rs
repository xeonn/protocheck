use clap::{Arg, Command};
use protobuf::descriptor::FileDescriptorSet;
use protobuf_parse::Parser;
use serde_json::Value;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, Read};

#[derive(Debug)]
struct ValidationError {
    field: String,
    error_type: ErrorType,
}

#[derive(Debug)]
enum ErrorType {
    AdditionalField,
    MissingField,
    WrongDataType,
    MissingArrayField,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse CLI arguments
    let matches = Command::new("Proto vs JSON Validator")
        .version("0.1.0")
        .author("Onn Khairuddin")
        .about("Validates JSON against a Protobuf schema")
        .arg(
            Arg::new("proto")
                .short('p')
                .long("proto")
                .value_name("FILE")
                .help("Path to the .proto file")
                .required(true),
        )
        .arg(
            Arg::new("include")
                .short('i')
                .long("include")
                .value_name("DIRECTORY")
                .help("Path containing .proto file")
                .required(true),
        )
        .arg(
            Arg::new("json")
                .short('j')
                .long("json")
                .value_name("FILE")
                .help("Path to the JSON file (or '-' for stdin)")
                .required(true),
        )
        .arg(
            Arg::new("ignore_missing")
                .short('g')
                .long("ignore-missing")
                .help("Ignore missing fields in JSON")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    // Read the .proto file
    let proto_path = matches.get_one::<String>("proto").unwrap();
    let proto_dir = matches.get_one::<String>("include").unwrap();

    // Parse the .proto file into a FileDescriptorSet
    let file_descriptor_set = Parser::new()
        .pure()
        .inputs(&[proto_path])
        .includes(&[proto_dir])
        .file_descriptor_set()
        .unwrap();

    // Read the JSON input
    let json_input = matches.get_one::<String>("json").unwrap();
    let json_value: Value = if json_input == "-" {
        // Read from stdin
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        serde_json::from_str(&buffer)?
    } else {
        // Read from file
        let file_content = read_file(json_input)?;
        serde_json::from_slice(&file_content)?
    };

    // Validate JSON against the Protobuf schema
    let ignore_missing = matches.get_flag("ignore_missing");
    let errors = validate_json(&file_descriptor_set, &json_value, ignore_missing);

    // Report validation errors
    if errors.is_empty() {
        println!("JSON is valid against the Protobuf schema.");
    } else {
        for error in errors {
            println!("Validation Error: {:?}", error);
        }
    }

    Ok(())
}

/// Reads a file into a byte vector.
fn read_file(path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

/// Validates JSON against a Protobuf schema.
fn validate_json(
    file_descriptor_set: &FileDescriptorSet,
    json_value: &Value,
    ignore_missing: bool,
) -> Vec<ValidationError> {
    let mut errors = Vec::new();

    // Extract field names and types from the Protobuf schema
    let mut proto_fields = HashSet::new();
    for file in &file_descriptor_set.file {
        for message in &file.message_type {
            for field in &message.field {
                proto_fields.insert(field.name.clone());
            }
        }
    }

    // Validate JSON fields
    if let Value::Object(json_obj) = json_value {
        for (key, _value) in json_obj {
            let k = Some(key.clone());
            if !proto_fields.contains(&k) {
                errors.push(ValidationError {
                    field: key.clone(),
                    error_type: ErrorType::AdditionalField,
                });
            } else {
                // TODO: Add data type validation
            }
        }
        if !ignore_missing {
            for field in &proto_fields {
                let fld = field.clone().unwrap().clone();
                if !json_obj.contains_key(&fld) {
                    errors.push(ValidationError {
                        field: fld,
                        error_type: ErrorType::MissingField,
                    });
                }
            }
        }
    }

    errors
}
