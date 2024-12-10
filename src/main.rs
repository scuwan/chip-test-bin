use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use aleo_program_service::get_program;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 10001);
    let getter = aleo_program_service::ProgramGetter::new(server_addr)
        .await
        .unwrap();
    if let Ok((program, consts)) = getter
        .get_program("ab1lnzh5cvsdnhtgcurh4gdeckw68k47f7ua0xfjln5nxwmfg5r9cpskff0vf".to_string())
        .await
    {
        println!("{:?} - {:?}", program, consts);
    }
    drop(getter);
    match get_program(
        server_addr,
        "ab1lnzh5cvsdnhtgcurh4gdeckw68k47f7ua0xfjln5nxwmfg5r9cpskff0vf".to_string(),
    )
    .await
    {
        Ok((program, consts)) => println!("{:?} - {:?}", program, consts),
        Err(error) => println!("{}", error.to_string()),
    }
    Ok(())
}
