import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App.tsx';
import './index.css';
import { ClickToComponent } from 'click-to-react-component';
import * as Sentry from '@sentry/react';
import {
  useLocation,
  useNavigationType,
  createRoutesFromChildren,
  matchRoutes,
} from 'react-router-dom';

// Only initialize Sentry if explicitly enabled via environment variable AND a DSN is provided
if (import.meta.env.VITE_ENABLE_SENTRY === 'true' && import.meta.env.VITE_SENTRY_DSN) {
  Sentry.init({
    dsn: import.meta.env.VITE_SENTRY_DSN,
    tracesSampleRate: 1.0,
    environment: import.meta.env.MODE === 'development' ? 'dev' : 'production',
    integrations: [
      Sentry.reactRouterV6BrowserTracingIntegration({
        useEffect: React.useEffect,
        useLocation,
        useNavigationType,
        createRoutesFromChildren,
        matchRoutes,
      }),
    ],
  });
  Sentry.setTag('source', 'frontend');
}

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <Sentry.ErrorBoundary fallback={<p>An error has occurred</p>} showDialog>
      <ClickToComponent />
      <App />
    </Sentry.ErrorBoundary>
  </React.StrictMode>
);
