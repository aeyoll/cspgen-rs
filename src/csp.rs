use std::fmt;

pub struct CSP {
    pub javascripts: Vec<String>,
    pub fonts: Vec<String>,
    pub images: Vec<String>,
    pub styles: Vec<String>,
    pub connects: Vec<String>,
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
            connect-src 'self' {}; iframe-src 'self' {};",
            self.javascripts.join(" "),
            self.fonts.join(" "),
            self.images.join(" "),
            self.styles.join(" "),
            self.connects.join(" "),
            self.iframes.join(" ")
        )
    }
}
