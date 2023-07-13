// This example only fetches and prints the suggested swaps to the standard output similarly to
// `loop suggestSwap`.
//
// This program accepts three arguments: address, cert file, macaroon file
// The address must start with `https://`!

//https://stackoverflow.com/questions/66428638/how-do-i-reverse-a-hex-string
pub fn read_le_u8(input: Vec<u8>) -> Vec<u8> {
    let mut bytes_reversed = Vec::new();
    for i in input.iter().rev() {
        bytes_reversed.push(*i);
    }
    bytes_reversed
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

    let pubkey = "021c97a90a411ff2b10dc2a8e32de2f29d2fa49d41bfbb52bd416e460db0747d0d".to_string();
    let pubkey_bytes = hex::decode(&pubkey).expect("should get bytes");
    //let le_pubkey_bytes = read_le_u8(pubkey_bytes);

    let info = client.loopclient()
        // All calls require at least empty parameter
        .get_loop_in_quote(tonic_lnd::looprpc::QuoteRequest {
            amt:25000000, 
            conf_target: 12, 
            external_htlc: false, 
            swap_publication_deadline: 0, 
            loop_in_last_hop: vec![], 
            loop_in_route_hints: vec![], 
            private: false })
        .await
        .expect("failed to get info").into_inner();

    // We only print it here, note that in real-life code you may want to call `.into_inner()` on
    // the response to get the message.
    println!("{:#?}", info);
}
