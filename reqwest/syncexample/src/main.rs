use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if false {
        let resp =
            reqwest::blocking::get("https://httpbin.org/ip")?.json::<HashMap<String, String>>()?;
        println!("{:#?}", resp);
    }

    {
        env_logger::init();

        // Some simple CLI args requirements...
        let url = match std::env::args().nth(1) {
            Some(url) => url,
            None => {
                println!("No CLI URL provided, using default.");
                "https://httpbin.org/get".into()
            }
        };

        eprintln!("Fetching {:?}...", url);

        // reqwest::blocking::get() is a convenience function.
        //
        // In most cases, you should create/build a reqwest::Client and reuse
        // it for all requests.
        let mut res = reqwest::blocking::get(url)?;

        eprintln!("Response: {:?} {}", res.version(), res.status());
        eprintln!("Headers: {:#?}\n", res.headers());

        // copy the response body directly to stdout
        res.copy_to(&mut std::io::stdout())?;
    }

    Ok(())
}
