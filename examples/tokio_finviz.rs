//! Requires chromedriver running on port 9515:
//!
//!     chromedriver --port=9515
//!
//! Run as follows:
//!
//!     cargo run --example tokio_async

extern crate sxd_document;
extern crate sxd_xpath;

use std::fs;
use thirtyfour::prelude::*;
use tokio::time::*;

fn main() -> color_eyre::Result<()> {
    let rt: tokio::runtime::Runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    rt.block_on(run())
}

///html/body/div[3]/div/table/tbody/tr/td/table/tbody/tr/td/table/tbody/tr/td[4]/div/div/div[1]/div/div[2]/div/div/canvas
////html/body/div[3]/div/table/tbody/tr/td/table/tbody/tr/td/table/tbody/tr/td[4]/div/div/div[1]/div/div[2]/div/div/canvas
/// /html/body/div[3]/div/table/tbody/tr/td/div[2]/table/tbody/tr[2]/td/table/tbody/tr/td[1]/table/tbody/tr[4]/td[2]
/// /html/body/div[3]/div/table/tbody/tr/td/div[2]/table/tbody/tr[2]/td/table/tbody/tr/td[1]/table/tbody/tr[4]/td[2]
/// /html/body/div[1]/div/div/div/div[2]/div/button[3]

/* function `set_filter_items` is never used
async fn set_filter_items(driver: &WebDriver,xpath_path : &str)  -> color_eyre::Result<()> {
    //color_eyre::install()?;

    println!("str {}",xpath_path);
    //let elem_exchange_nyse: WebElement = driver.find(By::XPath(exchange_nyse_xpath)).await?;
    //elem_exchange_nyse.click().await?;
    driver.find(By::XPath(xpath_path)).await?.click().await?;

    Ok(())

}
*/

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

    // site title
    println!("Title = {}", driver.title().await?);

    // just a test ok
    // driver.fullscreen_window().await?;

    let elem_form: WebElement = driver
        .find(By::XPath(
            "/html/body/div[1]/div/div/div/div[2]/div/button[3]",
        ))
        .await?;
    elem_form.click().await?;

    // click on screener
    // xpath
    // /html/body/table[2]/tbody/tr/td/table/tbody/tr/td[3]/a
    let elem_screener: WebElement = driver
        .find(By::XPath(
            "/html/body/table[2]/tbody/tr/td/table/tbody/tr/td[3]/a",
        ))
        .await?;
    elem_screener.click().await?;

    //wait for screener
    tokio::time::sleep(Duration::from_secs(10)).await;

    // set_filter_items(&driver,"/html/body/div[3]/table/tbody/tr[2]/td/table/tbody/tr[2]/td[8]");

    //process::exit(1);

    // select screener all view
    let screener_all_view_xpath: &str =
        "/html/body/div[3]/table/tbody/tr[2]/td/table/tbody/tr[2]/td[8]";
    let elem_screener_all: WebElement = driver.find(By::XPath(screener_all_view_xpath)).await?;
    elem_screener_all.click().await?;

    //wait for screener
    tokio::time::sleep(Duration::from_secs(3)).await;

    driver.maximize_window().await?;

    //wait for screener
    tokio::time::sleep(Duration::from_secs(3)).await;

    //select exchange
    let exchange_nyse_xpath: &str =
        "/html/body/div[3]/table/tbody/tr[3]/td/div/form/table/tbody/tr[1]/td[2]/select/option[4]";
    let elem_exchange_nyse: WebElement = driver.find(By::XPath(exchange_nyse_xpath)).await?;
    elem_exchange_nyse.click().await?;

    //wait for refresh
    tokio::time::sleep(Duration::from_secs(3)).await;

    //select Market Cap.
    let market_cap_xpath: &str =
        "/html/body/div[3]/table/tbody/tr[3]/td/div/form/table/tbody/tr[2]/td[2]/select/option[9]";
    let elem_market_cap: WebElement = driver.find(By::XPath(market_cap_xpath)).await?;
    elem_market_cap.click().await?;

    //wait for refresh
    tokio::time::sleep(Duration::from_secs(3)).await;

    //select Option/Short => Optionable
    let market_cap_xpath: &str =
        "/html/body/div[3]/table/tbody/tr[3]/td/div/form/table/tbody/tr[8]/td[8]/select/option[2]";
    let elem_market_cap: WebElement = driver.find(By::XPath(market_cap_xpath)).await?;
    elem_market_cap.click().await?;

    //wait for refresh
    tokio::time::sleep(Duration::from_secs(3)).await;

    //200-Day Simple Moving Average
    let over_200_xpath: &str = "/html/body/div[3]/table/tbody/tr[3]/td/div/form/table/tbody/tr[10]/td[6]/select/option[12]";
    let elem_over_200: WebElement = driver.find(By::XPath(over_200_xpath)).await?;
    elem_over_200.click().await?;

    //wait for refresh
    tokio::time::sleep(Duration::from_secs(3)).await;

    //50-Day Simple Moving Average
    let over_200_xpath: &str =
        "/html/body/div[3]/table/tbody/tr[3]/td/div/form/table/tbody/tr[10]/td[4]/select/option[9]";
    let elem_over_200: WebElement = driver.find(By::XPath(over_200_xpath)).await?;
    elem_over_200.click().await?;

    //wait for refresh
    tokio::time::sleep(Duration::from_secs(3)).await;

    // Price
    let price_xpath: &str = "/html/body/div[3]/table/tbody/tr[3]/td/div/form/table/tbody/tr[13]/td[2]/select/option[40]";
    let elem_price: WebElement = driver.find(By::XPath(price_xpath)).await?;
    elem_price.click().await?;

    //wait for refresh
    tokio::time::sleep(Duration::from_secs(3)).await;

    //Pattern channel up
    let pattern_xpath: &str = "/html/body/div[3]/table/tbody/tr[3]/td/div/form/table/tbody/tr[11]/td[8]/select/option[18]";
    let elem_pattern: WebElement = driver.find(By::XPath(pattern_xpath)).await?;
    elem_pattern.click().await?;

    //wait for refresh
    tokio::time::sleep(Duration::from_secs(3)).await;

    //PEG over one
    let peg_over_one_xpath: &str =
        "/html/body/div[3]/table/tbody/tr[3]/td/div/form/table/tbody/tr[2]/td[8]/select/option[7]";
    let elem_peg_over_one: WebElement = driver.find(By::XPath(peg_over_one_xpath)).await?;
    elem_peg_over_one.click().await?;

    //wait for refresh
    tokio::time::sleep(Duration::from_secs(3)).await;

    //EPS year
    let eps_year_xpath: &str =
        "/html/body/div[3]/table/tbody/tr[3]/td/div/form/table/tbody/tr[2]/td[8]/select/option[7]";
    let elem_eps_year: WebElement = driver.find(By::XPath(eps_year_xpath)).await?;
    elem_eps_year.click().await?;

    //wait for refresh
    tokio::time::sleep(Duration::from_secs(3)).await;

    // for what
    //println!("Driver status => {}",driver.status());

    //PEG
    //EPS year
    //EPS qtr

    //wait for debug not necessary
    tokio::time::sleep(Duration::from_secs(20)).await;

    // get result
    //old
    // let result_xpath: &str="/html/body/div[3]/table/tbody/tr[4]/td/div/table/tbody/tr[5]/td";
    let result_xpath: &str =
        "/html/body/div[3]/table/tbody/tr[4]/td/div/table/tbody/tr[4]/td/table/tbody/tr[2]/td[2]/a";

    let elem_result: Result<WebElement, std::num::ParseIntError> = driver.find(By::XPath(result_xpath)).await?;

    println!("Ticker {}", elem_result);

    //from here
    //https://docs.rs/thirtyfour/latest/thirtyfour/struct.WebElement.html
    let html: String = elem_result.inner_html().await?;
    println!("Inner HTML =>{}", html);

    //write to file
    fs::write("/home/trapapa/selenium_output.txt", html).expect("Unable to write file");

    /*
    let package = parser::parse("<root>hello</root>").expect("failed to parse XML");
    let document = package.as_document();

    let value = evaluate_xpath(&document, "/root").expect("XPath evaluation failed");

    println!("Result XPATH {}", value.string());
    */

    Ok(())
}
