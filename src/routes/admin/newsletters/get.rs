use std::fmt::Write;

use actix_web::{http::header::ContentType, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;

pub async fn post_newsletter_form(
    flash_message: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    let mut msg_html = String::new();
    for m in flash_message.iter() {
        writeln!(msg_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }
    let idempotency_key = uuid::Uuid::new_v4().to_string();
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta http-equiv="content-type" content="text/html; charset=utf-8">
                <title>Post a Newsletter</title>
            </head>
            <body>
                {msg_html}
                <form action="/admin/newsletters" method="post">
                    <label>
                        Title
                        <input
                            type="text"
                            placeholder="Enter Title"
                            name="title"
                        >
                    </label>
                    <br>
                    <label>
                        Text Content
                        <input
                            type="text"
                            placeholder="Enter Text Content"
                            name="text_content"
                        >
                    </label>
                    <br>
                    <label>
                        HTML Content
                        <input
                            type="text"
                            placeholder="Enter HTML Content"
                            name="html_content"
                        >
                    </label>
                    <br>
                    <input type="hidden" name="idempotency_key" value="{idempotency_key}>
                    <button type="submit">Send</button>
                </form>
                <p><a href="/admin/dashboard">&lt; - Back</a></p>
            </body>
        </html>
        "#,
        )))
}
