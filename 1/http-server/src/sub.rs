    use std::{
        path::Path,
        fs::read_to_string,
        io::{Result, Read, Write},
        net::TcpStream,
    };
    use regex::Regex;

    pub fn tcp_handler(mut stream: TcpStream) -> Result<()> {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer)?;
    
        let re = Regex::new(r"^GET /([a-zA-Z\d\.\-/_]+) HTTP/1\.1").unwrap();
        println!("{}", &String::from_utf8(buffer.to_vec()).unwrap());
        let response = match re.captures(&String::from_utf8(buffer.to_vec()).unwrap()) {
            Some(caps) => {
                let path = Path::new(&caps[1]);
                println!("{}", path.as_os_str().to_str().unwrap());
                if path.is_file() {
                    let status_line = "HTTP/1.1 200 OK";
                    let html = read_to_string(path)?;
                    let http_header = format!(
                        "Content-Length: {}\r\nContent-Type: text/html;charset=UTF-8",
                        html.len()
                    );
                    format!("{}\r\n{}\r\n\r\n{}", status_line, http_header, html)
                } else {
                    println!("Not found");
                    "HTTP/1.1 404 Not Found\r\n\r\n".to_string()
                }
            },
            None => {
                println!("No matched");
                "HTTP/1.1 400 Bad Request\r\n\r\n".to_string()
            },
        };
        stream.write_all(response.as_bytes())?;
        stream.flush()
    }
