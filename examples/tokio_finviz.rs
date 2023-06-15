//! Requires chromedriver running on port 9515:
//!
//!     chromedriver --port=9515
//!
//! Run as follows:
//!
//!     cargo run --example tokio_async

use thirtyfour::prelude::*;
use tokio::time::*;

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

    let mut caps: thirtyfour::ChromeCapabilities = DesiredCapabilities::chrome();
    caps.add_chrome_arg("--enable-automation")?;
    
    //let caps = DesiredCapabilities::firefox();
    let driver: WebDriver = WebDriver::new("http://localhost:9515", caps).await?;

    //driver.maximize_window().await?;

    // Navigate to https://finviz.com
    driver.goto("https://finviz.com").await?;

    //wait 5 second
    //from here
    //https://dev.to/stevepryde/using-selenium-with-rust-aca
    tokio::time::sleep(Duration::from_secs(5)).await;

    println!("Title = {}", driver.title().await?);

    // just a test ok
    // driver.fullscreen_window().await?;

    let elem_form: WebElement = driver.find(By::XPath("/html/body/div[1]/div/div/div/div[2]/div/button[3]")).await?;
    elem_form.click().await?;

    // click on screener
    // xpath
    // /html/body/table[2]/tbody/tr/td/table/tbody/tr/td[3]/a
    let elem_screener: WebElement = driver.find(By::XPath("/html/body/table[2]/tbody/tr/td/table/tbody/tr/td[3]/a")).await?;
    elem_screener.click().await?;

    //wait for screener
    tokio::time::sleep(Duration::from_secs(3)).await;

    // select screener all view
    let screener_all_view_xpath: &str  = "/html/body/div[3]/table/tbody/tr[2]/td/table/tbody/tr[2]/td[8]";
    let elem_screener_all: WebElement = driver.find(By::XPath(screener_all_view_xpath)).await?;
    elem_screener_all.click().await?;

    //wait for screener
    tokio::time::sleep(Duration::from_secs(3)).await;

    //select exchange
    let exchange_nyse_xpath: &str="/html/body/div[3]/table/tbody/tr[3]/td/div/form/table/tbody/tr[1]/td[2]/select/option[4]";
    let elem_exchange_nyse: WebElement = driver.find(By::XPath(exchange_nyse_xpath)).await?;
    elem_exchange_nyse.click().await?;

    //wait for debug not necessary
    tokio::time::sleep(Duration::from_secs(3)).await;

    //select Market Cap.
    let market_cap_xpath: &str = "/html/body/div[3]/table/tbody/tr[3]/td/div/form/table/tbody/tr[2]/td[2]/select/option[9]";
    let elem_market_cap: WebElement = driver.find(By::XPath(market_cap_xpath)).await?;
    elem_market_cap.click().await?;

    //wait for debug not necessary
    tokio::time::sleep(Duration::from_secs(10)).await;


    // Find element from element.
    //let elem_text: WebElement = elem_form.find(By::Id("searchInput")).await?;

    // Type in the search terms.
    //elem_text.send_keys("selenium").await?;

    // Click the search button.
    //let elem_button: WebElement = elem_form.find(By::Css("button[type='submit']")).await?;
    //elem_button.click().await?;

    // Look for header to implicitly wait for the page to load.
    //driver.find(By::ClassName("firstHeading")).await?;
    //assert_eq!(driver.title().await?, "Selenium – Wikipedia");

    // Always explicitly close the browser. There are no async destructors.
    
    driver.quit().await?;

    Ok(())
}
