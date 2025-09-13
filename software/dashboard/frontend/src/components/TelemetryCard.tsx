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
    <div className="bg-white rounded-lg shadow-md p-6 border-l-4" style={{ borderLeftColor: color }}>
      <div className="flex items-center justify-between">
        <div>
          <p className="text-sm font-medium text-gray-600">{title}</p>
          <p className="text-2xl font-bold text-gray-900">
            {value} <span className="text-sm font-normal text-gray-500">{unit}</span>
          </p>
        </div>
        <div className="flex items-center space-x-2">
          <div className="text-2xl" style={{ color }}>
            {icon}
          </div>
          {trend && (
            <span className="text-lg text-gray-400">{getTrendIcon()}</span>
          )}
        </div>
      </div>
    </div>
  );
};

export default TelemetryCard;
