import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import './App.css';

function App() {
  const [progress, setProgress] = useState(0);

  const startDownload = async () => {
    try {
      const response = await invoke('download_patch', {
        url: 'https://example.com/path/to/patch.zip',
        outputPath: 'downloaded_patch.zip',
        expectedHash: 'your_expected_patch_hash_here',
        useBlake3: true,
      });
      console.log(response);
    } catch (error) {
      console.error(error);
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