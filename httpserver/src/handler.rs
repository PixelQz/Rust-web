use http::{httprequest::HttpRequest,httpreponse::HttpReponse};
use serde::{Deserialize,Serialize}; 
use std::collections::HashMap;
use std::env;
use std::fs;

pub trait Handler {
    fn handle(req:&HttpRequest)->HttpReponse;
    fn load_file(file_name:&str)->Option<String> {
        let default_path=format!("{}/public",env!("CARGO_MANIFEST_DIR"));
        let public_path=env::var("PUBLIC_PATH").unwrap_or(default_path);
        let full_path=format!("{}/{}",public_path,file_name);
        let contents=fs::read_to_string(full_path);
        contents.ok()
    }
}

pub struct StaticPageHandler;
pub struct PageNotFoundHandler;
pub struct WebServiceHandler;

#[derive(Deserialize,Serialize)]
pub struct OrderStatus{
    order_id:i32,
    order_data:String,
    order_status:String,
}

impl Handler for PageNotFoundHandler {
    fn handle(_req:&HttpRequest)->HttpReponse {
        HttpReponse::new("404",None,Self::load_file("404.html"))
    }
}

impl Handler for StaticPageHandler {
    fn handle(req:&HttpRequest)->HttpReponse {
        let http::httprequest::Resource::Path(s)=&req.resource;
        let route:Vec<&str>=s.split("/").collect();
        match route[1] {
            ""=>HttpReponse::new("200",None,Self::load_file("index.html")),
            "health"=>HttpReponse::new("200",None,Self::load_file("health.html")),
            path=>match Self::load_file(path) {
                Some(contents)=>{
                    let mut map=HashMap::new();
                    if path.ends_with(".css") {
                        map.insert("Content-Type", "text/css");
                    }else if path.ends_with(".js") {
                        map.insert("Content-Type", "text/javascript");
                    }else {
                        map.insert("Content-Type", "text/html");
                    }
                    HttpReponse::new("200", Some(map), Some(contents))
                }
                None=>HttpReponse::new("400", None, Self::load_file("404.html")),
            }
        }
    }
}
impl WebServiceHandler {
    fn load_json()->Vec<OrderStatus> {
        let default_path=format!("{}/data",env!("CARGO_MANIFEST_DIR"));
        let data_path=env::var("DATA_PATH").unwrap_or(default_path);
        let full_path=format!("{}/{}",data_path,"orders.json");
        let json_contents=fs::read_to_string(full_path);
        let orders=serde_json::from_str(json_contents.unwrap().as_str()).unwrap();
        orders
    }
}       

impl Handler for WebServiceHandler {
    fn handle(req:&HttpRequest)->HttpReponse {
        let http::httprequest::Resource::Path(s)=&req.resource;
        let route:Vec<&str>=s.split("/").collect();
        match route[2] {
            "shipping" if route.len() > 2 && route[3] == "orders"=>{
                let body=Some(serde_json::to_string(&Self::load_json()).unwrap());
                let mut headers=HashMap::new();
                headers.insert("Content-Type", "application/json");
                HttpReponse::new("200", Some(headers), body)
            }
            _=>HttpReponse::new("404", None, Self::load_file("404.html"))
        }
    }
}