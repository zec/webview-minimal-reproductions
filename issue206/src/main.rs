/*
   2021 Zachary Catlin;
   avaliable under the terms of the Unlicense (see ../../UNLICENSE)
*/

extern crate web_view;

use web_view::WVResult;

fn main() {
    let result = run_webview();

    if let Err(ref e) = result {
        eprintln!("Error in building or running WebView: {}", e);
        std::process::exit(1);
    }
}

const HTML_CONTENT: &'static str = include_str!("../content.html");

fn run_webview() -> WVResult<()> {
    use web_view::*;

    let mut wv = WebViewBuilder::new()
        .title("Bug reproduction example")
        .content(Content::Url("data:text/plain,%20"))
        .user_data(())
        .invoke_handler(|wv, _arg| { println!("exiting"); wv.exit(); Ok(())})
        .build()?;

    wv.set_html(HTML_CONTENT)?;
    wv.run()?;

    Ok(())
}
