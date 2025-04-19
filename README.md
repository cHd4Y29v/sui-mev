# Sui MEV Bot


## Run 
Start the bot with your private key.

```bash
export SUI_RPC_URL=https://fullnode.mainnet.sui.io:443
cargo run -r --bin arb start-bot --private-key {}
```

## Supports

- BlueMove
- FlowX
- Aftermath
- Cetus 
- Kriya
- Abex
- Navi
- Turbos
- Deepbook
- Shio

## Relay
If you have a validator, you can let the validator push mempool transactions to your relay, which then send to the bot.

```bash
cargo run -r --bin relay
```