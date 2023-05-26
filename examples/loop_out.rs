// This example only fetches and prints the suggested swaps to the standard output similarly to
// `loop suggestSwap`.
//
// This program accepts three arguments: address, cert file, macaroon file
// The address must start with `https://`!

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

    let info = client.loopclient()
        // All calls require at least empty parameter
        .loop_out(tonic_lnd::looprpc::LoopOutRequest { 
            amt: 300000, 
            dest: "".to_string(), 
            max_swap_routing_fee: 10000, 
            max_prepay_routing_fee: 30000, 
            max_swap_fee: 10000, 
            max_prepay_amt: 30000, 
            max_miner_fee: 30000, 
            loop_out_channel: 0, 
            outgoing_chan_set: vec![869806056599388161], 
            sweep_conf_target: 20, 
            htlc_confirmations: 3, 
            swap_publication_deadline: 1684975824, 
            label: "first test".to_string(), 
            initiator: "LiT loop grpc".to_string() })
        .await
        .expect("failed to get info").into_inner();

    // We only print it here, note that in real-life code you may want to call `.into_inner()` on
    // the response to get the message.
    println!("{:#?}", info);
}
