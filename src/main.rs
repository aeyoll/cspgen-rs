#[macro_use]
extern crate clap;
use clap::App;

use headless_chrome::browser::tab::RequestInterceptionDecision;
use headless_chrome::protocol::network::methods::RequestPattern;
use headless_chrome::Browser;
use std::sync::{Arc, Mutex};
use url::Url;

mod csp;

use csp::CSP;

fn dedup(mut vec: Vec<String>) -> Vec<String> {
    vec.sort_unstable();
    vec.dedup();

    vec
}

fn generate_csp(url: String) -> Result<CSP, failure::Error> {
    let browser = Browser::default()?;
    let tab = browser.wait_for_initial_tab()?;

    let patterns = vec![RequestPattern {
        url_pattern: Some("*"),
        resource_type: None,
        interception_stage: Some("Request"),
    }];

    let main_url = Url::parse(url.as_str())?;
    let main_host = String::from(main_url.host_str().unwrap());

    let javascripts = Arc::new(Mutex::new(Vec::new()));
    let fonts = Arc::new(Mutex::new(Vec::new()));
    let images = Arc::new(Mutex::new(Vec::new()));
    let styles = Arc::new(Mutex::new(Vec::new()));
    let connects = Arc::new(Mutex::new(Vec::new()));

    let javascripts2 = javascripts.clone();
    let fonts2 = fonts.clone();
    let images2 = images.clone();
    let styles2 = styles.clone();
    let connects2 = connects.clone();

    tab.enable_request_interception(
        &patterns,
        Box::new(move |_transport, _session_id, intercepted| {
            let url = Url::parse(intercepted.request.url.as_str()).unwrap();
            let resource_type = intercepted.resource_type.as_str();
            let host = String::from(url.host_str().unwrap());

            if host != main_host {
                match resource_type {
                    "Image" => images2.lock().unwrap().push(host),
                    "Script" => javascripts2.lock().unwrap().push(host),
                    "Font" => fonts2.lock().unwrap().push(host),
                    "Stylesheet" => styles2.lock().unwrap().push(host),
                    "XHR" => connects2.lock().unwrap().push(host),
                    _ => {}
                };
            }

            RequestInterceptionDecision::Continue
        }),
    )?;

    tab.navigate_to(&url)?;
    tab.wait_until_navigated()?;

    let javascripts = dedup(javascripts.lock().unwrap().to_vec());
    let fonts = dedup(fonts.lock().unwrap().to_vec());
    let images = dedup(images.lock().unwrap().to_vec());
    let styles = dedup(styles.lock().unwrap().to_vec());
    let connects = dedup(connects.lock().unwrap().to_vec());

    let csp = CSP {
        javascripts,
        fonts,
        images,
        styles,
        connects,
    };

    Ok(csp)
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(url) = matches.value_of("URL") {
        match generate_csp(String::from(url)) {
            Ok(csp) => println!("{}", csp),
            Err(e) => println!("error {:?}", e),
        }
    }
}
