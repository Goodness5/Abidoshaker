use clap::Parser;
use std::{path::PathBuf, process::{Command, Output}};

#[derive(Parser)]
#[command(version = "1.0", about = "Automates the process of deploying contracts on StarkNet.")]
struct Arguments {
    /// Path to the directory containing the contract to deploy.
    #[arg(short = 'p', long)]
    path: PathBuf,

    /// Pattern to match against the contract files.
    #[arg(short = 't', long)]
    pattern: String,

    /// API key for Infura.
    #[clap(short, long)]
    infura_api_key: String,

    /// API key for Alchemy.
    #[clap(short, long)]
    alchemy_api_key: String,
}

fn main() {
    let args = Arguments::parse();

    // Compile the contract
    println!("Compiling the contract...");
    let filename = args.path.join("src");
    Command::new("scarb")
        .arg("build")
        .current_dir(filename.clone())
        .status()
        .expect("Failed to compile the contract");

    // Generate the account
    println!("Generating account...");
    let generate_account = Command::new("starkli")
        .args(&["signer", "keystore", "from-key", "account0_keystore.json"])
        .current_dir(filename.clone())
        .output()
        .expect("Failed to generate account");

    // Extract the public key from the output
    let public_key = extract_public_key(&generate_account);
    println!("Public key: {}", public_key);

    // Fetch the generated account
    // Function to extract the public key from the command output
    fn extract_public_key(output: &Output) -> String {
        let output_str = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = output_str.split('\n').collect();
        let public_key_line = lines.iter().find(|line| line.starts_with("Public key:"));
    
        if let Some(public_key_line) = public_key_line {
            let parts: Vec<&str> = public_key_line.split(':').collect();
            if let Some(public_key) = parts.get(1) {
                return public_key.trim().to_string();
            }
        }
    
        panic!("Failed to extract public key from output");
    }
    println!("Fetching generated account...");
    Command::new("starkli")
        .args(&[
            "account",
            "fetch",
            &public_key,
            "--rpc",
            &format!("https://starknet-goerli.infura.io/v3/{}", args.infura_api_key),
            "--output",
            "account0_account.json",
            ])
        .current_dir(filename.clone())
        .status()
        .expect("Failed to fetch account");

    // Declare the contract
    println!("Declaring the contract...");
    Command::new("starkli")
        .args(&[
            "declare",
            &format!("{}/contract_class.json", args.path.display()),
            "--rpc",
            &format!("https://starknet-goerli.infura.io/v3/{}", args.infura_api_key),
        ])
        .current_dir(filename.clone())
        .status()
        .expect("Failed to declare the contract");

    // Deploy the contract
    println!("Deploying the contract...");
    Command::new("starkli")
        .args(&[
            "deploy",
            "CLASS_HASH_DECLARED", // Replace CLASS_HASH_DECLARED with the actual class hash
            "--rpc",
            &format!("https://starknet-goerli.alchemyapi.io/v2/{}", args.alchemy_api_key),
        ])
        .current_dir(filename)
        .status()
        .expect("Failed to deploy the contract");

    println!("Deployment completed.");
}

