use std::error::Error;
use std::fs;

use headless_chrome::Browser;
use tera::{Context, Tera};

use crate::models::Invoice;

pub fn generate_invoice_pdf(invoice: &Invoice) -> Result<(), Box<dyn Error>> {
    let browser = Browser::default()?;

    let tab = browser.new_tab()?;

    let html_path = generate_invoice_html(&invoice)?;

    // Navigate to the local HTML file
    let file_path = format!(
        "file://{}/{}",
        std::env::current_dir()?.display(),
        html_path
    );
    tab.navigate_to(&file_path)?;

    // Take pdf of the entire browser window
    let pdf = tab.wait_until_navigated()?.print_to_pdf(None)?;
    // Save the pdf to disc
    std::fs::write(format!("invoice_{}.pdf", invoice.code), pdf)?;

    fs::remove_file(&html_path)?;

    Ok(())
}

fn render_invoice(invoice: &Invoice) -> tera::Result<String> {
    let tera = Tera::new("templates/**/*")?;
    let mut ctx = Context::new();
    ctx.insert("invoice", invoice);
    tera.render("invoice.html", &ctx)
}

pub fn generate_invoice_html(invoice: &Invoice) -> Result<String, Box<dyn std::error::Error>> {
    let html = render_invoice(&invoice)?;

    let html_path = format!("invoice_{}.html", invoice.code);

    fs::write(&html_path, &html)?;

    Ok(html_path)
}
