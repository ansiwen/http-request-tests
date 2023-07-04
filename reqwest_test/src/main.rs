use std::collections::HashMap;

 fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("https://httpbin.org/ip")?
        .json::<HashMap<String, String>>()?;
    println!("{:#?}", resp);
    let client = reqwest::blocking::Client::new();
    let res = client.post("http://httpbin.org/post")
    .body("the exact body that is sent")
    .send()?;
    println!("Status: {}", res.status());
    let body = res.text()?;
    println!("Body:\n\n{}", body);
    Ok(())
}