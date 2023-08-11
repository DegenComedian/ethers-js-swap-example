use ethers::prelude::*;
use ethers::utils::{parse_units, format_units};
use std::sync::Arc;
use log::info;
use ethers::utils::Anvil;
use std::time::{SystemTime, UNIX_EPOCH};


const UNISWAP_V2_ROUTER_ADDRESS: &str = "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D";
const MIM_ADDRESS: &str = "0x99D8a9C45b2ecA8864373A26D1459e3Dff1e17F3";
const WETH_ADDRESS: &str = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";   

fn get_epoch_milliseconds() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    const RPC_URL: &str = "https://eth.llamarpc.com";

    let mim_address: Address = MIM_ADDRESS.parse()?;
    let weth_address: Address = WETH_ADDRESS.parse()?;
    let uniswap_v2_router_address: Address = UNISWAP_V2_ROUTER_ADDRESS.parse()?;

    let anvil = Anvil::new().fork(RPC_URL).spawn();
    info!("Anvil running at `{}`", anvil.endpoint());
    //let provider = Provider::<Http>::try_from("http://localhost:8545")?;

    let provider = Provider::<Http>::try_from(anvil.endpoint())?;
    let chain_id = provider.get_chainid().await.unwrap().as_u64();
    info!("chain id: {chain_id}");

    let accounts = provider.get_accounts().await?;
    let client = Arc::new(provider);
    let wallet_address = accounts[0];
    let user_address = accounts[1];

    abigen!(IERC20, "./abi/IERC20.json");
    abigen!(IUNISWAPV2ROUTER, "./abi/IUNISWAPV2ROUTER.json");

    let router_contract = IUNISWAPV2ROUTER::new(uniswap_v2_router_address, client.clone());
    let mim_contract = IERC20::new(mim_address, client.clone());
    
    let weth_amount_in = parse_units(0.1, 18)?.into();

    let amounts_out = router_contract.get_amounts_out(weth_amount_in, [weth_address, mim_address].to_vec()).call().await?;
    println!("proposed amount of MIM to buy in wei {} for 0.1 ETH", amounts_out[1]);

    let deadline = U256::from(get_epoch_milliseconds()) + U256::from(60 * 1000);

    let eth_max_spend = parse_units(1, 18)?;

    let tx_receipt = router_contract.swap_eth_for_exact_tokens(amounts_out[1], [weth_address, mim_address].to_vec(), user_address, deadline )
    .value(eth_max_spend)
    .from(wallet_address)
    .gas(U256::from(50_000)) // this is crucial otherwise tx will get reverted without a reason
    .send()
    .await?
    .await?
    .unwrap();

    println!("tx receipt: {:?}", tx_receipt);

    info!("wallet balance of ETH in ether after first swap: {}", format_units(client.get_balance(wallet_address, Option::None).await?, 18)?);

    if let Ok(mim_balance) = mim_contract.balance_of(user_address).call().await {
        info!("MIM balance for user after first swap: {}", mim_balance);
    }
   

    // second try, using swapExactETHForTokens
    let tx_receipt2 = router_contract.swap_exact_eth_for_tokens(weth_amount_in, [weth_address, mim_address].to_vec(), user_address, deadline )
    .value(weth_amount_in)
    .from(wallet_address)
    .gas(U256::from(50_000)) // this is crucial otherwise tx will get reverted without a reason
    .send()
    .await?
    .await?
    .unwrap(); 

    println!("tx receipt 2: {:?}", tx_receipt2);

    info!("wallet balance of ETH in ether after second swap: {}", format_units(client.get_balance(wallet_address, Option::None).await?, 18)?);

    if let Ok(mim_balance) = mim_contract.balance_of(user_address).call().await {
        info!("MIM balance for user after second swap: {}", mim_balance);
    }
   

    Ok(())
}
