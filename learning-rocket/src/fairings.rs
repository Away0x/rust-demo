use rocket::{
    fairing::{Fairing, Info, Kind},
    http::Method,
    serde::json::{serde_json::json, Value},
};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

/// Rocket 的中间件
///
/// Fairing Callbacks
/// 1. Ignite(on_ignite) 启动时
/// 2. Liftoff(on_liftoff) 启动后
/// 3. Request(on_request) 收到请求后
/// 3. Response(on_response) 响应前

#[derive(Clone, Default)]
pub struct Counter {
    num: Arc<AtomicUsize>,
}

#[rocket::async_trait]
impl Fairing for Counter {
    fn info(&self) -> rocket::fairing::Info {
        Info {
            name: "Get Counter",
            kind: Kind::Ignite | Kind::Request, // 该 fairing 需要处理这两个 hook
        }
    }

    async fn on_ignite(&self, rocket: rocket::Rocket<rocket::Build>) -> rocket::fairing::Result {
        // 创建一个路由，获取 count
        #[get("/count")]
        fn counts(counts: &rocket::State<Counter>) -> Value {
            let count_num = counts.num.load(Ordering::Relaxed);
            json!({
                "status": "ok",
                "count": count_num,
            })
        }

        Ok(rocket
            .manage(self.clone())
            .mount("/", rocket::routes![counts]))
    }

    async fn on_request(&self, req: &mut rocket::Request<'_>, _data: &mut rocket::Data<'_>) {
        // 每次 get 请求 count +1
        if req.method() == Method::Get {
            self.num.fetch_add(1, Ordering::Relaxed);
        }
    }
}
