#!/usr/bin/env bash
# Rhyph — quick start script for bare metal
set -euo pipefail

GREEN='\033[0;32m'
NC='\033[0m'

if [ -f .env ]; then
    set -a; source .env; set +a
else
    cp .env.example .env
    set -a; source .env; set +a
fi

DB_USER="${DB_USER:-rhyph}"
DB_PASSWORD="${DB_PASSWORD:-rhyph}"
DB_NAME="${DB_NAME:-rhyph}"
DATABASE_URL="postgres://${DB_USER}:${DB_PASSWORD}@localhost/${DB_NAME}"
JWT_SECRET="${JWT_SECRET:-change...tion}"

echo -e "${GREEN}Starting Rhyph...${NC}"

# Kill any existing server on port 3000
fuser -k 3000/tcp 2>/dev/null || true

# Start backend
DATABASE_URL="${DATABASE_URL}" JWT_SECRET="${JWT_SECRET}" \
    ./target/release/rhyph-server &
BACKEND_PID=$!

# Wait for backend
for i in $(seq 1 10); do
    if curl -s http://localhost:3000/health >/dev/null 2>&1; then
        break
    fi
    sleep 1
done

echo -e "${GREEN}Backend:    http://localhost:3000 (pid ${BACKEND_PID})${NC}"

# Start frontend dev server if built files exist
if [ -d frontend/build ]; then
    echo -e "${GREEN}Frontend:   cd frontend && npm run dev${NC}"
else
    cd frontend && npm run dev &
    FRONTEND_PID=$!
    echo -e "${GREEN}Frontend:   http://localhost:5173 (pid ${FRONTEND_PID})${NC}"
    cd ..
fi

echo ""
echo -e "Login: ${GREEN}admin@rhyph.local${NC} / ${GREEN}admin123${NC}"
echo ""
echo "Press Ctrl+C to stop"

wait $BACKEND_PID
