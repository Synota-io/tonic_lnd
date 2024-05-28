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
        "faraday.proto",
        "looprpc/client.proto",
        "invoicesrpc/invoices.proto",
        "walletunlocker.proto",
        "stateservice.proto",
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
        //WalletUnlocker fields
        .type_attribute("GenSeedResponse", "#[derive(serde::Deserialize, serde::Serialize)]")
        .type_attribute("UnlockWalletRequest", "#[derive(serde::Deserialize, serde::Serialize)]")
        .type_attribute("UnlockWalletResponse", "#[derive(serde::Deserialize, serde::Serialize)]")
        .type_attribute("ChanBackupSnapshot", "#[derive(serde::Deserialize, serde::Serialize)]")
        .type_attribute("MultiChanBackup", "#[derive(serde::Deserialize, serde::Serialize)]")
        .type_attribute("ChannelBackups", "#[derive(serde::Deserialize, serde::Serialize)]")
        .type_attribute("ChannelBackup", "#[derive(serde::Deserialize, serde::Serialize)]")
        .type_attribute("ChannelPoint", "#[derive(serde::Deserialize, serde::Serialize)]")
        .type_attribute("funding_txid", "#[derive(serde::Deserialize, serde::Serialize)]")
        //StateService fields
        .type_attribute("GetState", "#[derive(serde::Deserialize, serde::Serialize)]")
        .type_attribute("SubscribeState", "#[derive(serde::Deserialize, serde::Serialize)]")
        //PendingChannels fields
        .type_attribute("PendingChannelsResponse", "#[derive(serde::Deserialize, serde::Serialize)]")
        .type_attribute("Commitments", "#[derive(serde::Deserialize, serde::Serialize)]")
        .type_attribute("PendingOpenChannel", "#[derive(serde::Deserialize, serde::Serialize)]")
        .type_attribute("PendingChannel", "#[derive(serde::Deserialize, serde::Serialize)]")
        .type_attribute("ClosedChannel", "#[derive(serde::Deserialize, serde::Serialize)]")
        .type_attribute("ForceClosedChannel", "#[derive(serde::Deserialize, serde::Serialize)]")
        .type_attribute("PendingHTLC", "#[derive(serde::Deserialize, serde::Serialize)]")
        .type_attribute("AnchorState", "#[derive(serde::Deserialize, serde::Serialize)]")
        .type_attribute("WaitingCloseChannel", "#[derive(serde::Deserialize, serde::Serialize)]")
        .format(false)
        .compile(&proto_paths, &[dir])?;
    Ok(())
}
