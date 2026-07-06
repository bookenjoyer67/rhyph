-- Organizer storefront customization
ALTER TABLE organizers ADD COLUMN theme JSONB NOT NULL DEFAULT '{}';
ALTER TABLE organizers ADD COLUMN custom_domain VARCHAR(255);

CREATE UNIQUE INDEX idx_organizers_custom_domain
    ON organizers(custom_domain) WHERE custom_domain IS NOT NULL;
