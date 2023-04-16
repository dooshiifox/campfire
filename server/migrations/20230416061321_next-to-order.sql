-- On the `roles`, `channels`, and `guild_members`, remove the
-- `next` column and add an `order` column instead.

ALTER TABLE roles DROP COLUMN next;
ALTER TABLE roles ADD COLUMN "order" integer NOT NULL DEFAULT 0;

ALTER TABLE channels DROP COLUMN next;
ALTER TABLE channels ADD COLUMN "order" integer NOT NULL DEFAULT 0;

ALTER TABLE guild_members DROP COLUMN next;
ALTER TABLE guild_members ADD COLUMN "order" integer NOT NULL DEFAULT 0;
