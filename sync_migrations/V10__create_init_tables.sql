CREATE TABLE "diff_article" (
    "key" varchar NOT NULL,
    "value" varchar,
    PRIMARY KEY ("key")
);

CREATE TABLE "diff_group" (
    "key" varchar NOT NULL,
    "value" varchar,
    PRIMARY KEY ("key")
);

CREATE TABLE "diff_event" (
    "key" varchar NOT NULL,
    "value" varchar,
    PRIMARY KEY ("key")
);

CREATE TABLE "registered_author" (
    "registered_authorid" SERIAL NOT NULL,
    "source" varchar NOT NULL,
    "name" varchar NOT NULL,
    "created_at" timestamptz NOT NULL DEFAULT now(),
    "updated_at" timestamptz,
    PRIMARY KEY ("registered_authorid")
);

insert into registered_author(name,source) values ('tkudlicka','medium'),('adamrossnelson','medium'),('kozyrkov','medium'),('mark_45452','medium'),('barrmoses','medium'),('afaqueumer','medium'),('thepycoach','medium'),('mark_45452','medium'),('ignacio.de.gregorio.noblejas','medium'),('slgero','medium'),('cornelliusyudhawijaya','medium'),('armanmadani','medium'),('clemensm','medium'),('davilirio99','medium'),('mastafa.foufa','medium'),('cagricebisli','medium'),('andymcdonaldgeo','medium'),('ramiromedina','medium'),('kozyrkov','medium'),('xinran.waibel','medium'),('jackblandin','medium'),('stabrak.abbes_65953','medium'),('lucianosphere','medium'),('mtths.mndr','medium'),('tim-lou','medium'),('ben.putney','medium');

CREATE TABLE "registered_group" (
    "registered_groupid" SERIAL NOT NULL,
    "source" varchar NOT NULL,
    "name" varchar NOT NULL,
    "created_at" timestamptz NOT NULL DEFAULT now(),
    "updated_at" timestamptz,
    PRIMARY KEY ("registered_groupid")
);