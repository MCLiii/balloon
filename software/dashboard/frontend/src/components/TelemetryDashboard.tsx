import React from 'react';
import { Thermometer, Gauge, Droplets, Mountain, MapPin, Activity } from 'lucide-react';
import TelemetryCard from './TelemetryCard';
import { TelemetryData } from '../types/telemetry';

interface TelemetryDashboardProps {
  data: TelemetryData | null;
}

const TelemetryDashboard: React.FC<TelemetryDashboardProps> = ({ data }) => {
  if (!data) {
    return (
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {[1, 2, 3, 4, 5, 6].map((i) => (
          <div key={i} className="bg-gray-200 rounded-lg p-6 animate-pulse">
            <div className="h-4 bg-gray-300 rounded w-1/2 mb-2"></div>
            <div className="h-8 bg-gray-300 rounded w-3/4"></div>
          </div>
        ))}
      </div>
    );
  }

  return (
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
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
    </div>
  );
};

export default TelemetryDashboard;
