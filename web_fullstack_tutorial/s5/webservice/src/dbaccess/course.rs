use crate::errors::MyError;
use crate::models::course::{Course, CreateCourse, UpdateCourse};
use sqlx::postgres::PgPool;

pub async fn get_courses_for_teacher_db(
    pool: &PgPool,
    teacher_id: i32,
) -> Result<Vec<Course>, MyError> {
    let rows = sqlx::query_as!(
        Course,
        r#"SELECT * FROM course WHERE teacher_id = $1"#,
        teacher_id,
    )
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn get_course_details_db(
    pool: &PgPool,
    teacher_id: i32,
    course_id: i32,
) -> Result<Course, MyError> {
    let row = sqlx::query_as!(
        Course,
        r#"SELECT * FROM course 
        WHERE teacher_id = $1 AND id = $2"#,
        teacher_id,
        course_id
    )
    .fetch_optional(pool)
    .await?;

    if let Some(course) = row {
        Ok(course)
    } else {
        Err(MyError::NotFound("Course not found".into()))
    }
}

pub async fn post_new_course_db(
    pool: &PgPool,
    new_course: CreateCourse,
) -> Result<Course, MyError> {
    let row = sqlx::query_as!(
        Course,
        r#"INSERT INTO course (teacher_id, name, description, format, structure, duration, price, language, level) 
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) 
        RETURNING id, teacher_id, name, time, description, format, structure, duration, price, language, level"#,
        new_course.teacher_id,
        new_course.name,
        new_course.description,
        new_course.format,
        new_course.structure,
        new_course.duration,
        new_course.price,
        new_course.language,
        new_course.level,
    )
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn delete_course_db(pool: &PgPool, teacher_id: i32, id: i32) -> Result<String, MyError> {
    let row = sqlx::query!(
        "DELETE FROM course WHERE teacher_id = $1 AND id = $2",
        teacher_id,
        id,
    )
    .execute(pool)
    .await?;

    Ok(format!("Delete {:?} record", row))
}

pub async fn update_course_details_db(
    pool: &PgPool,
    teacher_id: i32,
    id: i32,
    update_course: UpdateCourse,
) -> Result<Course, MyError> {
    let row = sqlx::query_as!(
        Course,
        "SELECT * FROM course WHERE teacher_id = $1 AND id = $2",
        teacher_id,
        id,
    )
    .fetch_one(pool)
    .await
    .map_err(|_err| MyError::NotFound("Course not found".into()))?;

    let name: String = if let Some(name) = update_course.name {
        name
    } else {
        row.name
    };

    let description: String = if let Some(description) = update_course.description {
        description
    } else {
        row.description.unwrap_or_default()
    };

    let format: String = if let Some(format) = update_course.format {
        format
    } else {
        row.format.unwrap_or_default()
    };

    let structure: String = if let Some(structure) = update_course.structure {
        structure
    } else {
        row.structure.unwrap_or_default()
    };

    let duration: String = if let Some(duration) = update_course.duration {
        duration
    } else {
        row.duration.unwrap_or_default()
    };

    let language: String = if let Some(language) = update_course.language {
        language
    } else {
        row.language.unwrap_or_default()
    };

    let level: String = if let Some(level) = update_course.level {
        level
    } else {
        row.level.unwrap_or_default()
    };

    let price: i32 = if let Some(price) = update_course.price {
        price
    } else {
        row.price.unwrap_or_default()
    };

    let update_row = sqlx::query_as!(
        Course,
        r#"UPDATE course SET name = $1, description = $2, format = $3,
        structure = $4, duration = $5, price = $6, language = $7, level = $8
        WHERE teacher_id = $9 AND id = $10
        RETURNING id, teacher_id, name, time, description, format, structure, duration, price, language, level"#,
        name,
        description,
        format,
        structure,
        duration,
        price,
        language,
        level,
        teacher_id,
        id,
    )
    .fetch_one(pool)
    .await;

    if let Ok(course) = update_row {
        Ok(course)
    } else {
        Err(MyError::NotFound("Course not found".into()))
    }
}
