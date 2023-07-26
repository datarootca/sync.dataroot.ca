CREATE TABLE "city" (
    "cityid" SERIAL NOT NULL,
    "created_at" timestamptz DEFAULT now(),
    "updated_at" timestamptz,
    "extid" varchar NOT NULL UNIQUE,
    "name" varchar NOT NULL UNIQUE,
    "slug" varchar NOT NULL UNIQUE,
    "stateid" INTEGER NOT NULL,
    "highres_link" varchar,
    "photo_link" varchar,
    "thumb_link" varchar,
    PRIMARY KEY ("cityid")
);

-- Column Comment
COMMENT ON COLUMN "city"."cityid" IS 'Primary key';
COMMENT ON COLUMN "city"."created_at" IS 'Creation timestamp';
COMMENT ON COLUMN "city"."updated_at" IS 'Last update timestamp';
COMMENT ON COLUMN "city"."extid" IS 'External identifier';
COMMENT ON COLUMN "city"."name" IS 'City name';
COMMENT ON COLUMN "city"."slug" IS 'URL slug';
COMMENT ON COLUMN "city"."stateid" IS 'State ID';
COMMENT ON COLUMN "city"."highres_link" IS 'High-res photo link';
COMMENT ON COLUMN "city"."photo_link" IS 'Photo link';
COMMENT ON COLUMN "city"."thumb_link" IS 'Thumbnail link';

INSERT INTO "city" ("cityid", "created_at", "updated_at", "extid", "name", "slug", "stateid") VALUES
(1, '2023-06-18 19:07:21.914266', NULL, '1', 'Airdrie', 'airdrie', 1),
(2, '2023-06-18 19:07:21.914266', NULL, '2', 'Cochrane', 'cochrane', 1),
(3, '2023-06-18 19:07:21.914266', NULL, '3', 'Edmonton', 'edmonton', 1),
(4, '2023-06-18 19:07:21.914266', NULL, '4', 'Leduc', 'leduc', 1),
(5, '2023-06-18 19:07:21.914266', NULL, '5', 'Grande Prairie', 'grande-prairie', 1),
(6, '2023-06-18 19:07:21.914266', NULL, '6', 'Red Deer
', 'red-deer', 1),
(7, '2023-06-18 19:07:21.914266', NULL, '7', 'Vancouver', 'vancouver', 2),
(8, '2023-06-18 19:07:21.914266', NULL, '8', 'Victoria', 'victoria', 2),
(9, '2023-06-18 19:07:21.914266', NULL, '9', 'Chilliwack', 'chilliwack', 2),
(10, '2023-06-18 19:07:21.914266', NULL, '10', 'Penticton
', 'penticton
', 2),
(11, '2023-06-18 19:07:21.914266', NULL, '11', 'Surrrey', 'surrrey', 2),
(12, '2023-06-18 19:07:21.914266', NULL, '12', 'Prince Rupert', 'prince-rupert', 2),
(13, '2023-06-18 19:07:21.914266', NULL, '13', 'Brandon', 'brandon', 3),
(14, '2023-06-18 19:07:21.914266', NULL, '14', 'Steinbach', 'steinbach', 3),
(15, '2023-06-18 19:07:21.914266', NULL, '15', 'Winnipeg', 'winnipeg', 3),
(16, '2023-06-18 19:47:10.6576', NULL, '16', 'Fredericton', 'fredericton', 4),
(17, '2023-06-18 19:47:10.6576', NULL, '17', 'Moncton', 'moncton', 4),
(18, '2023-06-18 19:47:10.6576', NULL, '18', 'Saint John', 'saint-john', 4),
(19, '2023-06-18 19:47:10.6576', NULL, '19', 'Mount Pearl', 'mount-pearl', 5),
(20, '2023-06-18 19:47:10.6576', NULL, '20', 'St. John''s', 'st-johns', 5),
(21, '2023-06-18 19:47:10.6576', NULL, '21', 'Halifax', 'halifax', 6),
(22, '2023-06-18 19:47:10.6576', NULL, '22', 'Sydney', 'sydney', 6),
(23, '2023-06-18 19:47:10.6576', NULL, '23', 'Dartmouth', 'dartmouth', 6),
(24, '2023-06-18 19:47:10.6576', NULL, '24', 'Corner Brook', 'corner-brook', 5),
(25, '2023-06-18 19:47:10.6576', NULL, '25', 'Toronto', 'toronto', 7),
(26, '2023-06-18 19:47:10.6576', NULL, '26', 'Ottawa
', 'ottawa
', 7),
(27, '2023-06-18 19:47:10.6576', NULL, '27', 'Mississauga', 'mississauga', 7),
(28, '2023-06-18 19:47:10.6576', NULL, '28', 'Charlottetown', 'charlottetown', 8),
(29, '2023-06-18 19:47:10.6576', NULL, '29', 'Summerside', 'summerside', 8),
(30, '2023-06-18 19:47:10.6576', NULL, '30', 'Stratford', 'stratford', 8),
(31, '2023-06-18 19:47:10.6576', NULL, '31', 'Montreal', 'montreal', 9),
(32, '2023-06-18 19:47:10.6576', NULL, '32', 'Quebec City', 'quebec-city', 9),
(33, '2023-06-18 19:47:10.6576', NULL, '33', 'Laval', 'laval', 9),
(34, '2023-06-18 19:07:21.914266', NULL, '34', 'Prince Albert', 'prince-albert', 10),
(35, '2023-06-18 19:07:21.914266', NULL, '35', 'Saskatoon', 'saskatoon', 10),
(36, '2023-06-18 19:07:21.914266', NULL, '36', 'Regina', 'regina', 10);

SELECT setval('city_cityid_seq', 37, true);