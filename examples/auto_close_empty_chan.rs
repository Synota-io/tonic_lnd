// This example only fetches and prints the node info to the standard output similarly to
// `lncli getinfo`.
//
// This program accepts three arguments: address, cert file, macaroon file
// The address must start with `https://`!

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use tonic_lnd::lnrpc::{ChannelPoint, channel_point::FundingTxid};

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

    let channels = client
    .lightning()
    // All calls require at least empty parameter
    .list_channels(tonic_lnd::lnrpc::ListChannelsRequest {
        active_only: false,
        inactive_only: false,
        public_only: false,
        private_only: false,
        peer: vec![],
    })
    .await
    .expect("failed to get wallet balance").into_inner();

    //close channel if local balance is less than 10% of capacity or less than 10k sats
    for channel in channels.channels {
        let channel_remaining = Decimal::from(channel.local_balance) / Decimal::from(channel.capacity);
        if (channel_remaining < dec!(0.1)) || (channel.local_balance < 10000) {
            if let Some((funding_txid, output_index)) = channel.channel_point.split_once(':'){
                let close_chan_req = tonic_lnd::lnrpc::CloseChannelRequest {
                    channel_point: Some(ChannelPoint {
                        funding_txid: Some(FundingTxid::FundingTxidStr(funding_txid.to_string())),
                        output_index: output_index.parse().expect("Expected to parse as u32"),
                    }),
                    force: false,
                    target_conf: 0,
                    sat_per_vbyte: 2,
                    max_fee_per_vbyte: 10,
                    delivery_address: "".to_string(),
                };
            
                client
                    .lightning()
                    .close_channel(close_chan_req)
                    .await
                    .expect("expected to be able to close empty channels (active only)");
            }
        }
    }

    // We only print it here, note that in real-life code you may want to call `.into_inner()` on
    // the response to get the message.
    //println!("{:#?}", info);
}
