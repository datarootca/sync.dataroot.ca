-- Table Definition
CREATE TABLE "event" (
    "eventid" SERIAL NOT NULL,
    "name" varchar NOT NULL,
    "description" varchar,
    "extid" varchar NOT NULL UNIQUE,
    "location" varchar NOT NULL,
    "groupid" INTEGER NOT NULL,
    "in_person" bool NOT NULL,
    "time" timestamptz NOT NULL,
    "duration" int4 NOT NULL,
    "link" varchar NOT NULL,
    "waitlist_count" int4,
    "rsvp_limit" int4,
    "is_online" bool NOT NULL,
    "yes_rsvp_count" int4 NOT NULL,
    "fee" bool NOT NULL,
    "highres_link" varchar,
    "photo_link" varchar,
    "thumb_link" varchar,
    "created_at" timestamptz NOT NULL DEFAULT now(),
    "updated_at" timestamptz,
    PRIMARY KEY ("eventid")
);

-- Column Comment
COMMENT ON COLUMN "event"."eventid" IS 'Primary key';
COMMENT ON COLUMN "event"."name" IS 'Event title';
COMMENT ON COLUMN "event"."description" IS 'Description';
COMMENT ON COLUMN "event"."created_at" IS 'Creation timestamp';
COMMENT ON COLUMN "event"."updated_at" IS 'Last update timestamp';
COMMENT ON COLUMN "event"."extid" IS 'External identifier';
COMMENT ON COLUMN "event"."location" IS 'Event URL or address';
COMMENT ON COLUMN "event"."groupid" IS 'Group ID';
COMMENT ON COLUMN "event"."in_person" IS 'In-person event status';
COMMENT ON COLUMN "event"."time" IS 'Event start time';
COMMENT ON COLUMN "event"."duration" IS 'Event duration';
COMMENT ON COLUMN "event"."link" IS 'Event URL';
COMMENT ON COLUMN "event"."waitlist_count" IS 'Waitlist count';
COMMENT ON COLUMN "event"."is_online" IS 'Online event status';
COMMENT ON COLUMN "event"."yes_rsvp_count" IS 'Attendance count';
COMMENT ON COLUMN "event"."fee" IS 'Event fee status';
COMMENT ON COLUMN "event"."highres_link" IS 'High-res photo link';
COMMENT ON COLUMN "event"."photo_link" IS 'Photo link';
COMMENT ON COLUMN "event"."thumb_link" IS 'Thumbnail link';