use merge::Merge;
use std::fmt;

#[derive(Merge)]
pub struct CSP {
    #[merge(strategy = merge::vec::append)]
    pub javascripts: Vec<String>,
    #[merge(strategy = merge::vec::append)]
    pub fonts: Vec<String>,
    #[merge(strategy = merge::vec::append)]
    pub images: Vec<String>,
    #[merge(strategy = merge::vec::append)]
    pub styles: Vec<String>,
    #[merge(strategy = merge::vec::append)]
    pub connects: Vec<String>,
    #[merge(strategy = merge::vec::append)]
    pub iframes: Vec<String>,
}

fn dedup(mut vec: Vec<String>) -> Vec<String> {
    vec.sort_unstable();
    vec.dedup();

    vec
}

impl fmt::Display for CSP {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Content-Security-Policy: \
            base-uri 'self';\
            default-src 'self';\
            connect-src 'self' {};\
            font-src 'self' data: {};\
            frame-src 'self' {};\
            img-src 'self' data: {};\
            script-src 'self' 'unsafe-inline' 'unsafe-eval' blob: {};\
            style-src 'self' data: 'unsafe-inline' {};",
            dedup(self.connects.to_owned()).join(" "),
            dedup(self.fonts.to_owned()).join(" "),
            dedup(self.iframes.to_owned()).join(" "),
            dedup(self.images.to_owned()).join(" "),
            dedup(self.javascripts.to_owned()).join(" "),
            dedup(self.styles.to_owned()).join(" "),
        )
    }
}
