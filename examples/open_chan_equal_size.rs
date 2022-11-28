// This example only fetches and prints the node info to the standard output similarly to
// `lncli getinfo`.
//
// This program accepts three arguments: address, cert file, macaroon file
// The address must start with `https://`!

use rust_decimal::{Decimal, prelude::ToPrimitive};
use rust_decimal_macros::dec;
use tonic_lnd::{lnrpc::{LightningAddress, WalletBalanceResponse, BatchOpenChannelResponse}, Client, walletrpc::{AddrResponse, ListUnspentResponse}};

use hex::decode;
use rand::{distributions::Alphanumeric, Rng, RngCore};

pub fn create_nonce_32() -> [u8; 32] {
    let mut data: [u8; 32] = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut data);

    data
}

pub async fn batch_open_channels_equal_size(
    client: &mut Client,
    max_channel_size: i64,
    fee_rate: i64,
    peer_pubkey: String,
) -> BatchOpenChannelResponse {
    let wallet_balance = get_onchain_balance(client).await.confirmed_balance;
    println!("Wallet Balance: {}", wallet_balance);
    let num_utxos = list_unspent_utxos(client, 1, 0, "".to_string(), false)
        .await
        .utxos
        .len();
    println!("Num utxos: {}", num_utxos);
    let mut num_channels = (Decimal::from(wallet_balance) / Decimal::from(max_channel_size)).ceil();
    println!("num_channels: {}", num_channels);

    //https://bitcoinops.org/en/tools/calc-size/
    //overhead + p2wpkh_inputs*68 + 31(p2wpkh_change_output) + p2wsh_channels*43
    let mut fee_estimate =
        (dec!(10.5) + Decimal::from(num_utxos) * dec!(68) + num_channels * dec!(43))
            * Decimal::from(fee_rate);
    println!("fee_estimate: {}", fee_estimate);

    //need to account for fees when creating channels
    let mut estimated_wallet_balance_for_funding = Decimal::from(wallet_balance) - fee_estimate;
    println!(
        "estimated_wallet_balance_for_funding: {}",
        estimated_wallet_balance_for_funding
    );

    if (estimated_wallet_balance_for_funding / (num_channels - dec!(1)))
        < Decimal::from(max_channel_size)
    {
        num_channels -= dec!(1);
        fee_estimate = (dec!(10.5) + Decimal::from(num_utxos) * dec!(68) + num_channels * dec!(43))
            * Decimal::from(fee_rate);
        estimated_wallet_balance_for_funding = Decimal::from(wallet_balance) - fee_estimate;
        println!("num_channels: {}", num_channels);
        println!("fee_estimate: {}", fee_estimate);
        println!(
            "estimated_wallet_balance_for_funding: {}",
            estimated_wallet_balance_for_funding
        );
    }

    let channel_size = (estimated_wallet_balance_for_funding / num_channels)
        .floor()
        .to_i64()
        .expect("Decimal should fit in i64");
    println!("channel_size: {}", channel_size);
    let mut channel_vec: Vec<tonic_lnd::lnrpc::BatchOpenChannel> = Vec::new();
    for _ in 0..num_channels.to_i32().expect("Decimal should fit in i32") {
        let channel = tonic_lnd::lnrpc::BatchOpenChannel {
            node_pubkey: decode(&peer_pubkey).unwrap(),
            local_funding_amount: channel_size,
            push_sat: 0,
            private: true,
            min_htlc_msat: 10,
            remote_csv_delay: 0,
            close_address: get_next_addr(client, "".to_string(), 4, false).await.addr,
            pending_chan_id: create_nonce_32().to_vec(),
            commitment_type: 2,
        };

        channel_vec.push(channel);
    }

    client
        .lightning()
        .batch_open_channel(tonic_lnd::lnrpc::BatchOpenChannelRequest {
            channels: channel_vec,
            target_conf: 0,
            sat_per_vbyte: fee_rate,
            min_confs: 1,
            spend_unconfirmed: false,
            label: "Opening Channel(s)".to_string(),
        })
        .await
        .expect("failed to open channel(s) or receive txid(s)")
        .into_inner()

}

pub async fn get_onchain_balance(client: &mut Client) -> WalletBalanceResponse {
    client
        .lightning()
        .wallet_balance(tonic_lnd::lnrpc::WalletBalanceRequest {})
        .await
        .expect("failed to get wallet balance")
        .into_inner()
}

pub async fn get_next_addr(
    client: &mut Client,
    account: String,
    r#type: i32,
    change: bool,
) -> AddrResponse {
    client
        .wallet()
        // All calls require at least empty parameter
        .next_addr(tonic_lnd::walletrpc::AddrRequest {
            account,
            r#type,
            change,
        })
        .await
        .expect("failed to get next address")
        .into_inner()
}

pub async fn list_unspent_utxos(
    client: &mut Client,
    min_confs: i32,
    max_confs: i32,
    account: String,
    unconfirmed_only: bool,
) -> ListUnspentResponse {
    client
        .wallet()
        // All calls require at least empty parameter
        .list_unspent(tonic_lnd::walletrpc::ListUnspentRequest {
            min_confs,
            max_confs,
            account,
            unconfirmed_only,
        })
        .await
        .expect("failed to get utxos")
        .into_inner()
}

#[tokio::main]
async fn main() {
    let mut args = std::env::args_os();
    args.next().expect("not even zeroth arg given");
    let address = args.next().expect("missing arguments: address, cert file, macaroon file");
    let cert_file = args.next().expect("missing arguments: cert file, macaroon file");
    let macaroon_file = args.next().expect("missing argument: macaroon file");
    let address = address.into_string().expect("address is not UTF-8");

    // Connecting to LND requires only address, cert file, and macaroon file
    let mut client = tonic_lnd::connect(address, cert_file, macaroon_file)
        .await
        .expect("failed to connect");

    let info = batch_open_channels_equal_size(
        &mut client,
        49000,
        1,
        "039819ecdcfd3bd78ef7574db028a4d6ea13acd5f45a903b9e85c3d9d2b76a6e96".to_string(),
    )
    .await;

    // We only print it here, note that in real-life code you may want to call `.into_inner()` on
    // the response to get the message.
    println!("{:#?}", info);
}
