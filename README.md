# tenuifolia

> A blockchain consensus framework.

## Usage

### For Native

Support node type:

- Seednode
- Voternode
- Fullnode
- Lightnode

Add into `Cargo.toml`:

```toml
tenuifolia = "0.1"
```

### In npm package

Support node type:

- Lightnode

Add into `Cargo.toml`:

```toml
tenuifolia = { version = "0.1", feature = ["browser"] }
```

## Interface Model

### Node Type

`tenuifolia` support the following node type:

- Seednode: provides p2p services, including bootstrapping and NAT traversal.
- Voternode: do consensus for blockchain.
- Fullnode: execute all transaction.
- Lightnode: only execute partial transaction.

### API Model

#### Mempool

#### BlockPacker

#### BlockExecutor

#### ConsensusHook

