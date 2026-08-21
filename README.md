# pulse

multiplayer netcode notes that compile.

vec3 math, snapshots, interpolation, a tiny in-process tick loop that integrates velocity. `pulse demo --ticks 5` prints a moving entity_x.

not a game engine. not production lag compensation. a place to hang prediction and interest management without a 40gb editor.

## works today

- Vec3 add / sub / scale
- entity lerp and snapshot interpolate (halfway 0 to 10 is 5)
- in-process server ticks spawn + integrate velocity
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
