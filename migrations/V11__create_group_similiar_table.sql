CREATE TABLE "group_similiar" (
    "groupid" int8 NOT NULL UNIQUE,
    "parent" int8 NOT NULL UNIQUE,
    "created_at" timestamptz DEFAULT now(),
    PRIMARY KEY ("groupid","parent")
);

-- Column Comment
COMMENT ON COLUMN "group_similiar"."groupid" IS 'group id';
COMMENT ON COLUMN "group_similiar"."parent" IS 'group similiar id';
COMMENT ON COLUMN "group_similiar"."created_at" IS 'Creation timestamp';
