
#[pallet::genesis_config]
pub struct GenesisConfig<T: Config> {
    genesis_utxos: Vec<TransactionOutput>,
}

#[pallet::genesis_build]
impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
    fn build(&self) {
        
        // UtxoStore build(|config: &GenesisConfig| {
        //     config.genesis_utxos
        //         .iter()
        //         .cloned()
        //         .map(|u| (BlakeTwo256::hash_of(&u), u))
        //         .collect::<Vec<_>>()
        // }): map hasher(identity) H256 => Option<TransactionOutput>;


    }
}

impl<T: Config> Default for GenesisConfig<T, I> {
    fn default() -> Self {
        GenesisConfig {
            genesis_utxos: vec![]
        }
    }
}
