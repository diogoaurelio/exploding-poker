Exploding Poker
=================

This is a toy project to code a variant of [Texas Hold'em](https://en.wikipedia.org/wiki/Texas_hold_%27em) inspired by [exploding kitens game](https://explodingkittens.com).
It is also my first go at rust programming language - bare with me.

### Running locally

```
cargo build
systemfd --no-pid -s http::3000 -- cargo watch -x run
```

### License
This project is MIT licensed.