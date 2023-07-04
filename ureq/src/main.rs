fn main() -> Result<(), ureq::Error> {
    let body: String = ureq::get("http://example.com")
        .set("Example-Header", "header value")
        .call()?
        .into_string()?;

    println!("{}", body);
    // Requires the `json` feature enabled.
    let resp: String = ureq::post("http://myapi.example.com/ingest")
        .send_json(ureq::json!({
            "name": "martin",
            "rust": true
        }))?
        .into_string()?;
    println!("{}", resp);

    Ok(())
}
