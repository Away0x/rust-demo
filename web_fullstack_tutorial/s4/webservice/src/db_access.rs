use super::errors::MyError;
use super::models::Course;
use chrono;
use sqlx::postgres::PgPool;

pub async fn get_courses_for_tutor_db(
    pool: &PgPool,
    tutor_id: i32,
) -> Result<Vec<Course>, MyError> {
    let course_rows = sqlx::query!(
        r#"SELECT tutor_id, course_id, course_name, posted_time
        FROM ezy_course_c4
        WHERE tutor_id = $1"#,
        tutor_id,
    )
    .fetch_all(pool)
    .await?;

    let courses: Vec<Course> = course_rows
        .iter()
        .map(|course_row| Course {
            course_id: course_row.course_id,
            tutor_id: course_row.tutor_id,
            course_name: course_row.course_name.clone(),
            posted_time: Some(chrono::NaiveDateTime::from(course_row.posted_time.unwrap())),
        })
        .collect();

    match courses.len() {
        0 => Err(MyError::NotFound("Courses not found for teacher".into())),
        _ => Ok(courses),
    }
}

pub async fn get_course_details_db(
    pool: &PgPool,
    tutor_id: i32,
    course_id: i32,
) -> Result<Course, MyError> {
    let row = sqlx::query!(
        r#"SELECT tutor_id, course_id, course_name, posted_time 
        FROM ezy_course_c4 
        WHERE tutor_id = $1 AND course_id = $2"#,
        tutor_id,
        course_id
    )
    .fetch_one(pool)
    .await;

    if let Ok(row) = row {
        Ok(Course {
            course_id: row.course_id,
            tutor_id: row.tutor_id,
            course_name: row.course_name.clone(),
            posted_time: Some(chrono::NaiveDateTime::from(row.posted_time.unwrap())),
        })
    } else {
        Err(MyError::NotFound("Course not found".into()))
    }
}

pub async fn post_new_course_db(pool: &PgPool, new_course: Course) -> Result<Course, MyError> {
    let course_row = sqlx::query!(
        r#"INSERT INTO ezy_course_c4 (course_id, tutor_id, course_name) 
        values ($1, $2, $3) 
        returning tutor_id, course_id, course_name, posted_time"#,
        new_course.course_id,
        new_course.tutor_id,
        new_course.course_name
    )
    .fetch_one(pool)
    .await?;

    Ok(Course {
        course_id: course_row.course_id,
        tutor_id: course_row.tutor_id,
        course_name: course_row.course_name.clone(),
        posted_time: Some(chrono::NaiveDateTime::from(course_row.posted_time.unwrap())),
    })
}
