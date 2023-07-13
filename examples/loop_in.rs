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
        .loop_in(tonic_lnd::looprpc::LoopInRequest { 
            amt: 25000000, 
            max_swap_fee: 50, 
            max_miner_fee: 10000, 
            last_hop: vec![], 
            external_htlc: false, 
            htlc_conf_target: 12, 
            label: "External swap".to_string(), 
            initiator: "LiT loop grpc".to_string(), 
            route_hints: vec![], 
            private: false, })
        .await
        .expect("failed to get info").into_inner();

    // We only print it here, note that in real-life code you may want to call `.into_inner()` on
    // the response to get the message.
    println!("{:#?}", info);
}
