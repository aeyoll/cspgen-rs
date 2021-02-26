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

impl fmt::Display for CSP {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Content-Security-Policy: \
            default-src 'self';\
            script-src 'self' 'unsafe-inline' 'unsafe-eval' {};\
            font-src 'self' data: {};\
            img-src 'self' data: {};\
            style-src 'self' data: 'unsafe-inline' {};\
            connect-src 'self' {};\
            frame-src 'self' {};",
            self.javascripts.join(" "),
            self.fonts.join(" "),
            self.images.join(" "),
            self.styles.join(" "),
            self.connects.join(" "),
            self.iframes.join(" ")
        )
    }
}
