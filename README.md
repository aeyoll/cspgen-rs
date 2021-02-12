# CSP Generation

Whoever has had to write CSP (Content Security Policy) knows that it is no picnic. This CLI written in Rust allows you to generate CSP headers from a given url, or multiple urls.

Usage
---

```shell script
cspgen <URL1> <URL2>...
```

#### Args:

```
<URL1> <URL2>...    The urls to generate the CSP from
```

Development
---

```shell script
git clone git@github.com:aeyoll/cspgen-rs.git
cd cspgen-rs
cargo run -- <URL1> <URL2>
