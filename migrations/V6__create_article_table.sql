
CREATE TABLE "article" (
    "articleid" SERIAL NOT NULL,
    "extid" varchar NOT NULL UNIQUE,
    "name" varchar NOT NULL,
    "description" varchar NOT NULL,
    "time_m" int4,
    "publish_at" timestamptz NOT NULL,
    "source" varchar NOT NULL,
    "link" varchar NOT NULL UNIQUE,
    "author" varchar NOT NULL,
    "highres_link" varchar,
    "photo_link" varchar,
    "thumb_link" varchar,
    "created_at" timestamptz NOT NULL DEFAULT now(),
    "updated_at" timestamptz,
    PRIMARY KEY ("articleid")
);

-- Column Comment
COMMENT ON COLUMN "article"."articleid" IS 'Primary key';
COMMENT ON COLUMN "article"."extid" IS 'External identifier';
COMMENT ON COLUMN "article"."name" IS 'Article title';
COMMENT ON COLUMN "article"."description" IS 'Description';
COMMENT ON COLUMN "article"."time_m" IS 'Article duration';
COMMENT ON COLUMN "article"."publish_at" IS 'Publication date';
COMMENT ON COLUMN "article"."source" IS 'Source name';
COMMENT ON COLUMN "article"."link" IS 'Article link';
COMMENT ON COLUMN "article"."author" IS 'Author name';
COMMENT ON COLUMN "article"."highres_link" IS 'High-res photo link';
COMMENT ON COLUMN "article"."photo_link" IS 'Photo link';
COMMENT ON COLUMN "article"."thumb_link" IS 'Thumbnail link';
COMMENT ON COLUMN "article"."created_at" IS 'Creation timestamp';
COMMENT ON COLUMN "article"."updated_at" IS 'Last update timestamp';