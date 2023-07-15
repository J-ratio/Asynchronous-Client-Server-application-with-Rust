mod http_protocol {

    struct Header {
        name: String,
        value: String
    }

    impl Header {
        
        fn from_string(s:&str) -> Header {

            let (n,v) = s.split_once(":").unwrap();

            Header {
                name: n.to_string(),
                value: v.to_string()
            }
        }
    }

    struct Status {
        protocol_version: String,
        status_code: i32,
        explaination: String
    }

    impl Status {
        
        fn from_string(s:&str) -> Status {

            let mut iter = s.split_whitespace();

            Status {
                protocol_version: iter.next().unwrap().to_string(),
                status_code: iter.next().unwrap().parse().unwrap(),
                explaination: iter.next().unwrap().to_string()
            }
        }
    }

    struct Response {
        status: Status,
        headers: Vec<Header>,
        body: String
    }

    impl Response {
        
        fn from_string(s: &str) -> Response {

            let (status_string, rest) = s.split_once("\n").unwrap();
            let (mut headers_string,body_string) = rest.split_once("\n\n").unwrap();
            
            let mut headers: Vec<Header> = vec![];

            while {

                let (header,rest) = headers_string.split_once("\n").unwrap();
                headers.push(Header::from_string(header));
                headers_string = rest;

                rest != ""
            } {}

            Response { 
                status: Status::from_string(status_string),
                headers: headers,
                body: body_string.to_string() 
            }
        }
    }

    struct  Error{
        err: String
    }
}

// HTTP/1.1 200 OK
// Server: nginx/1.21.5
// Date: Wed, 01 Feb 2023 22:30:51 GMT
// Content-Type: text/html; charset=utf-8
// Content-Length: 9465
// Last-Modified: Tue, 31 Jan 2023 09:13:04 GMT
// Connection: close
// ETag: "63d8dba0-24f9"
// Accept-Ranges: bytes

// <!DOCTYPE html PUBLIC "-//W3C//DTD HTML 4.01 Transitional//EN"
//  "http://www.w3.org/TR/html4/loose.dtd">
// […a long web page content…]