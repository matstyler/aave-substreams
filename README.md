# Aave Substream Powered Subgraph

- Track **USDC** pool **supply/borrow** transactions 
- Follow `total supply` and `total borrow` state

<br/>

**Pool address**
```
0x87870Bca3F3fD6335C3F4ce8392D69350B4fA4E2
```
**Initial block**
```
16496939
```

<br/>

## Subgraph studio
https://thegraph.com/studio/subgraph/aave-substreams/playground



## Aave v3 - USDC pool page
https://app.aave.com/reserve-overview/?underlyingAsset=0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48&marketName=proto_mainnet_v3


<br/>

## Requirements

1. Rust - https://rustup.rs/

2. Graph-cli - https://thegraph.com/docs/en/quick-start/#2-install-the-graph-cli

3. substreams-cli - https://substreams.streamingfast.io/getting-started/installing-the-cli

<br/>

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

<br/>

## Structure

- `substreams.yaml` - The substrems manifest
- `src/modules` - The substream modules
- `src/abi` - The ABI files
- `src/constants.rs` - Contract addresses
- `src/calls.rs` - Contract calls
- `proto` - Messages structure


- `subgraph.yaml` - The subgraph manifest
- `schema.graphql` - The subgraph schema

<br/>

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