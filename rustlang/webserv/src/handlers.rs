pub mod connection
{
    use std::io::prelude::*;
    use std::fs;
    use std::str;
    use std::net::TcpStream;

    pub fn handle_connection(mut stream: TcpStream)
    {
        let mut buffer = [0; 512];

        stream.read(&mut buffer).unwrap();

        let s = match str::from_utf8(&buffer)
        {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence {}", e),
        };
        println!("{}", s);

        if buffer.starts_with(b"GET / HTTP/1.1\r\n") 
        {
            let contents = fs::read_to_string("hello.html").unwrap();
            let response = format!("HTTP/1.1 200 OK\r\n\r\n{}{}", contents, s);
            stream.write(response.as_bytes()).unwrap();
        }
        else
        {
            let contents = fs::read_to_string("404.html").unwrap();
            let response = format!("HTTP/1.1 404 NOT FOUND\r\n\r\n{}{}", contents, s);
            stream.write(response.as_bytes()).unwrap();
        }

        stream.flush().unwrap();
    }
}
