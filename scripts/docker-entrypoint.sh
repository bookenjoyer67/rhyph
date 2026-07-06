#!/bin/sh
# Docker entrypoint — seed default admin + organizer on first start

SEED_MARKER="/data/.seeded"

if [ ! -f "$SEED_MARKER" ]; then
    echo "First start — seeding default admin user and organizer..."

    # Generate admin password hash
    ADMIN_HASH=$(rhyph-server hashpw "admin123" 2>/dev/null || echo "")

    if [ -n "$ADMIN_HASH" ]; then
        PGPASSWORD="${DB_PASSWORD:-rhyph}" psql \
            -h "${DB_HOST:-db}" \
            -U "${DB_USER:-rhyph}" \
            -d "${DB_NAME:-rhyph}" \
            -c "INSERT INTO users (email, password_hash, is_admin)
                VALUES ('admin@rhyph.local', '$ADMIN_HASH', true)
                ON CONFLICT (email) DO NOTHING;" 2>/dev/null

        PGPASSWORD="${DB_PASSWORD:-rhyph}" psql \
            -h "${DB_HOST:-db}" \
            -U "${DB_USER:-rhyph}" \
            -d "${DB_NAME:-rhyph}" \
            -c "INSERT INTO organizers (slug, name)
                VALUES ('default', 'My Venue')
                ON CONFLICT (slug) DO NOTHING;" 2>/dev/null
    fi

    mkdir -p /data
    touch "$SEED_MARKER"
    echo "Seed complete."
fi

exec rhyph-server
