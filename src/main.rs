use thirtyfour::prelude::*;
use thirtyfour::{DesiredCapabilities, WebDriver};


#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let caps: thirtyfour::FirefoxCapabilities = DesiredCapabilities::firefox();
    let driver: WebDriver = WebDriver::new("http://localhost:4444", caps).await.expect("failed to connect to WebDriver");

    // Navigate to https://wikipedia.org.
    driver.goto("https://wikipedia.org").await?;
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
    assert_eq!(driver.title().await?, "Selenium - Wikipedia");

    // Always explicitly close the browser.
    driver.quit().await?;

    Ok(())
}
