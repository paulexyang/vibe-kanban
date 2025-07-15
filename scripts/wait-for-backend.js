#!/usr/bin/env node

const http = require('http');

const MAX_RETRIES = 30; // 15 seconds (30 * 500ms)
const RETRY_INTERVAL = 500; // 500ms between retries

async function checkBackend(port) {
  return new Promise((resolve) => {
    const req = http.get(`http://127.0.0.1:${port}/api/health`, (res) => {
      resolve(res.statusCode === 200);
    });
    
    req.on('error', () => {
      resolve(false);
    });
    
    req.setTimeout(1000);
  });
}

async function waitForBackend() {
  const backendPort = process.env.BACKEND_PORT || '3001';
  console.log(`Waiting for backend on port ${backendPort}...`);
  
  for (let i = 0; i < MAX_RETRIES; i++) {
    if (await checkBackend(backendPort)) {
      console.log('✓ Backend is ready!');
      return true;
    }
    
    if (i < MAX_RETRIES - 1) {
      process.stdout.write('.');
      await new Promise(resolve => setTimeout(resolve, RETRY_INTERVAL));
    }
  }
  
  console.error('\n✗ Backend failed to start after 15 seconds');
  return false;
}

// Run if called directly
if (require.main === module) {
  waitForBackend().then(success => {
    process.exit(success ? 0 : 1);
  });
}

module.exports = { waitForBackend };