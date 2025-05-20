use std::fs;
use clap::Parser;
use serde_json::{Value, Map};
use clap_verbosity_flag::{InfoLevel, Verbosity};

#[derive(Parser, Debug)]
#[command(version="1.0.0", about="Json Filter Cli Tools", author="PeerHe <weblinuxgame@126.com>")]
struct CliArgs {
    #[command(flatten)]
    verbose: Verbosity<InfoLevel>,

    #[arg(short='i', long="input", help="Input JSON file path", default_value="",value_parser)]
    input: std::path::PathBuf,

    #[arg(short='t', long="template",help ="Template JSON file path", default_value="",value_parser)]
    template: std::path::PathBuf,

    #[arg(short='o', long="output", help="Output JSON file path", default_value = "output.json",value_parser)]
    output: Option<std::path::PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CliArgs::parse();
    
    let input_data: Value = serde_json::from_str(&fs::read_to_string(&args.input)?)?;
    let template_data: Value = serde_json::from_str(&fs::read_to_string(&args.template)?)?;

    let filtered = filter_json(&input_data, &template_data);

    let output_path = args.output.unwrap_or_else(|| "output.json".to_string().parse().unwrap());
    fs::write(&output_path, serde_json::to_string_pretty(&filtered)?)?;

    println!("Filtered JSON saved to {:?}", output_path);
    Ok(())
}

fn filter_json(input: &Value, template: &Value) -> Value {
    match (input, template) {
        (Value::Object(input_map), Value::Object(template_map)) => {
            let mut result = Map::new();
            for (key, template_value) in template_map {
                if let Some(input_value) = input_map.get(key) {
                    result.insert(key.clone(), filter_json(input_value, template_value));
                }else {
                    result.insert(key.clone(), template_value.clone());
                }
            }
            Value::Object(result)
        }
        (Value::Array(input_array), Value::Array(template_array)) => {
            let mut result = Vec::new();
            if template_array.len() == 1 && input_array.len() >=1 {
                for input_value in input_array {
                    result.push(filter_json(input_value, &template_array[0]));
                }
            }
            Value::Array(result)
        }
        (input_value, _) => input_value.clone(),
    }
}
