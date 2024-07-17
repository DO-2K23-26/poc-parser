use std::fs;

use yaml_rust::YamlLoader;
use crate::actions::ActionRequest;

pub fn parse() -> ActionRequest {
    let yaml_str = fs::read_to_string("src/ci.yaml")
        .expect("Failed to read YAML file");

    let docs = YamlLoader::load_from_str(&yaml_str).unwrap();
    let doc = &docs[0]["actions"];


    println!("{:?}", doc);

    ActionRequest {
        action_id: "1".to_string(),
        runner_type: 0,
        image: Some(0),
        commands: vec!["cargo build".to_string(), "cargo test".to_string()],
    }
}
