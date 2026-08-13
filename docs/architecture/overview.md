# Architecture

Pulse splits multiplayer netcode into focused pieces:

1. **core** — ticks, entity ids, positions, errors
2. **protocol** — client/server message types
3. **interest** — area-of-interest / relevance filtering
4. **snapshot** — full + delta world state
5. **prediction** — client-side movement + reconciliation
6. **transport** — reliable/unreliable channel abstraction
7. **server** — authoritative tick loop
8. **client** — connection + prediction glue

Most of the hard algorithms (delta compression, lag-comp rewind, priority queues) are still stubs.
