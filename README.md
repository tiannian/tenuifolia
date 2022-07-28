# tenuifolia

> A blockchain consensus framework.

## Usage

### For Native

Support node type:

- Validator
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

- Validator
- Fullnode
- Lightnode

### API Model

#### Mempool

#### BlockPacker

#### BlockExecutor

#### ConsensusHook

