use clap::Parser;
use serde_derive::Deserialize;
use std::{fs, path::PathBuf, process::{Command, exit}};
use toml;
use starknet::{contract::ContractFactory, core::types::FieldElement, signers::SigningKey};



#[derive(Parser, Clone)]
#[command(version = "1.0", about = "Automates the process of deploying contracts on StarkNet.")]
struct Arguments {
    /// Path to the directory containing the contract to deploy.
    #[arg(short = 'p', long)]
    path: PathBuf,

    /// Wallet address deploying contract
    #[clap(short = 'd', long = "wallet-address")]
    public_key: String,

    // Contract to be deployed
    #[clap(short = 'c', long = "contract-name")]
    contract_name: String,

    #[clap(long = "constructor")]
    constructor_inputs: Option<String>,
}

#[derive(Deserialize)]
struct Package {
    name: String,
}

#[derive(Deserialize)]
struct Data {
    package: Package,
}

fn read_scarb_toml(path: &PathBuf) -> String {
    let scarb_toml_path = path.join("Scarb.toml");
    fs::read_to_string(scarb_toml_path.clone()).unwrap_or_else(|_| {
        eprintln!("Could not read file `{:?}`", scarb_toml_path);
        exit(1);
    })
}

fn parse_package_name(contents: &str) -> String {
    let data: Data = toml::from_str(contents).unwrap_or_else(|_| {
        eprintln!("Unable to load data from provided content");
        exit(1);
    });
    data.package.name
}

fn compile_contract(path: &PathBuf) {
    println!("Compiling the contract...");
    let compile_command = Command::new("scarb")
        .arg("build")
        .current_dir(path)
        .status()
        .expect("Failed to compile the contract");
    if !compile_command.success() {
        eprintln!("Compilation failed");
        exit(1);
    }
}

fn generate_keystore() {
    println!("Generating Keystore...");
    let generate_keystore_command = Command::new("starkli")
        .args(&["signer", "keystore", "from-key", "account0_keystore.json"])
        .output()
        .expect("Failed to generate account");
    if !generate_keystore_command.status.success() {
        eprintln!("Account generation failed");
    }
}

fn fetch_account(public_key: &str) {
    println!("Fetching generated account...");
    let fetch_account_command = Command::new("starkli")
        .args(&[
            "account",
            "fetch",
            public_key,
            "--rpc",
            "https://starknet-sepolia.public.blastapi.io",
            "--output",
            "account0_account.json",
        ])
        .status()
        .expect("Failed to fetch account");
}

fn declare_contract(contract_file_path: &PathBuf, base_dir: &PathBuf) -> String {
    println!("Declaring the contract...");
    let mut binding = Command::new("starkli");
    let declare_command = binding
        .args(&[
            "declare",
            &contract_file_path.to_str().unwrap(),
            "--rpc",
            "https://starknet-sepolia.public.blastapi.io",
            "--account",
            "account0_account.json",
            "--keystore",
            "account0_keystore.json",
        ])
        .current_dir(base_dir);
    println!("Declare command: {:?}", declare_command);
    declare_command.status().expect("Failed to declare the contract");

    let output = declare_command.output().expect("Failed to execute declare command");
    let output_str = String::from_utf8(output.stdout)
    .expect("Invalid UTF-8 sequence in command output")
    .trim()
    .to_string();
            return output_str;
}

fn parse_constructor_args(constructor_inputs: &Option<String>) -> Vec<String> {
    match constructor_inputs {
        Some(input) => {
            if input.ends_with(".constructor") {
                fs::read_to_string(input).map_or_else(
                    |_| {
                        eprintln!("Could not read constructor file `{:?}`", input);
                        exit(1);
                    },
                    |contents| {
                        contents
                            .trim()
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .collect()
                    },
                )
            } else {
                input
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect()
            }
        }
        None => Vec::new(),
    }
    .into_iter()
    .flat_map(|arg| {
        arg.split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect::<Vec<String>>()
    })
    .map(|arg| {
        // Remove only the backslashes while keeping the quotes intact
        arg.replace(r#"\\(?=[^"])"#, "")
    })
    .collect()
}

fn deploy_contract(class_hash: &str, constructor_args: &[String], path: &PathBuf) {
    let formatted_constructor_args = constructor_args.join(" ");
    let deploy_args = [
        "deploy",
        "--account",
        "account0_account.json",
        class_hash,
        // " 'mytesttoken' 'MTT' 18 1000000000000 0x0455ea216a2ebc127b8e84c18f096542c62bc29182b0055638b06fa11571426a",
        &formatted_constructor_args,
        "--rpc",
        "https://starknet-sepolia.public.blastapi.io/rpc/v0_7",
        "--keystore",
        "account0_keystore.json",
    ];

    println!("Deploy command: {:?}", deploy_args);

    let deploy_command = Command::new("starkli")
        .args(&deploy_args)
        .current_dir(path)
        .status()
        .expect("Failed to deploy the contract");

    if !deploy_command.success() {
        eprintln!("Deploy command failed");
        exit(1);
    }
}

fn main() {
    let args = Arguments::parse();

    let contents = read_scarb_toml(&args.path);
    let package_name = parse_package_name(&contents);

    compile_contract(&args.path);
    generate_keystore();
    fetch_account(&args.public_key);

    let target_dev_dir = PathBuf::from("target/dev");
    let contract_file_path = target_dev_dir.join(format!("{}_{}.contract_class.json", package_name, &args.contract_name));
    let class_hash = declare_contract(&contract_file_path, &args.path);

    println!("Class hash: {}", class_hash);

    let constructor_args = parse_constructor_args(&args.constructor_inputs);
    deploy_contract(&class_hash, &constructor_args, &args.path);
}
