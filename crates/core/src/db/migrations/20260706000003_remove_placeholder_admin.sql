-- Remove the non-functional placeholder admin seed (replaced by web setup wizard)
DELETE FROM users WHERE email = 'admin@rhyph.local' AND password_hash LIKE '%placeholder%';
