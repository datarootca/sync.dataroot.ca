-- Table Definition
CREATE TABLE "public"."organizer" (
    "organizerid" SERIAL NOT NULL,
    "firstname" varchar NOT NULL,
    "lastname" varchar NOT NULL,
    "created_at" timestamptz NOT NULL DEFAULT now(),
    "updated_at" timestamptz,
    "extid" varchar NOT NULL UNIQUE,
    "bio" varchar NOT NULL,
    "highres_link" varchar,
    "photo_link" varchar,
    "thumb_link" varchar,
    PRIMARY KEY ("organizerid")
);

-- Column Comment
COMMENT ON COLUMN "organizer"."highres_link" IS 'High-res photo link';
COMMENT ON COLUMN "organizer"."photo_link" IS 'Photo link';
COMMENT ON COLUMN "organizer"."thumb_link" IS 'Thumbnail link';
COMMENT ON COLUMN "organizer"."created_at" IS 'Creation timestamp';
COMMENT ON COLUMN "organizer"."updated_at" IS 'Last update timestamp';