# Pulse

**is this a game engine?**
no. it is vec3 math, snapshots, interpolation, and a tiny in process tick loop.

**does it send packets?**
not yet. there is no UDP. lag compensation and interest management are the next things I want to hang on this, not things I will pretend exist.

**so what compiles?**
Vec3 add, sub, scale. entity lerp. snapshot interpolate (halfway from 0 to 10 is 5, the test is that boring on purpose). an in process server that spawns an entity and integrates velocity. `pulse version` and `pulse demo`.

run 5 ticks and you should see entity_x move:

    cargo test --workspace
    cargo build -p pulse-cli
    ./target/debug/pulse demo --ticks 5

MIT. predict locally, correct later, once there is something to correct against.
