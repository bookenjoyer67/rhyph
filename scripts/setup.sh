#!/usr/bin/env bash
# Rhyph — bare-metal setup script
# Run this once after cloning the repo.
set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${GREEN}=== Rhyph Setup ===${NC}"
echo ""

# ── Check prerequisites ──
command -v cargo >/dev/null 2>&1 || { echo -e "${RED}cargo not found. Install Rust: https://rustup.rs${NC}"; exit 1; }
command -v psql >/dev/null 2>&1 || { echo -e "${RED}psql not found. Install PostgreSQL client.${NC}"; exit 1; }
command -v npm >/dev/null 2>&1 || { echo -e "${RED}npm not found. Install Node.js.${NC}"; exit 1; }

# ── Load env ──
if [ -f .env ]; then
    set -a; source .env; set +a
else
    cp .env.example .env
    echo -e "${YELLOW}Created .env from .env.example — edit it if needed${NC}"
    set -a; source .env; set +a
fi

DB_USER="${DB_USER:-rhyph}"
DB_PASSWORD="${DB_PASSWORD:-rhyph}"
DB_NAME="${DB_NAME:-rhyph}"
DATABASE_URL="postgres://${DB_USER}:${DB_PASSWORD}@localhost/${DB_NAME}"
JWT_SECRET="${JWT_SECRET:-change...tion}"

# ── PostgreSQL ──
echo -e "${YELLOW}Setting up PostgreSQL...${NC}"

if pg_isready -q 2>/dev/null; then
    echo "  PostgreSQL is running"
else
    echo "  Starting PostgreSQL..."
    sudo systemctl start postgresql 2>/dev/null || pg_ctlcluster 18 main start 2>/dev/null || {
        echo -e "${RED}  Could not start PostgreSQL. Start it manually and re-run.${NC}"; exit 1;
    }
    sleep 1
fi

# Create user + database
sudo -u postgres psql -c "CREATE USER ${DB_USER} WITH PASSWORD '${DB_PASSWORD}' CREATEDB;" 2>/dev/null || true
sudo -u postgres psql -c "CREATE DATABASE ${DB_NAME} OWNER ${DB_USER};" 2>/dev/null || true
echo -e "${GREEN}  Database ready${NC}"

# ── Build backend ──
echo -e "${YELLOW}Building backend...${NC}"
cargo build --release -p rhyph-server -p hashpw
echo -e "${GREEN}  Backend built${NC}"

# ── Run migrations ──
echo -e "${YELLOW}Running migrations...${NC}"
DATABASE_URL="${DATABASE_URL}" JWT_SECRET="${JWT_SECRET}" \
    timeout 5 ./target/release/rhyph-server 2>/dev/null || true
sleep 1
fuser -k 3000/tcp 2>/dev/null || true
echo -e "${GREEN}  Migrations applied${NC}"

# ── Seed data ──
echo -e "${YELLOW}Seeding initial data...${NC}"
ADMIN_HASH=$(./target/release/hashpw "admin123")

PGPASSWORD="${DB_PASSWORD}" psql -h localhost -U "${DB_USER}" -d "${DB_NAME}" <<SQL
INSERT INTO users (email, password_hash, is_admin)
VALUES ('admin@rhyph.local', '${ADMIN_HASH}', true)
ON CONFLICT (email) DO NOTHING;

INSERT INTO organizers (slug, name)
VALUES ('default', 'My Venue')
ON CONFLICT (slug) DO NOTHING;
SQL
echo -e "${GREEN}  Seed data ready${NC}"

# ── Frontend ──
echo -e "${YELLOW}Building frontend...${NC}"
cd frontend && npm install --silent && npm run build && cd ..
echo -e "${GREEN}  Frontend built${NC}"

echo ""
echo -e "${GREEN}=== Setup complete! ===${NC}"
echo ""
echo -e "Start the backend:  ${YELLOW}./scripts/start.sh${NC}"
echo -e "Login:              ${YELLOW}admin@rhyph.local${NC} / ${YELLOW}admin123${NC}"
echo -e "Open:               ${YELLOW}http://localhost:5173${NC}"
echo ""
echo -e "Or use Podman:      ${YELLOW}podman compose up -d${NC}"
echo ""
