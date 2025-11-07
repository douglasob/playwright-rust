// Integration tests for BrowserType::launch()
//
// These tests verify that we can launch real browsers using the Playwright server.

use playwright_core::api::LaunchOptions;
use playwright_core::protocol::Playwright;

#[tokio::test]
async fn test_launch_chromium() {
    // Launch Playwright
    let playwright = Playwright::launch()
        .await
        .expect("Failed to launch Playwright");

    // Get chromium browser type
    let chromium = playwright.chromium();

    // Launch browser with default options
    let browser = chromium.launch().await.expect("Failed to launch Chromium");

    // Verify browser was created
    assert_eq!(browser.name(), "chromium");
    assert!(!browser.version().is_empty());

    println!("Launched Chromium version: {}", browser.version());

    // Cleanup
    browser.close().await.expect("Failed to close browser");
}

#[tokio::test]
async fn test_launch_with_headless_option() {
    let playwright = Playwright::launch()
        .await
        .expect("Failed to launch Playwright");

    let chromium = playwright.chromium();

    // Launch with explicit headless option
    let options = LaunchOptions::default().headless(true);

    let browser = chromium
        .launch_with_options(options)
        .await
        .expect("Failed to launch Chromium with options");

    assert_eq!(browser.name(), "chromium");
    assert!(!browser.version().is_empty());

    // Cleanup
    browser.close().await.expect("Failed to close browser");
}

#[tokio::test]
async fn test_launch_all_three_browsers() {
    let playwright = Playwright::launch()
        .await
        .expect("Failed to launch Playwright");

    // Test Chromium
    let chromium = playwright.chromium();
    let chromium_browser = chromium.launch().await.expect("Failed to launch Chromium");
    assert_eq!(chromium_browser.name(), "chromium");
    println!("✓ Chromium: {}", chromium_browser.version());
    chromium_browser
        .close()
        .await
        .expect("Failed to close Chromium");

    // Test Firefox
    let firefox = playwright.firefox();
    let firefox_browser = firefox.launch().await.expect("Failed to launch Firefox");
    assert_eq!(firefox_browser.name(), "firefox");
    println!("✓ Firefox: {}", firefox_browser.version());
    firefox_browser
        .close()
        .await
        .expect("Failed to close Firefox");

    // Test WebKit
    let webkit = playwright.webkit();
    let webkit_browser = webkit.launch().await.expect("Failed to launch WebKit");
    assert_eq!(webkit_browser.name(), "webkit");
    println!("✓ WebKit: {}", webkit_browser.version());
    webkit_browser
        .close()
        .await
        .expect("Failed to close WebKit");
}

#[tokio::test]
async fn test_browser_close() {
    let playwright = Playwright::launch()
        .await
        .expect("Failed to launch Playwright");

    let chromium = playwright.chromium();
    let browser = chromium.launch().await.expect("Failed to launch Chromium");

    // Verify browser is open
    assert_eq!(browser.name(), "chromium");

    // Close browser
    browser.close().await.expect("Failed to close browser");

    println!("✓ Browser closed successfully");
}

#[tokio::test]
async fn test_close_multiple_browsers() {
    let playwright = Playwright::launch()
        .await
        .expect("Failed to launch Playwright");

    // Launch multiple browsers
    let chromium = playwright.chromium();
    let browser1 = chromium
        .launch()
        .await
        .expect("Failed to launch Chromium 1");
    let browser2 = chromium
        .launch()
        .await
        .expect("Failed to launch Chromium 2");

    println!("Launched 2 browsers");

    // Close both browsers
    browser1.close().await.expect("Failed to close browser 1");
    println!("✓ Browser 1 closed");

    browser2.close().await.expect("Failed to close browser 2");
    println!("✓ Browser 2 closed");
}
