DROP TABLE IF EXISTS course;

CREATE TABLE course (
    id serial PRIMARY KEY,
    teacher_id INT NOT NULL,
    name varchar(140) NOT NULL,
    description varchar(2000),
    format varchar(30),
    structure varchar(200),
    duration varchar(30),
    price INT,
    language varchar(30),
    level varchar(30),
    time TIMESTAMP DEFAULT NOW()
);

INSERT INTO "public"."course" ("id", "teacher_id", "name", "description", "format", "structure", "duration", "price", "language", "level", "time") VALUES
(1, 1, 'Test course', 'This is a course', NULL, NULL, NULL, NULL, 'English', 'Beginner', '2022-05-05 14:50:54.987483'),
(2, 1, 'Course name changed', 'This is another test course', '', '', '', 0, 'Chinese', 'Intermediate', '2022-05-05 14:51:08.401111'),
(3, 1, 'Test course', 'This is a course', NULL, NULL, NULL, NULL, 'English', 'Beginner', '2022-05-05 14:51:40.544948');