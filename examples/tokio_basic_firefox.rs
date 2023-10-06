//! Requires chromedriver running on port 9515:
//!
//!     chromedriver --port=9515
//!
//! Run as follows:
//!
//!     cargo run --example tokio_async

use thirtyfour::prelude::*;
use thirtyfour::{DesiredCapabilities, WebDriver};

//pub use common::capabilities::firefox::FirefoxCapabilities;

fn main() -> color_eyre::Result<()> {
    let rt: tokio::runtime::Runtime = tokio::runtime::Builder::new_current_thread().enable_all().build()?;
    rt.block_on(run())
}

async fn run() -> color_eyre::Result<()> {
    // The use of color_eyre gives much nicer error reports, including making
    // it much easier to locate where the error occurred.
    color_eyre::install()?;

//https://docs.rs/thirtyfour/0.31.0/thirtyfour/common/capabilities/firefox/struct.FirefoxCapabilities.html
    let caps: thirtyfour::FirefoxCapabilities = DesiredCapabilities::firefox();
    



    //let driver: WebDriver = WebDriver::new("http://localhost:4444", caps).await?;
    let driver: WebDriver = WebDriver::new("http://0.0.0.0:4444", caps).await?;
    
    // Navigate to https://wikipedia.org.
    driver.goto("https://wikipedia.org").await?;
    
    let elem_form: WebElement = driver.find(By::Id("search-form")).await?;

    // Find element from element.
    let elem_text: WebElement = elem_form.find(By::Id("searchInput")).await?;

    // Type in the search terms.
    elem_text.send_keys("rust").await?;

    // Click the search button.
    let elem_button: WebElement = elem_form.find(By::Css("button[type='submit']")).await?;
    elem_button.click().await?;

    // Look for header to implicitly wait for the page to load.
    driver.find(By::ClassName("firstHeading")).await?;
    assert_eq!(driver.title().await?, "Selenium – Wikipedia");

    // Always explicitly close the browser. There are no async destructors.
    driver.quit().await?;

    //driver.close_window().await?;
    //driver.close_window().await?;
    Ok(())
}
