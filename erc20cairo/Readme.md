# ERC20 Token Contract

This is an ERC20 token contract implemented in the Cairo programming language. The ERC20 standard is a widely adopted interface for fungible tokens on the Ethereum blockchain.

## Contract Overview

The ERC20 contract provides basic functionality for managing and transferring tokens. It supports the following features:

- Token name and symbol retrieval.
- Decimal places configuration for token precision.
- Total supply of tokens in circulation.
- Token balances of individual accounts.
- Transfer of tokens between accounts.
- Approval mechanism for delegated token transfers.
- Allowance tracking for approved token transfers.
- Minting of new tokens by the contract owner.
- Burning of tokens by token holders.

## Contract Structure

The contract is structured as follows:

- The `Storage` struct stores the contract's state variables, including the token name, symbol, decimals, total supply, balances, and allowances.
- The `Transfer` and `Approval` events are emitted to notify listeners about token transfers and approvals.
- The `constructor` function initializes the contract by setting the initial token supply and assigning it to the recipient.
- Basic getter functions are provided to retrieve the token name, symbol, decimals, total supply, and balance of an account.
- The `transfer` function allows token holders to transfer tokens to another account.
- The `approve` function enables token holders to approve a spender to transfer a specific amount of tokens on their behalf.
- The `allowance` function retrieves the approved token transfer allowance between an owner and a spender.
- The `transferFrom` function allows a spender to transfer tokens from the owner's account, given the necessary approval.
- The `mint` function allows the contract owner to create and assign new tokens to a recipient.
- The `burn` function enables token holders to burn a specific amount of their own tokens, reducing both their balance and the total token supply.
