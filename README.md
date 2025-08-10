# Toy CEX - Centralized Exchange Implementation

A Rust-based implementation of a centralized exchange with order book management and matching engine, designed for learning high-performance systems programming.

## Performance

- **Throughput**: 100,000+ trades per second on M4 Mac Pro
- **Concurrent Users**: 10,000+ trading agents
- **Scalability**: Handles high-frequency trading scenarios

## Architecture

- **Order Book**: Price-time priority matching with FIFO queues
- **Matching Engine**: Real-time order matching and execution
- **Ledger System**: Atomic balance management and trade settlement
- **Async Runtime**: Tokio-based concurrent processing
- **Price-Time Priority**: Orders matched by best price, then timestamp
- **Atomic Operations**: All balance updates and settlements are atomic


## Order Book Implementation

The order book uses B-tree maps for efficient price level management:

- **Bids**: Stored in descending price order (highest bid first)
- **Asks**: Stored in ascending price order (lowest ask first)
- **FIFO Queues**: Orders at the same price level maintain time priority
- **Immediate Execution**: Marketable orders execute instantly against resting orders

## Ledger System

- **Multi-Asset Support**: USD and BTC balances tracked per account
- **Pre-trade Validation**: Balance checks before order placement
- **Atomic Settlement**: Trade execution and balance updates in single operation
- **Order Ownership**: Tracks open orders for settlement

## Trading Flow

1. **Order Creation**: Trading agents generate orders with random parameters
2. **Balance Check**: Ledger validates sufficient funds
3. **Order Insertion**: Order book attempts immediate matching
4. **Trade Execution**: Matched orders create trades
5. **Settlement**: Ledger updates balances atomically
6. **Order Management**: Unfilled orders rest in the book

## Getting Started

### Running the Simulation

```bash
cargo run --release
```

The simulation will:
- Initialize 100,000 trading accounts with initial balances
- Spawn 100,000 concurrent trading bots
- Process orders at 1ms intervals
- Display real-time performance metrics

### Configuration

Adjust simulation parameters in `src/main.rs`:
- Number of trading agents
- Initial account balances
- Order generation frequency
- Price ranges and quantities

## Performance Tuning

### Key Optimizations

- **Async Task Spawning**: Efficient concurrent processing
- **Lock-free Data Structures**: DashMap for concurrent access
- **Memory Layout**: Optimized data structures for cache locality
- **Batch Processing**: Efficient trade settlement

## 🧪 Testing

```bash
# Run unit tests
cargo test

# Run with logging
RUST_LOG=debug cargo test

```

## Disclaimer

**This is a toy/educational implementation and should NOT be used in production trading environments.**
