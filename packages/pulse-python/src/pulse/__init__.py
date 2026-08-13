"""Pulse Python client (stub)."""

class Vec3:
    def __init__(self, x: float = 0.0, y: float = 0.0, z: float = 0.0):
        self.x = x
        self.y = y
        self.z = z

class PulseClient:
    def __init__(self, endpoint: str = "ws://localhost:7777"):
        self.endpoint = endpoint
        self._pos = Vec3()

    async def connect(self, room: str, name: str) -> None:
        pass

    def send_input(self, move_dir: Vec3, buttons: int = 0) -> None:
        self._pos.x += move_dir.x * 0.1
        self._pos.y += move_dir.y * 0.1
        self._pos.z += move_dir.z * 0.1

    def predicted_position(self) -> Vec3:
        return self._pos

def version() -> str:
    return "0.1.0"

__all__ = ["Vec3", "PulseClient", "version"]
