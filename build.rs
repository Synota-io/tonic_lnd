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
        .type_attribute("ListChannelsResponse", "#[derive(serde::Deserialize, serde::Serialize)]")
        .type_attribute("Channel", "#[derive(serde::Deserialize, serde::Serialize)]")
        .type_attribute("HTLC", "#[derive(serde::Deserialize, serde::Serialize)]")
        .type_attribute("CommitmentType", "#[derive(serde::Deserialize, serde::Serialize)]")
        .type_attribute("ChannelConstraints", "#[derive(serde::Deserialize, serde::Serialize)]")
        .format(false)
        .compile(&proto_paths, &[dir])?;
    Ok(())
}
