#!/bin/sh
# Docker entrypoint — run migrations, seed admin, start server

echo "Running migrations..."
# Run server briefly to apply migrations, then kill it
timeout 10 rhyph-server 2>/dev/null || true
sleep 1

# Replace placeholder password with real hash
echo "Seeding admin user..."
ADMIN_HASH=$(rhyph-server hashpw "admin123" 2>/dev/null || echo "")

if [ -n "$ADMIN_HASH" ]; then
    PGPASSWORD="${DB_PASSWORD:-rhyph}" psql \
        -h "${DB_HOST:-db}" \
        -U "${DB_USER:-rhyph}" \
        -d "${DB_NAME:-rhyph}" \
        -c "UPDATE users SET password_hash = '$ADMIN_HASH'
            WHERE email = 'admin@rhyph.local';" 2>/dev/null || true

    PGPASSWORD="${DB_PASSWORD:-rhyph}" psql \
        -h "${DB_HOST:-db}" \
        -U "${DB_USER:-rhyph}" \
        -d "${DB_NAME:-rhyph}" \
        -c "INSERT INTO organizers (slug, name)
            VALUES ('default', 'My Venue')
            ON CONFLICT (slug) DO NOTHING;" 2>/dev/null || true
fi

echo "Seed complete. Login: admin@rhyph.local / admin123"
echo "Starting Rhyph..."

exec rhyph-server
