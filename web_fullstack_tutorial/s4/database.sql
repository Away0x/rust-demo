DROP TABLE IF EXISTS ezy_course_c4;

CREATE TABLE ezy_course_c4 (
    course_id serial PRIMARY KEY,
    tutor_id INT NOT NULL,
    course_name varchar(140) NOT NULL,
    posted_time TIMESTAMP DEFAULT NOW()
);

INSERT INTO
    ezy_course_c4 (course_id, tutor_id, course_name, posted_time)
VALUES
    (1, 1, 'First course', '2020-12-17 05:40:00');

INSERT INTO
    ezy_course_c4 (course_id, tutor_id, course_name, posted_time)
VALUES
    (2, 1, 'Second course', '2020-12-18 05:45:00');