import React from 'react';
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer, AreaChart, Area } from 'recharts';
import { TelemetryData } from '../types/telemetry';

interface TelemetryChartsProps {
  data: TelemetryData[];
}

const TelemetryCharts: React.FC<TelemetryChartsProps> = ({ data }) => {
  const chartData = data.map((item, index) => ({
    time: new Date(item.timestamp * 1000).toLocaleTimeString(),
    temperature: item.temperature,
    accel_x: item.accel_x,
    accel_y: item.accel_y,
    accel_z: item.accel_z,
    gyro_x: item.gyro_x,
    gyro_y: item.gyro_y,
    gyro_z: item.gyro_z,
  }));

  return (
    <div className="charts-grid">
      {/* Temperature Chart */}
      <div className="chart-container">
        <h3 className="chart-title">Temperature</h3>
        <ResponsiveContainer width="100%" height={300}>
          <AreaChart data={chartData}>
            <CartesianGrid strokeDasharray="3 3" />
            <XAxis dataKey="time" />
            <YAxis />
            <Tooltip />
            <Area 
              type="monotone" 
              dataKey="temperature" 
              stroke="#ef4444" 
              fill="#ef4444" 
              fillOpacity={0.3}
            />
          </AreaChart>
        </ResponsiveContainer>
      </div>



      {/* Accelerometer Chart */}
      <div className="chart-container">
        <h3 className="chart-title">Accelerometer</h3>
        <ResponsiveContainer width="100%" height={300}>
          <LineChart data={chartData}>
            <CartesianGrid strokeDasharray="3 3" />
            <XAxis dataKey="time" />
            <YAxis />
            <Tooltip />
            <Line 
              type="monotone" 
              dataKey="accel_x" 
              stroke="#ec4899" 
              strokeWidth={2}
              name="X"
            />
            <Line 
              type="monotone" 
              dataKey="accel_y" 
              stroke="#f97316" 
              strokeWidth={2}
              name="Y"
            />
            <Line 
              type="monotone" 
              dataKey="accel_z" 
              stroke="#eab308" 
              strokeWidth={2}
              name="Z"
            />
          </LineChart>
        </ResponsiveContainer>
      </div>

      {/* Gyroscope Chart */}
      <div className="chart-container">
        <h3 className="chart-title">Gyroscope</h3>
        <ResponsiveContainer width="100%" height={300}>
          <LineChart data={chartData}>
            <CartesianGrid strokeDasharray="3 3" />
            <XAxis dataKey="time" />
            <YAxis />
            <Tooltip />
            <Line 
              type="monotone" 
              dataKey="gyro_x" 
              stroke="#84cc16" 
              strokeWidth={2}
              name="X"
            />
            <Line 
              type="monotone" 
              dataKey="gyro_y" 
              stroke="#22c55e" 
              strokeWidth={2}
              name="Y"
            />
            <Line 
              type="monotone" 
              dataKey="gyro_z" 
              stroke="#06b6d4" 
              strokeWidth={2}
              name="Z"
            />
          </LineChart>
        </ResponsiveContainer>
      </div>
    </div>
  );
};

export default TelemetryCharts;
