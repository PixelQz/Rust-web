use std::collections::HashMap;
use std::io::{Result,Write};
#[derive(Debug,PartialEq,Clone)]
pub struct HttpReponse<'a>{
    version:&'a str,
    status_code:&'a str,
    status_text:&'a str,
    headers:Option<HashMap<&'a str,&'a str>>,
    body:Option<String>,
}
impl<'a>Default for HttpReponse<'a>  {
    fn default() -> Self {
        Self { 
            version: "HTTP/1.1".into(), 
            status_code: "200".into(), 
            status_text: "OK".into(), 
            headers: None, 
            body: None }
    }
}

impl<'a> From<HttpReponse<'a>> for String {
    fn from(res: HttpReponse) -> String {
        let res1=res.clone();
        format!(
            "{}{}{}\r\n{}Content-Lenth:{}\r\n\r\n{}",
            &res1.version(),
            &res1.status_code(),
            &res1.status_text(),
            &res1.headers(),
            &res.body.unwrap().len(),
            &res1.body()
        )
    }
}

impl<'a> HttpReponse<'a> {
    pub fn new(
        status_code:&'a str,
        headers:Option<HashMap<&'a str,&'a str>>,
        body:Option<String>,
    )->HttpReponse<'a> {
        let mut response=HttpReponse::default();
        if status_code != "200"{
            response.status_code=status_code.into();
        };
        response.headers=match &headers {
            Some(_h)=>headers,
            None=>{
                let mut h =HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            }
        };
        response.status_text=match response.status_code {
            "200"=>"OK".into(),
            "400"=>"Bad Request".into(),
            "404"=>"Not Found".into(),
            "500"=>"Internel Server Error".into(),
            _=>"Not Found".into(),
        };
        response.body=body;
        response
    }
    pub fn send_response(&self,write_stream:&mut impl Write)->Result<()> {
        let res=self.clone();
        let response_string=String::from(res);
        let _ =write!(write_stream,"{}",response_string);
        Ok(())
    }
    fn version(&self)->&str {
        self.version
    }
    fn status_code(&self)->&str {
        self.status_code
    }
    fn status_text(&self)->&str {
        self.status_text
    }
    fn headers(&self)->String {
        let map = self.headers.clone().unwrap();
        let mut header_string="".into();
        for (k,v) in map.iter() {
            header_string=format!("{}{}:{}\r\n",header_string,k,v);
        }
        header_string
    }
    pub fn body(&self)->&str {
        match &self.body {
            Some(b)=>b.as_str(),
            None=>"",
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_reponse_200() {
        let reponse_actual=HttpReponse::new(
            "200", 
            None, 
            Some("xxxx".into())
        );
        let reponse_expected=HttpReponse{
            version:"HTTP/1.1",
            status_code:"200",
            status_text:"OK",
            headers:{
                let mut h=HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body:Some("xxxx".into())
        };
        assert_eq!(reponse_actual,reponse_expected);
    }

    #[test]
    fn test_reponse_400() {
        let reponse_actual=HttpReponse::new(
            "400", 
            None, 
            Some("xxxx".into())
        );
        let reponse_expected=HttpReponse{
            version:"HTTP/1.1",
            status_code:"200",
            status_text:"Not Found",
            headers:{
                let mut h=HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body:Some("xxxx".into())
        };
        assert_eq!(reponse_actual,reponse_expected);
    }
}