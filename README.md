# pulse

multiplayer netcode notes that compile.

vec3 math, snapshots, a tiny in-process tick loop. `pulse demo --ticks 5` actually ticks.

not a game engine. not production lag compensation. a place to hang prediction and interest management without a 40gb editor.

## works today

- Vec3 add / sub / scale
- in-process server ticks
- `pulse version` and `pulse demo`

## does not work yet

- real UDP transport
- tuned lag compensation
- interest management under load

## try it

```bash
cargo test --workspace
cargo build -p pulse-cli
./target/debug/pulse demo --ticks 5
```

## license

mit. predict locally, correct later.
