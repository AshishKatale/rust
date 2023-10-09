use std::{collections::HashMap, fmt::Display};

pub enum URLScheme {
    HTTP,
    HTTPS,
}

pub struct IP(pub u8, pub u8, pub u8, pub u8);

impl Display for IP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}.{}", self.0, self.1, self.2, self.3)
    }
}

pub struct URL {
    scheme: URLScheme,
    host: IP,
    port: u16,
    params: HashMap<String, String>,
}

impl URL {
    pub fn new() -> URLBuilder {
        URLBuilder {
            scheme: None,
            host: None,
            port: None,
            params: None,
        }
    }
}

impl Display for URL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let scheme = match self.scheme {
            URLScheme::HTTP => "http",
            URLScheme::HTTPS => "https",
        };

        let mut params = String::from("?");
        for kv in &self.params {
            params += kv.0.as_str();
            params += "=";
            params += kv.1.as_str();
            params += "&";
        }
        params.remove(params.len() - 1);
        write!(f, "{}://{}:{}/{}", scheme, self.host, self.port, params)
    }
}

pub struct URLBuilder {
    scheme: Option<URLScheme>,
    host: Option<IP>,
    port: Option<u16>,
    params: Option<HashMap<String, String>>,
}

impl URLBuilder {
    pub fn scheme(mut self, scheme: URLScheme) -> Self {
        self.scheme = Some(scheme);
        self
    }

    pub fn host(mut self, host: IP) -> Self {
        self.host = Some(host);
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    pub fn params(mut self, params: HashMap<String, String>) -> Self {
        self.params = Some(params);
        self
    }

    pub fn with_defaults(mut self) -> Self {
        self.scheme = Some(URLScheme::HTTP);
        self.host = Some(IP(127, 0, 0, 1));
        self.port = Some(3000);
        self.params = Some(HashMap::new());
        self
    }

    pub fn build(self) -> URL {
        URL {
            scheme: self.scheme.unwrap(),
            host: self.host.unwrap(),
            port: self.port.unwrap(),
            params: self.params.unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{URLScheme, IP, URL};
    use std::collections::HashMap;

    #[test]
    fn create_fibonacci() {
        let url_default = URL::new().with_defaults().build();
        let url_change_port_default = URL::new().with_defaults().port(5000).build();
        let url = URL::new()
            .scheme(URLScheme::HTTPS)
            .host(IP(192, 168, 0, 1))
            .port(27017)
            .params(HashMap::from([
                ("key".to_owned(), "value".to_owned()),
                ("key2".to_owned(), "value2".to_owned()),
            ]))
            .build();

        assert_eq!(url_default.to_string(), "http://127.0.0.1:3000/");
        assert_eq!(url_change_port_default.to_string(), "http://127.0.0.1:5000/");
        assert!(
            url.to_string() == "https://192.168.0.1:27017/?key=value&key2=value2"
                || url.to_string() == "https://192.168.0.1:27017/?key2=value2&key=value"
        );

    }

    #[test]
    #[should_panic]
    fn uninitialized() {
        let _ = URL::new().port(5500).build();
    }
}
