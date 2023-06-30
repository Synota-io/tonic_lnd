use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-env-changed=LND_REPO_DIR");
    let dir = match std::env::var_os("LND_REPO_DIR") {
        Some(lnd_repo_path) => {
            let mut lnd_rpc_dir = PathBuf::from(lnd_repo_path);
            lnd_rpc_dir.push("lnrpc");
            lnd_rpc_dir
        },
        None => PathBuf::from("vendor"),
    };

    let lnd_rpc_proto_file = dir.join("lightning.proto");
    println!("cargo:rerun-if-changed={}", lnd_rpc_proto_file.display());

    let protos = [
        "signrpc/signer.proto",
        "walletrpc/walletkit.proto",
        "routerrpc/router.proto",
        "lightning.proto",
    ];

    let proto_paths: Vec<_> = protos
        .iter()
        .map(|proto| {
            let proto_path = dir.join(proto);
            proto_path
        })
        .collect();

    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        //ListChannels fields
        .type_attribute("ListChannelsResponse", "#[derive(serde::Deserialize, serde::Serialize)]")
        .type_attribute("Channel", "#[derive(serde::Deserialize, serde::Serialize)]")
        .type_attribute("HTLC", "#[derive(serde::Deserialize, serde::Serialize)]")
        .type_attribute("CommitmentType", "#[derive(serde::Deserialize, serde::Serialize)]")
        .type_attribute("ChannelConstraints", "#[derive(serde::Deserialize, serde::Serialize)]")
        //ClosedChannels fields
        .type_attribute("ClosedChannelsResponse", "#[derive(serde::Deserialize, serde::Serialize)]")
        .type_attribute("ChannelCloseSummary", "#[derive(serde::Deserialize, serde::Serialize)]")
        .type_attribute("ClosureType", "#[derive(serde::Deserialize, serde::Serialize)]")
        .type_attribute("Initiator", "#[derive(serde::Deserialize, serde::Serialize)]")
        .type_attribute("Resolution", "#[derive(serde::Deserialize, serde::Serialize)]")
        .type_attribute("ResolutionType", "#[derive(serde::Deserialize, serde::Serialize)]")
        .type_attribute("ResolutionOutcome", "#[derive(serde::Deserialize, serde::Serialize)]")
        .type_attribute("OutPoint", "#[derive(serde::Deserialize, serde::Serialize)]")
        //GetTransactions fields
        .type_attribute("TransactionDetails", "#[derive(serde::Deserialize, serde::Serialize)]")
        .type_attribute("Transaction", "#[derive(serde::Deserialize, serde::Serialize)]")
        .type_attribute("OutputDetail", "#[derive(serde::Deserialize, serde::Serialize)]")
        .type_attribute("PreviousOutPoint", "#[derive(serde::Deserialize, serde::Serialize)]")
        .type_attribute("OutputScriptType", "#[derive(serde::Deserialize, serde::Serialize)]")
        //ChannelBalance fields
        .type_attribute("ChannelBalanceResponse", "#[derive(serde::Deserialize, serde::Serialize)]")
        .type_attribute("Amount", "#[derive(serde::Deserialize, serde::Serialize)]")
        //WalletBalance fields
        .type_attribute("WalletBalanceResponse", "#[derive(serde::Deserialize, serde::Serialize)]")
        .type_attribute("AccountBalanceEntry", "#[derive(serde::Deserialize, serde::Serialize)]")
        .type_attribute("WalletAccountBalance", "#[derive(serde::Deserialize, serde::Serialize)]")
        //ListSwaps fields
        .type_attribute("ListSwapsResponse", "#[derive(serde::Deserialize, serde::Serialize)]")
        .type_attribute("SwapStatus", "#[derive(serde::Deserialize, serde::Serialize)]")
        .type_attribute("SwapType", "#[derive(serde::Deserialize, serde::Serialize)]")
        .type_attribute("SwapState", "#[derive(serde::Deserialize, serde::Serialize)]")
        .type_attribute("FailureReason", "#[derive(serde::Deserialize, serde::Serialize)]")
        //GetLsatTokens fields
        .type_attribute("TokensResponse", "#[derive(serde::Deserialize, serde::Serialize)]")
        .type_attribute("LsatToken", "#[derive(serde::Deserialize, serde::Serialize)]")
        .format(false)
        .compile(&proto_paths, &[dir])?;
    Ok(())
}
