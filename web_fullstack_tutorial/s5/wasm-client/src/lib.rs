use wasm_bindgen::prelude::*;
pub mod errors;
pub mod models;

use models::course::Course;

#[wasm_bindgen(start)] // start 表示项目一启动就会执行这个函数
pub async fn main() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global window exists");
    let document = window.document().expect("no global document exists");

    let left_tbody = document
        .get_element_by_id("left-tbody")
        .expect("left_div_not_exists");

    let courses: Vec<Course> = models::course::get_courses_by_teacher(1).await.unwrap();
    for c in courses.iter() {
        let tr = document.create_element("tr")?;
        tr.set_attribute("id", format!("tr-{}", c.id).as_str())?;
        let id = document.create_element("id")?;
        id.set_text_content(Some(format!("{}", c.id).as_str()));
        tr.append_child(&id)?;

        let td = document.create_element("td")?;
        td.set_text_content(Some(c.name.as_str()));
        tr.append_child(&td)?;

        let td = document.create_element("td")?;
        td.set_text_content(Some(c.time.format("%Y-%m-%d").to_string().as_str()));
        tr.append_child(&td)?;

        let td = document.create_element("td")?;
        if let Some(desc) = c.description.clone() {
            td.set_text_content(Some(desc.as_str()));
        }
        tr.append_child(&td)?;

        let td = document.create_element("td")?;
        let btn = document.create_element("button")?;
        btn.set_attribute("type", "button")?;
        btn.set_attribute("class", "btn btn-danger btn-sm")?;
        btn.set_text_content(Some("Delete"));
        td.append_child(&btn)?;
        tr.append_child(&td)?;

        left_tbody.append_child(&tr)?;
    }

    Ok(())
}