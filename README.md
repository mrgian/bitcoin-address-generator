# Bitcoin Address Generator
A bitcoin address generator written in Rust.\
Generates a Bitcoin wallet address from a random private 256bit key.

Made for learning purposes only.

Here are the steps used to generate the address:

```
 ┌─────────────────────┐                      ┌────────────┐
 │                     │                      │            │
 │ 256 bit Private Key ├─────────────────────►│ Public Key │
 │                     │  Elliptic Curve      │            │
 └─────────────────────┘  Digital Signature   └──────┬─────┘
                          Algorithm                  │
                                                     │SHA256/RIPEMD160
                                                     │
 ┌─────────────────────┐      Add network     ┌──────▼─────┐
 │     Hashed Key      │      byte prefix     │            │
 │   with net prefix   │◄─────────────────────┤ Hashed Key │
 └──────────┬─────┬────┘                      │            │
            │     │                           └────────────┘
            │     │
SHA256 twice│     │
            │     │
            │     │
            │     │                                     ┌─────────────┐
            │     └────────────────────────────────────►│             │
            │               Add first four bytes        │  Unencoded  │
  ┌─────────▼──────────┐ of checksum to hashed key      │   Address   │
  │                    ├───────────────────────────────►│             │
  │      Checksum      │                                └──────┬──────┘
  │                    │                                       │
  └────────────────────┘                                       │
                                                               │Base58 Encoding
                                                               │
                                                     ┌─────────▼──────────┐
                                                     │                    │
                                                     │     BITCOIN        │
                                                     │       WALLET       │
                                                     │        ADDRESS     │
                                                     │                    │
                                                     └────────────────────┘
```
