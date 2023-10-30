# Aave Substreams based Subgraph

- Track **USDC** pool **supply/borrow** transactions 
- Follow `total supply` and `total borrow` state



## Subgraph studio
https://thegraph.com/studio/subgraph/aave-substreams/playground



## Aave v3 - USDC pool page
https://app.aave.com/reserve-overview/?underlyingAsset=0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48&marketName=proto_mainnet_v3



## Requirements

1. Rust - https://rustup.rs/

2. Graph-cli - https://thegraph.com/docs/en/quick-start/#2-install-the-graph-cli

3. substreams-cli - https://substreams.streamingfast.io/getting-started/installing-the-cli



## Quickstart

Install graph-cli

```
yarn install
```

Build and package the substreams module

```
yarn substreams:prepare
```

Run the substreams locally

```
yarn substreams:stream
```

Deploy the subgraph

```
yarn subgraph:deploy
```



## Structure

- `substreams.yaml` - The substrems manifest
- `src/modules` - The substream modules
- `src/abi` - The ABI files
- `src/constants.rs` - Contract addresses
- `src/calls.rs` - Contract calls
- `proto` - Messages structure


- `subgraph.yaml` - The subgraph manifest
- `schema.graphql` - The subgraph schema



## Example query (get all the index transfers and pool state)

```graphql
{
    pools {
        id
        last_transfer
        total_supply
        total_borrow
        transfers {
            id
            type
            amount
        }
    }
}
```