# Todo

- [ ] Determine precise format of messages
- [ ] Implement client better, with Slint UI
- [ ] Make constants in lib for consistency in Slint
- [ ] Create server UI design
- [ ] Figure out how to run client through Slint

## Message Format

```json
Message {
    Header {
        Name: "string",
        Time: 0000, // unix time in milliseconds?
        Hash: 0000,
        Kind: enum,
    },
    Body {
        // Depends on the Kind
    }
}
```