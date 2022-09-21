use clap::Parser;
use headless_chrome::browser::tab::RequestInterceptionDecision;
use headless_chrome::protocol::network::methods::RequestPattern;
use headless_chrome::Browser;
use merge::Merge;
use std::sync::{Arc, Mutex};
use url::Url;

mod csp;

use csp::CSP;


#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Args {
   /// The url to generate the CSP from
   #[clap(value_parser)]
   urls: Vec<String>,
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
    let iframes = Arc::new(Mutex::new(Vec::new()));

    let javascripts2 = javascripts.clone();
    let fonts2 = fonts.clone();
    let images2 = images.clone();
    let styles2 = styles.clone();
    let connects2 = connects.clone();
    let iframes2 = iframes.clone();

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
                    "Document" => iframes2.lock().unwrap().push(host),
                    _ => {}
                };
            }

            RequestInterceptionDecision::Continue
        }),
    )?;

    tab.navigate_to(&url)?;
    tab.wait_until_navigated()?;

    let javascripts = javascripts.lock().unwrap().to_vec();
    let fonts = fonts.lock().unwrap().to_vec();
    let images = images.lock().unwrap().to_vec();
    let styles = styles.lock().unwrap().to_vec();
    let connects = connects.lock().unwrap().to_vec();
    let iframes = iframes.lock().unwrap().to_vec();

    let csp = CSP {
        javascripts,
        fonts,
        images,
        styles,
        connects,
        iframes,
    };

    Ok(csp)
}

fn main() {
   let args = Args::parse();

    let urls = args.urls;

    let mut csps = CSP {
        javascripts: Vec::new(),
        fonts: Vec::new(),
        images: Vec::new(),
        styles: Vec::new(),
        connects: Vec::new(),
        iframes: Vec::new(),
    };

    for url in urls {
        match generate_csp(String::from(url)) {
            Ok(csp) => csps.merge(csp),
            Err(e) => println!("error {:?}", e),
        }
    }

    println!("{}", csps);
}
