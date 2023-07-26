-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Sequence and defined type
CREATE SEQUENCE IF NOT EXISTS group_groupid_seq;

-- Table Definition
CREATE TABLE "group" (
    "groupid" SERIAL NOT NULL,
    "name" varchar NOT NULL,
    "description" varchar NOT NULL,
    "extid" varchar NOT NULL UNIQUE,
    "slug" varchar NOT NULL UNIQUE,
    "active" bool NOT NULL,
    "private" bool NOT NULL,
    "members" int4 NOT NULL,
    "cityid" INTEGER NOT NULL,
    "organizer" varchar NOT NULL,
    "highres_link" varchar,
    "photo_link" varchar,
    "thumb_link" varchar,
    "created_at" timestamptz NOT NULL DEFAULT now(),
    "updated_at" timestamptz,
    PRIMARY KEY ("groupid")
);

-- Column Comment
COMMENT ON COLUMN "group"."groupid" IS 'Primary key';
COMMENT ON COLUMN "group"."name" IS 'Group name';
COMMENT ON COLUMN "group"."description" IS 'Group description';
COMMENT ON COLUMN "group"."extid" IS 'External identifier';
COMMENT ON COLUMN "group"."slug" IS 'URL slug';
COMMENT ON COLUMN "group"."active" IS 'Active status';
COMMENT ON COLUMN "group"."private" IS 'Private status';
COMMENT ON COLUMN "group"."members" IS 'Member count';
COMMENT ON COLUMN "group"."cityid" IS 'City ID';
COMMENT ON COLUMN "group"."organizer" IS 'Organizer name';
COMMENT ON COLUMN "group"."highres_link" IS 'High-res photo link';
COMMENT ON COLUMN "group"."photo_link" IS 'Photo link';
COMMENT ON COLUMN "group"."thumb_link" IS 'Thumbnail link';
COMMENT ON COLUMN "group"."created_at" IS 'Creation timestamp';
COMMENT ON COLUMN "group"."updated_at" IS 'Last update timestamp';