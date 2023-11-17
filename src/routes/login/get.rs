use actix_web::{cookie::Cookie, http::header::ContentType, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

pub async fn login_form(flash_messages: IncomingFlashMessages) -> HttpResponse {
    let mut html_msg = String::new();
    for m in flash_messages.iter() {
        writeln!(html_msg, "<p><i>{}</i></p>", m.content()).unwrap();
    }

    let mut response = HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"
            <!DOCTYPE html>
            <html lang="en">
            <head>
                <meta http-equiv="content-type" content="text/html; charset=utf-8">
                <title>Login</title>
            </head>
            <body>
                {html_msg}
                <form action="/login" method="post">
                <label>
                    Username
                    <input
                    type="text"
                    placeholder="Enter username"
                    name="username"
                    >
                </label>
                <label>
                    Password
                    <input
                    type="password"
                    placeholder="Enter Password"
                    name="password"
                    >
                </label>
                <button type="submit">Login</button>
                </form>
            </body>
            </html>
            "#,
        ));
    response
        .add_removal_cookie(&Cookie::new("_flash", ""))
        .unwrap();
    response
}
