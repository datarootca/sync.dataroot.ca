CREATE TABLE "group_social" (
    "group_socialid" SERIAL NOT NULL,
    "groupid" int(8) NOT NULL UNIQUE,
    "source" varchar NOT NULL,
    "link" varchar NOT NULL,
    "created_at" timestamptz DEFAULT now(),
    PRIMARY KEY ("eventid","categoryid")
);

-- Column Comment
COMMENT ON COLUMN "group_social"."group_socialid" IS 'Primary key';
COMMENT ON COLUMN "group_social"."eventid" IS 'Group id';
COMMENT ON COLUMN "group_social"."source" IS 'platform';
COMMENT ON COLUMN "group_social"."link" IS 'URL link';
COMMENT ON COLUMN "group_social"."created_at" IS 'Creation timestamp';
COMMENT ON COLUMN "group_social"."updated_at" IS 'Last update timestamp';
