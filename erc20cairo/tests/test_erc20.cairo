// use array::ArrayTrait;
// use result::ResultTrait;
// use starknet::get_caller_address;
// use starknet::ContractAddress;
// use starknet::ContractAddressZeroable;
// use super::ERC20;
// use core::testing;

// #[test]
// fn test_transfer() {
//     let contract_address = deploy_contract('ERC20', @ArrayTrait::new()).unwrap();
//     let caller = get_caller_address();

//     // Get initial balances
//     let initial_balance_sender = call(contract_address, 'balanceOf', @[caller]).unwrap();
//     let initial_balance_recipient = call(contract_address, 'balanceOf', @[contract_address]).unwrap();

//     // Transfer tokens
//     let mut invoke_calldata = ArrayTrait::new();
//     invoke_calldata.append(contract_address);
//     invoke_calldata.append(100);
//     invoke(contract_address, 'transfer', @invoke_calldata).unwrap();

//     // Get updated balances
//     let updated_balance_sender = call(contract_address, 'balanceOf', @[caller]).unwrap();
//     let updated_balance_recipient = call(contract_address, 'balanceOf', @[contract_address]).unwrap();

//     // Verify balances
//     assert(*initial_balance_sender.at(0_u32) == 100, 'Invalid initial sender balance');
//     assert(*initial_balance_recipient.at(0_u32) == 0, 'Invalid initial recipient balance');
//     assert(*updated_balance_sender.at(0_u32) == 0, 'Invalid updated sender balance');
//     assert(*updated_balance_recipient.at(0_u32) == 100, 'Invalid updated recipient balance');
// }

// #[test]
// fn test_approve_and_transfer_from() {
//     let contract_address = deploy_contract('ERC20', @ArrayTrait::new()).unwrap();
//     let caller = get_caller_address();

//     // Get initial balances
//     let initial_balance_owner = call(contract_address, 'balanceOf', @[caller]).unwrap();
//     let initial_balance_spender = call(contract_address, 'balanceOf', @[contract_address]).unwrap();

//     // Approve spender
//     let mut invoke_calldata_approve = ArrayTrait::new();
//     invoke_calldata_approve.append(contract_address);
//     invoke_calldata_approve.append(100);
//     invoke(contract_address, 'approve', @invoke_calldata_approve).unwrap();

//     // Get allowance
//     let allowance = call(contract_address, 'allowance', @[caller, contract_address]).unwrap();
//     assert(*allowance.at(0_u32) == 100, 'Invalid allowance');

//     // Transfer from owner's account
//     let mut invoke_calldata_transfer_from = ArrayTrait::new();
//     invoke_calldata_transfer_from.append(caller);
//     invoke_calldata_transfer_from.append(contract_address);
//     invoke_calldata_transfer_from.append(50);
//     invoke(contract_address, 'transferFrom', @invoke_calldata_transfer_from).unwrap();

//     // Get updated balances
//     let updated_balance_owner = call(contract_address, 'balanceOf', @[caller]).unwrap();
//     let updated_balance_spender = call(contract_address, 'balanceOf', @[contract_address]).unwrap();

//     // Verify balances
//     assert(*initial_balance_owner.at(0_u32) == 100, 'Invalid initial owner balance');
//     assert(*initial_balance_spender.at(0_u32) == 0, 'Invalid initial spender balance');
//     assert(*updated_balance_owner.at(0_u32) == 50, 'Invalid updated owner balance');
//     assert(*updated_balance_spender.at(0_u32) == 50, 'Invalid updated spender balance');
// }
