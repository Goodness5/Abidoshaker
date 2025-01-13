use clap::Parser;
use serde_derive::Deserialize;
use std::{fs, path::PathBuf, process::{Command, exit}};
use toml;



#[derive(Parser, Clone)]
#[command(version = "1.0", about = "Automates the process of deploying contracts on StarkNet.")]
struct Arguments {
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

fn check_path_exists(path: &PathBuf) -> bool {
    path.exists()
}

fn generate_keystore(_target_dir: &PathBuf) {
    println!("Generating Keystore...");
    let generate_keystore_command = Command::new("starkli")
        .args(&["signer", "keystore", "from-key", "target/account0_keystore.json"])
        .output()
        .expect("Failed to generate account");
    if !generate_keystore_command.status.success() {
        eprintln!("Account generation failed");
    }
}

fn fetch_account(public_key: &str) {
    println!("Fetching generated account...");
    let _fetch_account_command = Command::new("starkli")
        .args(&[
            "account",
            "fetch",
            public_key,
            "--rpc",
            "https://free-rpc.nethermind.io/sepolia-juno",
            "--output",
            "target/account0_account.json",
        ])
        .status()
        .expect("Failed to fetch account");
}

fn declare_contract(contract_file_path: &PathBuf, base_dir: &PathBuf) -> String {
    println!("Declaring the contract...");

    // Verify that the contract file exists
    if !check_path_exists(contract_file_path) {
        eprintln!("Contract file not found: {:?}", contract_file_path);
        exit(1);
    }

    // Verify that the keystore file exists
    let keystore_path = base_dir.join("account0_keystore.json");
    if !check_path_exists(&keystore_path) {
        eprintln!("Keystore file not found: {:?}", keystore_path);
        exit(1);
    }

    // Verify that the account file exists
    let account_path = base_dir.join("account0_account.json");
    if !check_path_exists(&account_path) {
        eprintln!("Account file not found: {:?}", account_path);
        exit(1);
    }

    let mut binding = Command::new("starkli");
    let declare_command = binding
        .args(&[
            "declare",
            &contract_file_path.to_str().unwrap(),
            "--rpc",
            "https://free-rpc.nethermind.io/sepolia-juno",
            "--account",
            &account_path.to_str().unwrap(),
            "--keystore",
            &keystore_path.to_str().unwrap(),
            "--max-fee",
            "0.001"
        ]);

    println!("Declare command: {:?}", declare_command);

    let output = declare_command.output().expect("Failed to execute declare command");

    if !output.status.success() {
        eprintln!("Declare command failed: {}", String::from_utf8_lossy(&output.stderr));
        exit(1);
    }

    let output_str = String::from_utf8(output.stdout)
        .expect("Invalid UTF-8 sequence in command output")
        .trim()
        .to_string();

    println!("Declare output: {}", output_str);
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
        
        arg.replace(r#"\\(?=[^"])"#, "")
    })
    .collect()
}

fn deploy_contract(class_hash: &str, constructor_args: &[String], base_dir: &PathBuf) {
    println!("Deploying contract.....");

    // Prepare the deploy arguments as a mutable vector
    let mut deploy_args: Vec<&str> = vec![
        "deploy",
        "--account",
        "account0_account.json",
        class_hash,
        "--rpc",
        "https://free-rpc.nethermind.io/sepolia-juno",
        "--keystore",
        "account0_keystore.json",
    ];

    // Only add constructor arguments if they exist
    if !constructor_args.is_empty() {
        // Extend the deploy_args with the constructor arguments
        for arg in constructor_args {
            deploy_args.push(arg);
        }
    }

    println!("Deploy command: {:?}", deploy_args);

    let deploy_command = Command::new("starkli")
        .args(&deploy_args)
        .current_dir(base_dir)
        .status()
        .expect("Failed to deploy the contract");

    if !deploy_command.success() {
        eprintln!("Deploy command failed");
        exit(1);
    }
}

fn main() {
    let args = Arguments::parse();

    // Verify the path to the contract
    if !check_path_exists(&args.path) {
        eprintln!("Contract path not found: {:?}", args.path);
        exit(1);
    }

    let contents = read_scarb_toml(&args.path);
    let package_name = parse_package_name(&contents);

    compile_contract(&args.path);
    
    // Define the target directory
    let target_dir = PathBuf::from("target");
    
    generate_keystore(&target_dir); // Pass the target directory to the function
    fetch_account(&args.public_key);

    // Correctly construct the target_dev_dir path
    let target_dev_dir = PathBuf::from(format!("{}/target/dev", package_name));

    let contract_file_path = target_dev_dir.join(format!("{}_{}.contract_class.json", package_name, &args.contract_name));

    // Declare the contract
    let class_hash = declare_contract(&contract_file_path, &target_dir);

    println!("Class hash: {}", class_hash);

    let constructor_args = parse_constructor_args(&args.constructor_inputs);
    deploy_contract(&class_hash, &constructor_args, &target_dir);
}
