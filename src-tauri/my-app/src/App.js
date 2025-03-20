import React, { useState } from 'react';
import './App.css';

function App() {
  const [progress, setProgress] = useState(0);

  const startDownload = async () => {
    // Simulate a download process
    for (let i = 0; i <= 100; i++) {
      setProgress(i);
      await new Promise((resolve) => setTimeout(resolve, 50));
    }
  };

  return (
    <div className="App">
      <header className="App-header">
        <h1>Rust Autopatcher</h1>
        <button onClick={startDownload} className="download-button">Start Download</button>
        <div className="progress-bar">
          <div className="progress" style={{ width: `${progress}%` }}></div>
        </div>
      </header>
    </div>
  );
}

export default App;