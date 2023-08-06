CREATE TABLE "group_category" (
    "groupid" int8 NOT NULL UNIQUE,
    "categoryid" int8 NOT NULL UNIQUE,
    "created_at" timestamptz DEFAULT now(),
    PRIMARY KEY ("groupid","categoryid")
);

-- Column Comment
COMMENT ON COLUMN "group_category"."groupid" IS 'group id';
COMMENT ON COLUMN "group_category"."categoryid" IS 'Category id';
COMMENT ON COLUMN "group_category"."created_at" IS 'Creation timestamp';
