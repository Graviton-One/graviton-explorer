use web3;

enum ChainID {
    Ethereum = 1,
    Binance = 56,
    Fantom, // TODO
}

enum DEXName {
    Uniswap,
    Spirit,
    Spooky,
    Sushi,
    Pancake,
    Raydium,
    Serum
}

pub struct DEXPool<'a> {
    pub address: &'a str,
    pub pair: &'a str,
}

pub struct DEXPools<'a> {
    pub chain_id: ChainID,
    pub pools: Vec<DEXPool<'a>>,
    pub name: DEXName,
}



pub struct PoolList;

impl PoolList {
    const Uniswap: &'static DEXPools<'static> = &DEXPools {
        chain_id: ChainID::Ethereum,
        name: DEXName::Uniswap,
        pools: vec![
            DEXPool {
                pair: "GTON/USDC",
                address: "0xE40a2eAB69D4dE66BcCb0Ac8E2517a230c6312E8",
            },
        ],
    };
    const Sushi: &'static DEXPools<'static> = &DEXPools {
        chain_id: ChainID::Ethereum,
        name: DEXName::Uniswap,
        pools: vec![
            DEXPool {
                pair: "GTON/WETH",
                address: "0xBA38eca6DFdB92EC605C4281C3944fCcD9DeC898",
            },
        ],
    };
    const Spooky: &'static DEXPools<'static> = &DEXPools {
        chain_id: ChainID::Fantom,
        name: DEXName::Spooky,
        pools: vec![
            DEXPool {
                pair: "GTON/USDC",
                address: "0xcf9f857ffe6ff32b41b2a0d0b4448c16564886de",
            },
            DEXPool {
                pair: "GTON/FTM",
                address: "0xb9b452a71dd1cfb4952d90e03bf701a6c7ae263b",
            },
        ],
    };
    const Spirit: &'static DEXPools<'static> = &DEXPools {
        chain_id: ChainID::Fantom,
        name: DEXName::Spirit,
        pools: vec![
            DEXPool {
                pair: "GTON/FTM",
                address: "0x25F5B3840D414a21c4Fc46D21699e54d48F75FDD",
            },
            DEXPool {
                pair: "GTON/USDC",
                address: "0x8a5555c4996B72E5725Cf108Ad773Ce5E715DED4",
            },
            DEXPool {
                pair: "GTON/fUSDT",
                address: "0x070AB37714b96f1A938e75CAbbb64ED5F5748170",
            },
        ],
    };
    const Pancake: &'static DEXPools<'static> = &DEXPools {
        chain_id: ChainID::Binance,
        name: DEXName::Pancake,
        pools: vec![
            DEXPool {
                pair: "GTON/BUSD",
                address: "0xbe2c760aE00CbE6A5857cda719E74715edC22279",
            },
            DEXPool {
                pair: "GTON/WBNB",
                address: "0xA216571b69dd69600F50992f7c23b07B1980CfD8",
            },
        ],
    };

    // const UNISWAP_GTON_USDC: &'static str = "0xE40a2eAB69D4dE66BcCb0Ac8E2517a230c6312E8";
    // const SUSHI_GTON_WETH: &'static str = "0xBA38eca6DFdB92EC605C4281C3944fCcD9DeC898";
    // const SPOOKY_GTON_USDC: &'static str = "0xcf9f857ffe6ff32b41b2a0d0b4448c16564886de";
    // const SPOOKY_GTON_FTM: &'static str = "0xb9b452a71dd1cfb4952d90e03bf701a6c7ae263b";
    // const SPIRIT_GTON_FTM: &'static str = "0x25F5B3840D414a21c4Fc46D21699e54d48F75FDD";
    // const SPIRIT_GTON_USDC: &'static str = "0x8a5555c4996B72E5725Cf108Ad773Ce5E715DED4";
    // const SPIRIT_GTON_FUSDT: &'static str = "0x070AB37714b96f1A938e75CAbbb64ED5F5748170";
    // const PANCAKE_GTON_BUSD: &'static str = "0xbe2c760aE00CbE6A5857cda719E74715edC22279";
    // const PANCAKE_GTON_BNB: &'static str = "0xA216571b69dd69600F50992f7c23b07B1980CfD8";

    pub fn get_all_gton() -> Vec<&'static DEXPools<'static>> {
        return vec![
            Self::Uniswap,
            Self::Spooky,
            Self::Sushi,
            Self::Spirit,
            Self::Pancake,
        ]
    }
}

pub struct PoolReserves {

}

pub struct PoolsProvider;

impl PoolsProvider {
    pub async fn get_pool_reserves() -> Result<(), Box<dyn std::error::Error>> {
        let transport = web3::transports::Http::new("http://localhost:8545")?;
        let web3 = web3::Web3::new(transport);
    
        println!("Calling accounts.");
        let mut accounts = web3.eth().accounts().await?;
        println!("Accounts: {:?}", accounts);
        accounts.push("00a329c0648769a73afac7f9381e08fb43dbea72".parse().unwrap());
    
        println!("Calling balance.");
        for account in accounts {
            let balance = web3.eth().balance(account, None).await?;
            println!("Balance of {:?}: {}", account, balance);
        }
        
        Ok(())
    }
}