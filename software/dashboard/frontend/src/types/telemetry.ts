export interface TelemetryData {
  sync: number;
  timestamp: number;
  temperature: number;
  pressure: number;
  humidity: number;
  altitude: number;
  latitude: number;
  longitude: number;
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
}

export interface WebSocketMessage {
  type: 'telemetry' | 'initial_data';
  data: TelemetryData | TelemetryData[];
}
