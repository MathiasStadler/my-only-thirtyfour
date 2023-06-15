//! Requires chromedriver running on port 9515:
//!
//!     chromedriver --port=9515
//!
//! Run as follows:
//!
//!     cargo run --example tokio_async

use thirtyfour::prelude::*;

fn main() -> color_eyre::Result<()> {
    let rt: tokio::runtime::Runtime = tokio::runtime::Builder::new_current_thread().enable_all().build()?;
    rt.block_on(run())
}

///html/body/div[3]/div/table/tbody/tr/td/table/tbody/tr/td/table/tbody/tr/td[4]/div/div/div[1]/div/div[2]/div/div/canvas
////html/body/div[3]/div/table/tbody/tr/td/table/tbody/tr/td/table/tbody/tr/td[4]/div/div/div[1]/div/div[2]/div/div/canvas
/// /html/body/div[3]/div/table/tbody/tr/td/div[2]/table/tbody/tr[2]/td/table/tbody/tr/td[1]/table/tbody/tr[4]/td[2]
/// /html/body/div[3]/div/table/tbody/tr/td/div[2]/table/tbody/tr[2]/td/table/tbody/tr/td[1]/table/tbody/tr[4]/td[2]
/// /html/body/div[1]/div/div/div/div[2]/div/button[3]

async fn run() -> color_eyre::Result<()> {
    // The use of color_eyre gives much nicer error reports, including making
    // it much easier to locate where the error occurred.
    color_eyre::install()?;

    let caps: thirtyfour::ChromeCapabilities = DesiredCapabilities::chrome();

    
    //let caps = DesiredCapabilities::firefox();
    let driver: WebDriver = WebDriver::new("http://localhost:9515", caps).await?;

    driver.maximize_window().await?;

    // Navigate to https://wikipedia.org.
    driver.goto("https://finviz.com").await?;

   // driver.fullscreen_window().await?;

    let elem_form: WebElement = driver.find(By::Id("search-form")).await?;

    // Find element from element.
    let elem_text: WebElement = elem_form.find(By::Id("searchInput")).await?;

    // Type in the search terms.
    elem_text.send_keys("selenium").await?;

    // Click the search button.
    let elem_button: WebElement = elem_form.find(By::Css("button[type='submit']")).await?;
    elem_button.click().await?;

    // Look for header to implicitly wait for the page to load.
    driver.find(By::ClassName("firstHeading")).await?;
    assert_eq!(driver.title().await?, "Selenium – Wikipedia");

    // Always explicitly close the browser. There are no async destructors.
    driver.quit().await?;

    Ok(())
}
