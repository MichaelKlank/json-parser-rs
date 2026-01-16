/// Main entry point for the JSON parser CLI tool
///
/// Professional Rust developers structure CLI tools with:
/// 1. Clear separation between library code and CLI code
/// 2. Proper error handling with meaningful messages
/// 3. Exit codes following Unix conventions (0 = success, 1-255 = error)
/// 4. Minimal allocations in hot paths
use std::env;
use std::fs;
use std::process;

use json_parser_rs::parse_json;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        process::exit(1);
    }

    let file_path = &args[1];

    // Read file content
    let content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", file_path, e);
            process::exit(1);
        }
    };

    // Parse JSON
    match parse_json(&content) {
        Ok(_json_value) => {
            // For valid JSON, exit with code 0 (success)
            // Optionally print the parsed value for debugging
            if env::var("DEBUG").is_ok() {
                println!("Valid JSON");
                dbg!(_json_value);
            }
            process::exit(0);
        }
        Err(e) => {
            // For invalid JSON, print error and exit with code 1 (error)
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use json_parser_rs::parse_json;

    fn read_file(file_name: &str) -> String {
        fs::read_to_string(file_name).expect("Failed to read test file")
    }

    #[test]
    fn test_is_valid_json_step_2() {
        let content = read_file("tests/step2/valid.json");
        assert!(parse_json(&content).is_ok());
    }

    #[test]
    fn test_is_invalid_json_step_2() {
        let content = read_file("tests/step2/invalid.json");
        assert!(parse_json(&content).is_err());
    }

    #[test]
    fn test_is_valid_json_step_2_2() {
        let content = read_file("tests/step2/valid2.json");
        assert!(parse_json(&content).is_ok());
    }

    #[test]
    fn test_is_invalid_json_step_2_2() {
        let content = read_file("tests/step2/invalid2.json");
        assert!(parse_json(&content).is_err());
    }

    #[test]
    fn test_is_invalid_json_step_2_3() {
        let content = read_file("tests/step2/invalid3.json");
        assert!(parse_json(&content).is_err());
    }

    #[test]
    fn test_is_invalid_json_step_3() {
        let content = read_file("tests/step3/invalid.json");
        assert!(parse_json(&content).is_err());
    }

    #[test]
    fn test_is_valid_json_step_3() {
        let content = read_file("tests/step3/valid.json");
        assert!(parse_json(&content).is_ok());
    }

    #[test]
    fn test_is_invalid_json_step_4() {
        let content = read_file("tests/step4/invalid.json");
        assert!(parse_json(&content).is_err());
    }

    #[test]
    fn test_is_valid_json_step_4() {
        let content = read_file("tests/step4/valid.json");
        assert!(parse_json(&content).is_ok());
    }

    #[test]
    fn test_is_valid_json_step_4_2() {
        let content = read_file("tests/step4/valid2.json");
        assert!(parse_json(&content).is_ok());
    }
}
