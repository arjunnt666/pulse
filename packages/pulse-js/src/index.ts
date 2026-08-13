export interface Vec3 { x: number; y: number; z: number; }

export class PulseClient {
  private predicted = { x: 0, y: 0, z: 0 };
  constructor(private endpoint: string = "ws://localhost:7777") {}
  async connect(_room: string, _name: string): Promise<void> {}
  sendInput(moveDir: Vec3, buttons = 0) {
    this.predicted.x += moveDir.x * 0.1;
    this.predicted.y += moveDir.y * 0.1;
    this.predicted.z += moveDir.z * 0.1;
  }
  predictedPosition(): Vec3 { return { ...this.predicted }; }
}

export default PulseClient;
