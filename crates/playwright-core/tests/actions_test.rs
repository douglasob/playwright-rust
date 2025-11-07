// Integration tests for Locator actions
//
// Following TDD: Write tests first (Red), then implement (Green)
//
// Tests cover:
// - Click actions (single, double)
// - Fill actions (input, textarea)
// - Clear actions
// - Press actions (keyboard)

mod test_server;

use playwright_core::protocol::Playwright;
use test_server::TestServer;

#[tokio::test]
async fn test_click_button() {
    let server = TestServer::start().await;
    let playwright = Playwright::launch()
        .await
        .expect("Failed to launch Playwright");
    let browser = playwright
        .chromium()
        .launch()
        .await
        .expect("Failed to launch browser");
    let page = browser.new_page().await.expect("Failed to create page");

    page.goto(&format!("{}/button.html", server.url()), None)
        .await
        .expect("Failed to navigate");

    // Test: Click button changes its text
    let button = page.locator("#btn").await;
    button.click(None).await.expect("Failed to click button");

    let text = button.text_content().await.expect("Failed to get text");
    assert_eq!(text, Some("clicked".to_string()));

    browser.close().await.expect("Failed to close browser");
    server.shutdown();
}

#[tokio::test]
async fn test_dblclick() {
    let server = TestServer::start().await;
    let playwright = Playwright::launch()
        .await
        .expect("Failed to launch Playwright");
    let browser = playwright
        .chromium()
        .launch()
        .await
        .expect("Failed to launch browser");
    let page = browser.new_page().await.expect("Failed to create page");

    page.goto(&format!("{}/dblclick.html", server.url()), None)
        .await
        .expect("Failed to navigate");

    // Test: Double-click changes div text
    let div = page.locator("#target").await;
    div.dblclick(None).await.expect("Failed to double-click");

    let text = div.text_content().await.expect("Failed to get text");
    assert_eq!(text, Some("double clicked".to_string()));

    browser.close().await.expect("Failed to close browser");
    server.shutdown();
}

#[tokio::test]
async fn test_fill_input() {
    let server = TestServer::start().await;
    let playwright = Playwright::launch()
        .await
        .expect("Failed to launch Playwright");
    let browser = playwright
        .chromium()
        .launch()
        .await
        .expect("Failed to launch browser");
    let page = browser.new_page().await.expect("Failed to create page");

    page.goto(&format!("{}/form.html", server.url()), None)
        .await
        .expect("Failed to navigate");

    // Test: Fill input field
    let input = page.locator("#name").await;
    input
        .fill("John Doe", None)
        .await
        .expect("Failed to fill input");

    // Verify the input value
    let value = input
        .input_value(None)
        .await
        .expect("Failed to get input value");
    assert_eq!(value, "John Doe");

    browser.close().await.expect("Failed to close browser");
    server.shutdown();
}

#[tokio::test]
async fn test_fill_textarea() {
    let server = TestServer::start().await;
    let playwright = Playwright::launch()
        .await
        .expect("Failed to launch Playwright");
    let browser = playwright
        .chromium()
        .launch()
        .await
        .expect("Failed to launch browser");
    let page = browser.new_page().await.expect("Failed to create page");

    page.goto(&format!("{}/form.html", server.url()), None)
        .await
        .expect("Failed to navigate");

    // Test: Fill textarea
    let textarea = page.locator("#bio").await;
    textarea
        .fill("Hello\nWorld", None)
        .await
        .expect("Failed to fill textarea");

    // Verify the textarea value
    let value = textarea
        .input_value(None)
        .await
        .expect("Failed to get textarea value");
    assert_eq!(value, "Hello\nWorld");

    browser.close().await.expect("Failed to close browser");
    server.shutdown();
}

#[tokio::test]
async fn test_clear_input() {
    let server = TestServer::start().await;
    let playwright = Playwright::launch()
        .await
        .expect("Failed to launch Playwright");
    let browser = playwright
        .chromium()
        .launch()
        .await
        .expect("Failed to launch browser");
    let page = browser.new_page().await.expect("Failed to create page");

    page.goto(&format!("{}/input.html", server.url()), None)
        .await
        .expect("Failed to navigate");

    // Test: Clear input field with initial value
    let input = page.locator("#input").await;

    // Verify initial value
    let initial_value = input
        .input_value(None)
        .await
        .expect("Failed to get initial value");
    assert_eq!(initial_value, "initial");

    // Clear the input
    input.clear(None).await.expect("Failed to clear input");

    // Verify input is now empty
    let cleared_value = input
        .input_value(None)
        .await
        .expect("Failed to get cleared value");
    assert_eq!(cleared_value, "");

    browser.close().await.expect("Failed to close browser");
    server.shutdown();
}

#[tokio::test]
async fn test_press_enter() {
    let server = TestServer::start().await;
    let playwright = Playwright::launch()
        .await
        .expect("Failed to launch Playwright");
    let browser = playwright
        .chromium()
        .launch()
        .await
        .expect("Failed to launch browser");
    let page = browser.new_page().await.expect("Failed to create page");

    page.goto(&format!("{}/keyboard.html", server.url()), None)
        .await
        .expect("Failed to navigate");

    // Test: Press Enter key changes input value via JavaScript
    let input = page.locator("#input").await;
    input.click(None).await.expect("Failed to focus input");
    input
        .press("Enter", None)
        .await
        .expect("Failed to press Enter");

    // Verify keypress had effect (keyboard.html sets value to "submitted" on Enter)
    let value = input
        .input_value(None)
        .await
        .expect("Failed to get input value");
    assert_eq!(value, "submitted");

    browser.close().await.expect("Failed to close browser");
    server.shutdown();
}

// Cross-browser tests

#[tokio::test]
async fn test_click_firefox() {
    let server = TestServer::start().await;
    let playwright = Playwright::launch()
        .await
        .expect("Failed to launch Playwright");
    let browser = playwright
        .firefox()
        .launch()
        .await
        .expect("Failed to launch Firefox");
    let page = browser.new_page().await.expect("Failed to create page");

    page.goto(&format!("{}/button.html", server.url()), None)
        .await
        .expect("Failed to navigate");

    let button = page.locator("#btn").await;
    button.click(None).await.expect("Failed to click button");

    let text = button.text_content().await.expect("Failed to get text");
    assert_eq!(text, Some("clicked".to_string()));

    browser.close().await.expect("Failed to close browser");
    server.shutdown();
}

#[tokio::test]
async fn test_fill_webkit() {
    let server = TestServer::start().await;
    let playwright = Playwright::launch()
        .await
        .expect("Failed to launch Playwright");
    let browser = playwright
        .webkit()
        .launch()
        .await
        .expect("Failed to launch WebKit");
    let page = browser.new_page().await.expect("Failed to create page");

    page.goto(&format!("{}/form.html", server.url()), None)
        .await
        .expect("Failed to navigate");

    let input = page.locator("#name").await;
    input
        .fill("Test", None)
        .await
        .expect("Failed to fill input");

    // Verify the input value
    let value = input
        .input_value(None)
        .await
        .expect("Failed to get input value");
    assert_eq!(value, "Test");

    browser.close().await.expect("Failed to close browser");
    server.shutdown();
}
