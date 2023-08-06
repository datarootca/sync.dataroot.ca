CREATE TABLE "event_social" (
    "event_socialid" SERIAL NOT NULL,
    "eventid" int(8) NOT NULL UNIQUE,
    "source" varchar NOT NULL,
    "link" varchar NOT NULL,
    "created_at" timestamptz DEFAULT now(),
    PRIMARY KEY ("eventid","categoryid")
);

-- Column Comment
COMMENT ON COLUMN "event_social"."event_socialid" IS 'Primary key';
COMMENT ON COLUMN "event_social"."eventid" IS 'Event id';
COMMENT ON COLUMN "event_social"."source" IS 'platform';
COMMENT ON COLUMN "event_social"."link" IS 'URL link';
COMMENT ON COLUMN "event_social"."created_at" IS 'Creation timestamp';
COMMENT ON COLUMN "event_social"."updated_at" IS 'Last update timestamp';
