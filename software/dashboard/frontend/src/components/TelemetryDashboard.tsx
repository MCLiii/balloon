import React from 'react';
import { Thermometer, Gauge, Droplets, Mountain, MapPin, Activity, Zap, RotateCcw } from 'lucide-react';
import TelemetryCard from './TelemetryCard';
import { TelemetryData } from '../types/telemetry';

interface TelemetryDashboardProps {
  data: TelemetryData | null;
}

const TelemetryDashboard: React.FC<TelemetryDashboardProps> = ({ data }) => {
  if (!data) {
    return (
      <div className="telemetry-grid">
        {[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12].map((i) => (
          <div key={i} className="telemetry-card-skeleton">
            <div className="skeleton-line skeleton-title"></div>
            <div className="skeleton-line skeleton-value"></div>
          </div>
        ))}
      </div>
    );
  }

  return (
    <div className="telemetry-grid">
      <TelemetryCard
        title="Temperature"
        value={data.temperature.toFixed(1)}
        unit="°C"
        icon={<Thermometer />}
        color="#ef4444"
      />
      <TelemetryCard
        title="Pressure"
        value={data.pressure.toFixed(1)}
        unit="hPa"
        icon={<Gauge />}
        color="#3b82f6"
      />
      <TelemetryCard
        title="Humidity"
        value={data.humidity.toFixed(1)}
        unit="%"
        icon={<Droplets />}
        color="#06b6d4"
      />
      <TelemetryCard
        title="Altitude"
        value={data.altitude.toFixed(1)}
        unit="m"
        icon={<Mountain />}
        color="#10b981"
      />
      <TelemetryCard
        title="Latitude"
        value={data.latitude.toFixed(6)}
        unit="°"
        icon={<MapPin />}
        color="#8b5cf6"
      />
      <TelemetryCard
        title="Longitude"
        value={data.longitude.toFixed(6)}
        unit="°"
        icon={<MapPin />}
        color="#f59e0b"
      />
      <TelemetryCard
        title="Accel X"
        value={data.accel_x.toFixed(2)}
        unit="m/s²"
        icon={<Activity />}
        color="#ec4899"
      />
      <TelemetryCard
        title="Accel Y"
        value={data.accel_y.toFixed(2)}
        unit="m/s²"
        icon={<Activity />}
        color="#ec4899"
      />
      <TelemetryCard
        title="Accel Z"
        value={data.accel_z.toFixed(2)}
        unit="m/s²"
        icon={<Activity />}
        color="#ec4899"
      />
      <TelemetryCard
        title="Gyro X"
        value={data.gyro_x.toFixed(1)}
        unit="°/s"
        icon={<RotateCcw />}
        color="#84cc16"
      />
      <TelemetryCard
        title="Gyro Y"
        value={data.gyro_y.toFixed(1)}
        unit="°/s"
        icon={<RotateCcw />}
        color="#84cc16"
      />
      <TelemetryCard
        title="Gyro Z"
        value={data.gyro_z.toFixed(1)}
        unit="°/s"
        icon={<RotateCcw />}
        color="#84cc16"
      />
    </div>
  );
};

export default TelemetryDashboard;
