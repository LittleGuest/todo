CREATE TABLE "project" (
	"id"	INTEGER NOT NULL,
	"name"	TEXT NOT NULL UNIQUE,
	"cover"	TEXT DEFAULT "",
	"start_time"	INTEGER,
	"end_time"	INTEGER,
	"status"	INTEGER,
	"create_at"	INTEGER,
	"update_at"	INTEGER,
	PRIMARY KEY("id")
);


CREATE TABLE "task" (
	"id"	INTEGER NOT NULL,
	"project_id"	INTEGER NOT NULL,
	"pid"	INTEGER NOT NULL DEFAULT 0,
	"title"	TEXT NOT NULL,
	"desc"	TEXT DEFAULT "",
	"priority"	INTEGER NOT NULL,
	"tags"	TEXT DEFAULT "",
	"start_time"	INTEGER NOT NULL,
	"end_time"	INTEGER NOT NULL,
	"status"	INTEGER NOT NULL,
	"create_at"	INTEGER,
	"update_at"	INTEGER,
	PRIMARY KEY("id")
);