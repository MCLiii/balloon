import React from 'react';
import { useWebSocket } from './hooks/useWebSocket';
import TelemetryDashboard from './components/TelemetryDashboard';
import TelemetryCharts from './components/TelemetryCharts';
import TelemetryMap from './components/TelemetryMap';
import { Activity, Wifi, WifiOff } from 'lucide-react';
import './App.css';

function App() {
  const { isConnected, telemetryData, latestData } = useWebSocket('ws://localhost:8000/ws');

  return (
    <div className="min-h-screen bg-gray-50">
      {/* Header */}
      <header className="bg-white shadow-sm border-b">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex justify-between items-center py-4">
            <div className="flex items-center space-x-3">
              <div className="w-8 h-8 bg-blue-600 rounded-lg flex items-center justify-center">
                <Activity className="w-5 h-5 text-white" />
              </div>
              <h1 className="text-2xl font-bold text-gray-900">Balloon Telemetry Dashboard</h1>
            </div>
            <div className="flex items-center space-x-2">
              {isConnected ? (
                <div className="flex items-center space-x-2 text-green-600">
                  <Wifi className="w-5 h-5" />
                  <span className="text-sm font-medium">Connected</span>
                </div>
              ) : (
                <div className="flex items-center space-x-2 text-red-600">
                  <WifiOff className="w-5 h-5" />
                  <span className="text-sm font-medium">Disconnected</span>
                </div>
              )}
            </div>
          </div>
        </div>
      </header>

      {/* Main Content */}
      <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {/* Status Bar */}
        <div className="mb-8">
          <div className="bg-white rounded-lg shadow-md p-4">
            <div className="flex items-center justify-between">
              <div>
                <h2 className="text-lg font-semibold text-gray-900">Mission Status</h2>
                <p className="text-sm text-gray-600">
                  {telemetryData.length > 0 
                    ? `Received ${telemetryData.length} telemetry packets`
                    : 'Waiting for telemetry data...'
                  }
                </p>
              </div>
              <div className="text-right">
                <p className="text-sm text-gray-600">Last Update</p>
                <p className="text-sm font-medium text-gray-900">
                  {latestData 
                    ? new Date(latestData.timestamp * 1000).toLocaleString()
                    : 'Never'
                  }
                </p>
              </div>
            </div>
          </div>
        </div>

        {/* Telemetry Cards */}
        <div className="mb-8">
          <h2 className="text-xl font-semibold text-gray-900 mb-6">Current Readings</h2>
          <TelemetryDashboard data={latestData} />
        </div>

        {/* Charts */}
        <div className="mb-8">
          <h2 className="text-xl font-semibold text-gray-900 mb-6">Historical Data</h2>
          <TelemetryCharts data={telemetryData} />
        </div>

        {/* Map */}
        <div className="mb-8">
          <h2 className="text-xl font-semibold text-gray-900 mb-6">Location & Trajectory</h2>
          <TelemetryMap data={telemetryData} latestData={latestData} />
        </div>
      </main>

      {/* Footer */}
      <footer className="bg-white border-t">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4">
          <div className="text-center text-sm text-gray-600">
            <p>Balloon Telemetry System - Real-time monitoring dashboard</p>
          </div>
        </div>
      </footer>
    </div>
  );
}

export default App;