export interface TelemetryData {
  sync: number;
  timestamp: number;
  temperature: number;
  pressure: number;
  humidity: number;
  altitude: number;
  latitude: number;
  longitude: number;
  accel_x: number;
  accel_y: number;
  accel_z: number;
  gyro_x: number;
  gyro_y: number;
  gyro_z: number;
  status: number;
  received_at: string;
}

export interface TelemetryStats {
  total_packets: number;
  temperature: {
    min: number;
    max: number;
    avg: number;
  };
  pressure: {
    min: number;
    max: number;
    avg: number;
  };
  humidity: {
    min: number;
    max: number;
    avg: number;
  };
  altitude: {
    min: number;
    max: number;
    avg: number;
  };
  accelerometer: {
    x: { min: number; max: number; avg: number };
    y: { min: number; max: number; avg: number };
    z: { min: number; max: number; avg: number };
  };
  gyroscope: {
    x: { min: number; max: number; avg: number };
    y: { min: number; max: number; avg: number };
    z: { min: number; max: number; avg: number };
  };
}

export interface WebSocketMessage {
  type: 'telemetry' | 'initial_data';
  data: TelemetryData | TelemetryData[];
}
