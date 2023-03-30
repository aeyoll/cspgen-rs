use clap::Parser;
use headless_chrome::browser::tab::RequestPausedDecision;
use headless_chrome::protocol::cdp::Fetch::events::RequestPausedEvent;
use headless_chrome::protocol::cdp::Fetch::{RequestPattern, RequestStage};
use headless_chrome::protocol::cdp::Network::ResourceType;
use headless_chrome::Browser;
use merge::Merge;
use std::sync::{Arc, Mutex};
use url::Url;

mod csp;

use csp::Csp;

#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Args {
    /// The url to generate the Csp from
    #[clap(value_parser)]
    urls: Vec<String>,
}

fn generate_csp(url: String) -> Result<Csp, anyhow::Error> {
    let browser = Browser::default()?;
    let tab = browser.new_tab()?;

    let patterns = vec![RequestPattern {
        url_pattern: Some(String::from("*")),
        resource_Type: None,
        request_stage: Some(RequestStage::Request),
    }];

    tab.enable_fetch(Some(&patterns), None)?;

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

    tab.enable_request_interception(Arc::new(
        move |_transport, _session_id, intercepted: RequestPausedEvent| {
            let url = Url::parse(intercepted.params.request.url.as_str()).unwrap();
            let resource_type = intercepted.params.resource_Type;
            let host = String::from(url.host_str().unwrap());

            if host != main_host {
                match resource_type {
                    ResourceType::Image => images2.lock().unwrap().push(host),
                    ResourceType::Script => javascripts2.lock().unwrap().push(host),
                    ResourceType::Font => fonts2.lock().unwrap().push(host),
                    ResourceType::Stylesheet => styles2.lock().unwrap().push(host),
                    ResourceType::Xhr => connects2.lock().unwrap().push(host),
                    ResourceType::Document => iframes2.lock().unwrap().push(host),
                    _ => {}
                };
            }

            RequestPausedDecision::Continue(None)
        },
    ))?;

    tab.navigate_to(&url)?;
    tab.wait_until_navigated()?;

    let javascripts = javascripts.lock().unwrap().to_vec();
    let fonts = fonts.lock().unwrap().to_vec();
    let images = images.lock().unwrap().to_vec();
    let styles = styles.lock().unwrap().to_vec();
    let connects = connects.lock().unwrap().to_vec();
    let iframes = iframes.lock().unwrap().to_vec();

    let csp = Csp {
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

    let mut csps = Csp {
        javascripts: Vec::new(),
        fonts: Vec::new(),
        images: Vec::new(),
        styles: Vec::new(),
        connects: Vec::new(),
        iframes: Vec::new(),
    };

    for url in urls {
        match generate_csp(url) {
            Ok(csp) => csps.merge(csp),
            Err(e) => println!("error {:?}", e),
        }
    }

    println!("{}", csps);
}
