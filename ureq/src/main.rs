use std::collections::HashMap;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let agent = ureq::AgentBuilder::new()
        .tls_connector(Arc::new(native_tls::TlsConnector::new()?))
        .build();
    let resp = agent
        .get("https://httpbin.org/ip")
        .set("Example-Header", "header value")
        .call()?
        .into_json::<HashMap<String, String>>()?;
    println!("{:#?}", resp);
    // Requires the `json` feature enabled.
    let resp = agent
        .post("https://httpbin.org/post")
        .send_json(ureq::json!({
            "name": "martin",
            "rust": true
        }))?;
    println!("Status: {}", resp.status());
    let body = resp.into_string()?;
    println!("Body:\n\n{}", body);

    Ok(())
}
