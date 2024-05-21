
use starknet::ContractAddress;
#[starknet::interface]
trait IErc20<TContractState> {
    
    fn name(self: @TContractState) -> felt252;
    fn symbol(self: @TContractState) -> felt252;
    fn decimals(self: @TContractState) -> u8;
    fn totalSupply(self: @TContractState) ->u256;
    fn balanceOf(self: @TContractState) -> u256;
    fn allowance(self: @TContractState, owner: ContractAddress, spender: ContractAddress) -> u256;
    fn transferFrom(ref self: TContractState, sender: ContractAddress, recipient: ContractAddress, amount: u256);
    fn mint(ref self: TContractState, recipient: ContractAddress, amount: u256);
    fn burn(ref self: TContractState, amount: u256);
    fn transfer(ref self: TContractState, recipient: ContractAddress, amount: u256);
    fn approve(ref self: TContractState, spender: ContractAddress, amount: u256);
    // TODO
    // fn increaseAllowance(ref self: TContractState, spender: ContractAddress, added_value: u256);
    // fn decreaseAllowance(ref self: TContractState, spender: ContractAddress, subtracted_value: u256);

}

#[starknet::contract]
mod Erc20{
    use starknet::ContractAddress;
    use starknet::contract_address_const;
    use starknet::get_caller_address;
    


        #[constructor]
        fn constructor(ref self: ContractState, _name : felt252, _symbol: felt252, _decimals: u8, initial_supply: u256, recipient: ContractAddress ){
            self.name.write(_name);
            self.symbol.write(_symbol);
            self.decimals.write(_decimals);
            self.total_supply.write(initial_supply);
            self.balances.write(recipient, initial_supply);
            Transfer(contract_address_const::<0>(), recipient, initial_supply);

        }
        #[derive(starknet::Store)]
        #[storage]
        struct Storage {
            name: felt252,
            symbol: felt252,
            owner: ContractAddress,
            decimals: u8,
            total_supply: u256,
            balances: LegacyMap::<ContractAddress, u256>,
            allowances: LegacyMap::<(ContractAddress, ContractAddress), u256>,
        }

    #[event]
    fn Transfer(from: ContractAddress, to: ContractAddress, value: u256) {}

    #[event]
    fn Approval(owner: ContractAddress, spender: ContractAddress, value: u256) {}


    fn _isowner(self: @ContractState) -> bool{
        let caller = get_caller_address();
        let _owner = self.owner.read();
        if caller != _owner{
            return false;
        }
        else{
            return true;
        }
    }

        #[abi(embed_v0)]
        impl erc20 of super::IErc20<ContractState> {


            fn name(self: @ContractState) -> felt252 {
                self.name.read()
            }
            fn symbol(self: @ContractState) -> felt252 {
                self.symbol.read()
            }

            fn decimals(self: @ContractState) -> u8 {
                self.decimals.read()
            }

            fn totalSupply(self: @ContractState) -> u256{
                self.total_supply.read()
            }

            fn balanceOf(self: @ContractState) -> u256{
            let caller = get_caller_address();

                self.balances.read(caller)
            }



            fn allowance(self: @ContractState, owner: ContractAddress, spender: ContractAddress) -> u256 {
                return self.allowances.read((owner, spender));
            }



                
        fn transferFrom(ref self: ContractState, sender: ContractAddress, recipient: ContractAddress, amount: u256) {
            let caller = get_caller_address();
            
            assert(!sender.is_zero(), 'ERC20: to 0' );
            assert(!recipient.is_zero(), 'ec20:transfer amount exceeded');
            let allowance = self.allowances.read((sender, caller));
            assert(amount <= allowance, 'transfer_amount_exceed_allowace');
            self.balances.write(sender, self.balances.read(sender) - amount);
            self.balances.write(recipient, self.balances.read(recipient) + amount);
            self.allowances.write((sender, caller), allowance - amount);
            Transfer(sender, recipient, amount);
        }

        
        fn mint(ref self: ContractState, recipient: ContractAddress, amount: u256) {
            assert(!recipient.is_zero(), 'ERC20: mint to the 0 address');
            let current_total_supply = self.total_supply.read();
            self.total_supply.write(current_total_supply + amount);
            self.balances.write(recipient, self.balances.read(recipient) + amount);
            Transfer(contract_address_const::<0x0>(), recipient, amount);
        }

        
        fn burn(ref self: ContractState, amount: u256) {
            let caller = get_caller_address();
            let caller_balance = self.balances.read(caller);
            assert(amount <= caller_balance, 'ERC20: burn amount exceeds');
            self.balances.write(caller, caller_balance - amount);
            self.total_supply.write(self.total_supply.read() - amount);
            Transfer(caller, contract_address_const::<0>(), amount);
        }

        fn transfer(ref self: ContractState, recipient: ContractAddress, amount: u256) {
            let sender = get_caller_address();
            transfer_helper(ref self, sender, recipient, amount);
        }


        fn approve(ref self: ContractState, spender: ContractAddress, amount: u256) {
                let owner = get_caller_address();
                approve_helper(ref self, owner, spender, amount);
            }


    }


        fn transfer_helper(ref self: ContractState, sender: ContractAddress, recipient: ContractAddress, amount: u256) {
            assert(!sender.is_zero(), 'ERC20: transfer from 0');
            assert(!recipient.is_zero(), 'ERC20: transfer to 0');
            self.balances.write(sender, self.balances.read(sender) - amount);
            self.balances.write(recipient, self.balances.read(recipient) + amount);
            Transfer(sender, recipient, amount);
        }

        fn approve_helper(ref self: ContractState, owner: ContractAddress, spender: ContractAddress, amount: u256) {
            assert(!owner.is_zero(), 'ERC20: approve from 0');
            assert(!spender.is_zero(), 'ERC20: approve to 0');
            self.allowances.write((owner, spender), amount);
            Approval(owner, spender, amount);
        }
    




}