#!/bin/sh
# Docker entrypoint — run migrations, seed organizer, start server

echo "Running migrations..."
# Run server briefly to apply migrations, then kill it
timeout 10 rhyph-server 2>/dev/null || true
sleep 1

echo "Seeding default organizer..."
PGPASSWORD="${DB_PASSWORD:-rhyph}" psql \
    -h "${DB_HOST:-db}" \
    -U "${DB_USER:-rhyph}" \
    -d "${DB_NAME:-rhyph}" \
    -c "INSERT INTO organizers (slug, name)
        VALUES ('default', 'My Venue')
        ON CONFLICT (slug) DO NOTHING;" 2>/dev/null || true

echo "Starting Rhyph..."
echo "Open /login — the setup wizard will create your admin account."

exec rhyph-server
