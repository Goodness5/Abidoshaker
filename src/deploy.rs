use cairo_rs::contract::Contract;
use cairo_rs::deploy::DeployOptions;


// struct
// Function to deploy a Cairo contract
pub fn deploy_cairo_contract(contract_code: &str, options: DeployOptions) -> Result<Contract, String> {
    // ... existing code ...

    // Create a new contract instance
    let contract = Contract::new(contract_code, options)?;

    // Deploy the contract
    contract.deploy()?;

    // Return the deployed contract
    Ok(contract)
}
