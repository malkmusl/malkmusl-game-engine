use select::document::Document;
use select::node::Node;
use reqwest::blocking::Client;

fn parse() {
    // Make an HTTP request to the URL containing the HTML
    let url = "https://example.com"; // Replace with your actual URL
    let body = Client::new()
        .get(url)
        .send()
        .expect("Failed to send request")
        .text()
        .expect("Failed to get response body");

    // Parse the HTML document
    let document = Document::from(body.as_str());

    // Find all tables on the page
    for table in document.find(Name("table")) {
        // Find the <tbody> within each table
        if let Some(tbody) = table.find(Name("tbody")).next() {
            // Extract every <tr> element within the <tbody>
            for row in tbody.find(Name("tr")) {
                // Extract specific elements within each <tr> element
                if let Some(img_alt) = row.find(Name("img")).next().and_then(|img| img.attr("alt")) {
                    println!("Alt: {}", img_alt);
                }

                if let Some(span_text) = row.find(Name("span").and(Attr("class", "infocard-cell-data"))).next() {
                    if let Ok(number) = span_text.text().parse::<u32>() {
                        println!("Number: {}", number);
                    }
                }
            }
        }
    }
}
