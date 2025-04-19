mod arb;
mod collector;
mod common;
mod config;
mod defi;
mod executor;
mod pool_ids;
mod start_bot;
mod strategy;
mod types;

use clap::Parser;
use eyre::Result;

// readnote1: &str 像 String 类型的“引用”（它不拥有内容）；String 像 StringBuilder，你可以动态拼接和构造
pub const BUILD_VERSION: &str = version::build_version!();

// readnote2: derive会自动实现trait。这里用的clap的Parser（命令行解析器）
#[derive(clap::Parser, Debug)]
pub struct Args {
    // readnote3: 这里的用的clap的命令解析，将命令解析成subcommand
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Clone, Debug, Parser)]
#[command(about = "Common configuration")]
pub struct HttpConfig {
    #[arg(long, env = "SUI_RPC_URL", default_value = "http://localhost:9000")]
    pub rpc_url: String,

    #[arg(long, help = "deprecated")]
    pub ipc_path: Option<String>,
}

// readnote4: 驼峰自动转成"-"分隔，命令行里就是start-bot
#[derive(clap::Subcommand, Debug)]
pub enum Command {
    StartBot(start_bot::Args),
    Run(arb::Args),
    /// Generate a file with objectIDs of all pools and their underlying objects
    PoolIds(pool_ids::Args),
}

// readnote5: 异步运行主函数
#[tokio::main]
async fn main() -> Result<()> {
    let terminal_args = Args::parse();

    match terminal_args.command {
        Command::StartBot(args) => start_bot::run(args).await,
        Command::Run(args) => arb::run(args).await,
        Command::PoolIds(args) => pool_ids::run(args).await,
    }
}
