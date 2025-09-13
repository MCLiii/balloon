import React from 'react';
import { Thermometer, Gauge, Droplets, Mountain, MapPin, Activity } from 'lucide-react';

interface TelemetryCardProps {
  title: string;
  value: string | number;
  unit: string;
  icon: React.ReactNode;
  color: string;
  trend?: 'up' | 'down' | 'stable';
}

const TelemetryCard: React.FC<TelemetryCardProps> = ({ 
  title, 
  value, 
  unit, 
  icon, 
  color,
  trend 
}) => {
  const getTrendIcon = () => {
    if (trend === 'up') return '↗';
    if (trend === 'down') return '↘';
    return '→';
  };

  return (
    <div className="telemetry-card" style={{ borderLeftColor: color }}>
      <div className="telemetry-card-content">
        <div className="telemetry-card-info">
          <p className="telemetry-card-title">{title}</p>
          <p className="telemetry-card-value">
            {value} <span className="telemetry-card-unit">{unit}</span>
          </p>
        </div>
        <div className="telemetry-card-icon-container">
          <div className="telemetry-card-icon" style={{ color }}>
            {icon}
          </div>
          {trend && (
            <span className="telemetry-card-trend">{getTrendIcon()}</span>
          )}
        </div>
      </div>
    </div>
  );
};

export default TelemetryCard;
