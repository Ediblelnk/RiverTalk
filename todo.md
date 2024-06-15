# Todo

- [ ] Determine precise format of messages
- [ ] Implement client better, with Slint UI

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