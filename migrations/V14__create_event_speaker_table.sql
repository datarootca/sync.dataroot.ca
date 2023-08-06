CREATE TABLE "event_speaker" (
    "eventid" int8 NOT NULL UNIQUE,
    "speakerid" int8 NOT NULL UNIQUE,
    "created_at" timestamptz DEFAULT now(),
    PRIMARY KEY ("eventid","speakerid")
);

-- Column Comment
COMMENT ON COLUMN "event_speaker"."eventid" IS 'event id';
COMMENT ON COLUMN "event_speaker"."speakerid" IS 'speaker id';
COMMENT ON COLUMN "event_speaker"."created_at" IS 'Creation timestamp';
