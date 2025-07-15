#!/bin/bash

# Vibe Kanban Production Start Script

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}Starting Vibe Kanban in production mode...${NC}"

# Check if build exists
if [ ! -f "backend/target/release/vibe-kanban" ]; then
    echo -e "${YELLOW}Backend binary not found. Building...${NC}"
    pnpm build
fi

if [ ! -d "frontend/dist" ]; then
    echo -e "${YELLOW}Frontend build not found. Building...${NC}"
    pnpm build
fi

# Export ports
export FRONTEND_PORT=4173  # Vite preview default port
export BACKEND_PORT=3001

# Start backend
echo -e "${GREEN}Starting backend on port $BACKEND_PORT...${NC}"
./target/release/vibe-kanban &
BACKEND_PID=$!
cd ..

# Wait for backend to be ready
echo -e "${YELLOW}Waiting for backend to start...${NC}"
for i in {1..30}; do
    if curl -s http://localhost:$BACKEND_PORT/api/health > /dev/null 2>&1; then
        echo -e "${GREEN}Backend is ready!${NC}"
        break
    fi
    sleep 1
done

# Start frontend with preview (serves the built files)
echo -e "${GREEN}Starting frontend on port $FRONTEND_PORT...${NC}"
cd frontend && npx vite preview --port $FRONTEND_PORT --host 0.0.0.0 &
FRONTEND_PID=$!
cd ..

echo -e "${GREEN}Vibe Kanban is running!${NC}"
echo -e "Frontend: http://localhost:$FRONTEND_PORT"
echo -e "Backend API: http://localhost:$BACKEND_PORT"
echo -e ""
echo -e "${YELLOW}Press Ctrl+C to stop${NC}"

# Handle Ctrl+C
trap 'echo -e "\n${RED}Stopping services...${NC}"; kill $BACKEND_PID $FRONTEND_PID 2>/dev/null; exit' INT

# Wait for processes
wait
