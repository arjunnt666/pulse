# Lag compensation

server rewinds world state to the client time of an action, then validates.

skeleton defaults:
- history buffer: last 128 ticks
- target rewind: ~120ms at 60hz
- hitscan only in the notes; projectiles need extra care

do not ship competitive rules on these numbers. measure rtt distribution first.
