# pulse

real-time multiplayer netcode.

interest management. lag compensation. snapshots. client prediction.  
not a full game engine. not unreal. not photon. just the boring networking bits that always end up rewritten from scratch.

## why

every multiplayer prototype eventually hits the same wall:
- “why does the other player rubber-band?”
- “how do i only send relevant entities?”
- “can we rewind time a bit when a shot is fired?”

pulse is the skeleton for those answers. the interesting algorithms are sketched; the production polish is still on the todo list.

## what it tries to cover

- interest: spatial / priority-based relevance so you don’t flood every client with the whole world
- snapshots: delta-compressed world state at a fixed tick rate
- prediction: client-side movement that gets reconciled when the server corrects you
- lag compensation: server-side rewind for hitscan / projectile validation (stub)
- transport: reliable + unreliable channels over a generic socket abstraction

## status

early. types and the overall shape exist.  
the actual delta compressor, interest graph, and rewind buffer are still “yes, later”.

don’t ship a competitive shooter on this yet. do poke at the structure if you’re curious how these systems fit together.

## crates

- `pulse-core` — ticks, entity ids, errors
- `pulse-protocol` — message types, serialization stubs
- `pulse-interest` — relevance / aoi
- `pulse-snapshot` — world snapshots + deltas
- `pulse-prediction` — client prediction + reconciliation
- `pulse-transport` — channel abstraction
- `pulse-server` — authoritative server loop
- `pulse-client` — client connection + prediction
- `pulse-cli` — local tooling

js + python packages under `packages/`.

## license

mit. steal the ideas. improve the code. don’t blame me when the lag feels wrong.
