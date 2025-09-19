import React from 'react';
import { useWebSocket } from './hooks/useWebSocket';
import TelemetryDashboard from './components/TelemetryDashboard';
import TelemetryCharts from './components/TelemetryCharts';
// import TelemetryMap from './components/TelemetryMap';
import { Activity, Wifi, WifiOff } from 'lucide-react';
import './styles/styles.css';

function App() {
  const { isConnected, telemetryData, latestData } = useWebSocket('ws://localhost:8000/ws');

  
  return (
    <div className="app-container">
      {/* Header */}
      <header className="app-header">
        <div className="app-header-content">
          <div className="app-header-inner">
            <div className="app-logo">
              <div className="app-logo-icon">
                <Activity className="app-status-icon" />
              </div>
              <h1 className="app-title">Balloon Telemetry Dashboard</h1>
            </div>
            <div className="app-status">
              {isConnected ? (
                <div className="app-status-connected">
                  <Wifi className="app-status-icon" />
                  <span className="app-status-text">Connected</span>
                </div>
              ) : (
                <div className="app-status-disconnected">
                  <WifiOff className="app-status-icon" />
                  <span className="app-status-text">Disconnected</span>
                </div>
              )}
            </div>
          </div>
        </div>
      </header>

      {/* Main Content */}
      <main className="app-main">
        {/* Status Bar */}
        <div className="status-bar">
          <div className="status-card">
            <div className="status-card-content">
              <div className="status-info">
                <h2>Mission Status</h2>
                <p>
                  {telemetryData.length > 0 
                    ? `Received ${telemetryData.length} telemetry packets`
                    : 'Waiting for telemetry data...'
                  }
                </p>
              </div>
              <div className="status-last-update">
                <p>Last Update</p>
                <p>
                  {latestData 
                    ? new Date(latestData.timestamp * 1000).toLocaleString()
                    : 'Never'
                  }
                </p>
              </div>
            </div>
          </div>
        </div>

        <div className="main-content-grid">
          {/* Telemetry Cards */}
          <div className="mb-8">
            <h2 className="section-header">Current Readings</h2>
            <TelemetryDashboard data={latestData} />
            {/* <h2 className="section-header">Location & Trajectory</h2>
            <TelemetryMap data={telemetryData} latestData={latestData} /> */}
          </div>

        
          {/* Charts */}
          <div className="mb-8">
            <h2 className="section-header">Historical Data</h2>
            <TelemetryCharts data={telemetryData} />
          </div>

          {/* Map */}
          <div className="mb-8">
            
            
          </div>
        </div>
      </main>

      {/* Footer */}
      <footer className="app-footer">
        <div className="app-footer-content">
          <div className="app-footer-text">
            <p>Balloon Telemetry System - Real-time monitoring dashboard</p>
          </div>
        </div>
      </footer>
    </div>
  );
}

export default App;