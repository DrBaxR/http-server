pub mod http {
    use std::{
        fmt::Debug,
        io::{BufRead, BufReader, Read},
        net::TcpStream,
    };

    pub enum RequestData {
        WithoutBody(String, Vec<String>),
        WithBody(String, Vec<String>, Vec<u8>),
    }

    impl Debug for RequestData {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::WithoutBody(arg0, arg1) => f
                    .debug_tuple("WithoutBody")
                    .field(arg0)
                    .field(arg1)
                    .finish(),
                Self::WithBody(arg0, arg1, arg2) => f
                    .debug_tuple("WithBody")
                    .field(arg0)
                    .field(arg1)
                    .field(arg2)
                    .finish(),
            }
        }
    }

    pub fn read_req(stream: TcpStream) -> RequestData {
        let mut reader = BufReader::new(&stream);

        let request_line = read_request_line(&mut reader);
        let header_lines = read_header_lines(&mut reader);
        let body = read_body_bytes(&mut reader, &header_lines);

        if let Some(body_bytes) = body {
            RequestData::WithBody(request_line, header_lines, body_bytes)
        } else {
            RequestData::WithoutBody(request_line, header_lines)
        }
    }

    fn read_request_line(reader: &mut BufReader<&TcpStream>) -> String {
        let mut request_line = String::new();
        reader.read_line(&mut request_line).unwrap();

        request_line.trim().to_string()
    }

    fn read_header_lines(reader: &mut BufReader<&TcpStream>) -> Vec<String> {
        reader
            .lines()
            .map(|res| res.unwrap())
            .take_while(|line| !line.is_empty())
            .collect()
    }

    fn read_body_bytes(
        reader: &mut BufReader<&TcpStream>,
        header_lines: &Vec<String>,
    ) -> Option<Vec<u8>> {
        let content_length_line = header_lines
            .iter()
            .find(|line| line.trim().to_lowercase().contains("content-length:"));

        if let Some(length_line) = content_length_line {
            let length_vec: Vec<_> = length_line
                .split(":")
                .skip(1)
                .map(|val| val.trim())
                .collect();
            let length: usize = length_vec[0]
                .parse()
                .expect("Request 'content-length' header not formatted correctly");

            Some(reader.bytes().take(length).map(|b| b.unwrap()).collect())
        } else {
            None
        }
    }
}
