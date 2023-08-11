run with 

```bash
$ RUST_LOG=info cargo run
```

* the transaction seem to be good
* but no token has been swapped

* if you replace the in-code anvil fork with an external local hardhat tracing or anvil blockchain you see the following error

```bash
$ npx hardhat node --verbose  --fork  https://eth.llamarpc.com
```

```bash
eth_sendTransaction
  Contract call:   <UnrecognizedContract>
  Transaction:     0x47b355700e65bd9a40e97012fbe30971f41e3f593e2686f84a6bcd3b602720d5
  From:            0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266
  To:              0x7a250d5630b4cf539739df2c5dacb4c659f2488d
  Value:           5 ETH
  Gas used:        30995 of 50000
  Block #17889993: 0xf7d9dbf378dcc0790a3f5bad8245a47fd141d6b7784475a60805026d5b8d9a1c

  Error: VM Exception while processing transaction: reverted with reason string 'ds-math-sub-underflow'
```

in external anvil you see the following


```bash
$ anvil --steps-tracing -f https://eth.llamarpc.com 
```

```bash
eth_sendTransaction

    Transaction: 0xc147d36c78e6f2d2fc1c550e79cffa9cdac06d593626eb7a9445ac8c8ac52704
    Gas used: 30995
    Error: reverted with 'ds-math-sub-underflow'

    Block Number: 17889642
    Block Hash: 0x3d40dea39bc8c162b4263cf5bcc31a3f10320ddae7386fc7ad4d8e3c9bc37b1e
    Block Time: "Fri, 11 Aug 2023 06:05:48 +0000"
```