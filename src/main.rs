// use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

// #[get("/")]
// async fn welcome() -> impl Responder {
//     HttpResponse::Ok().body("Hello!")
// }

// #[get("/test")]
// async fn test() -> impl Responder {
//     HttpResponse::Ok().body("This is a test")
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     //add a print message to the console that the service is running
//     println!("Starting service...");
//     HttpServer::new(|| {
//         App::new()
//             .service(welcome)
//             .service(test)

//     })
//         .bind("0.0.0.0:8080")?
//         .run()
//         .await
// }

use cpython::{Python, PyDict, PyResult};

fn main() {
    let gil = Python::acquire_gil();
    hello(gil.python()).unwrap();
}

fn hello(py: Python) -> PyResult<()> {
    let sys = py.import("sys")?;
    let version: String = sys.get(py, "version")?.extract(py)?;

    let locals = PyDict::new(py);
    locals.set_item(py, "os", py.import("os")?)?;
    let user: String = py.eval("os.getenv('USER') or os.getenv('USERNAME')", None, Some(&locals))?.extract(py)?;

    println!("Hello {user}, I'm Python {version}");
    Ok(())
}
