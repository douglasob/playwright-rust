// Integration tests for Action Options (Phase 4, Slice 5)
//
// Following TDD: Write tests first (Red), then implement (Green)
//
// Tests cover:
// - Fill options (force, timeout)
// - Press options (delay, timeout)
// - Check options (force, position, timeout, trial)
// - Hover options (force, modifiers, position, timeout, trial)
// - Select options (force, timeout)
// - Keyboard options (delay)
// - Mouse options (button, click_count, delay, steps)
// - Cross-browser compatibility
//
// Note: Tests are combined where possible to reduce browser launches

mod test_server;

use playwright_core::protocol::action_options::{
    CheckOptions, FillOptions, HoverOptions, KeyboardOptions, MouseOptions, PressOptions,
    SelectOptions,
};
use playwright_core::protocol::{MouseButton, Playwright, Position};
use test_server::TestServer;

#[tokio::test]
async fn test_fill_with_options() {
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

    // Test fill with force option
    let input = page.locator("#input").await;
    let options = FillOptions::builder().force(true).build();
    input
        .fill("Hello World", Some(options))
        .await
        .expect("Failed to fill with force");

    let value = input.input_value(None).await.unwrap();
    assert_eq!(value, "Hello World");

    browser.close().await.expect("Failed to close browser");
    server.shutdown();
}

#[tokio::test]
async fn test_press_with_delay() {
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

    // Press with delay option
    let input = page.locator("#input").await;
    input.click(None).await.expect("Failed to click");

    let options = PressOptions::builder().delay(50.0).build();
    input
        .press("Enter", Some(options))
        .await
        .expect("Failed to press with delay");

    let value = input.input_value(None).await.unwrap();
    assert_eq!(value, "submitted");

    browser.close().await.expect("Failed to close browser");
    server.shutdown();
}

#[tokio::test]
async fn test_check_with_options() {
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

    page.goto(&format!("{}/checkbox.html", server.url()), None)
        .await
        .expect("Failed to navigate");

    // Test check with force option
    let checkbox = page.locator("#checkbox").await;
    let options = CheckOptions::builder().force(true).build();
    checkbox
        .check(Some(options))
        .await
        .expect("Failed to check with force");

    assert!(
        checkbox.is_checked().await.unwrap(),
        "Checkbox should be checked"
    );

    // Test uncheck with trial option (dry-run)
    let checked_checkbox = page.locator("#checked-checkbox").await;
    let trial_options = CheckOptions::builder().trial(true).build();
    checked_checkbox
        .uncheck(Some(trial_options))
        .await
        .expect("Failed to trial uncheck");

    // Trial should not actually uncheck
    assert!(
        checked_checkbox.is_checked().await.unwrap(),
        "Trial uncheck should not actually uncheck"
    );

    browser.close().await.expect("Failed to close browser");
    server.shutdown();
}

#[tokio::test]
async fn test_hover_with_options() {
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

    page.goto(&format!("{}/hover.html", server.url()), None)
        .await
        .expect("Failed to navigate");

    // Hover with position option
    let button = page.locator("#hover-button").await;
    let options = HoverOptions::builder()
        .position(Position { x: 5.0, y: 5.0 })
        .build();
    button
        .hover(Some(options))
        .await
        .expect("Failed to hover with position");

    // Tooltip should be visible after hover
    let tooltip = page.locator("#tooltip").await;
    assert!(
        tooltip.is_visible().await.unwrap(),
        "Tooltip should be visible after hover"
    );

    browser.close().await.expect("Failed to close browser");
    server.shutdown();
}

#[tokio::test]
async fn test_select_with_options() {
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

    page.goto(&format!("{}/select.html", server.url()), None)
        .await
        .expect("Failed to navigate");

    // Select with force option
    let select = page.locator("#single-select").await;
    let options = SelectOptions::builder().force(true).build();
    let selected = select
        .select_option("apple", Some(options))
        .await
        .expect("Failed to select with force");

    assert_eq!(selected, vec!["apple"]);

    browser.close().await.expect("Failed to close browser");
    server.shutdown();
}

#[tokio::test]
async fn test_keyboard_with_delay() {
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

    page.goto(&format!("{}/keyboard_mouse.html", server.url()), None)
        .await
        .expect("Failed to navigate");

    // Type with delay
    let input = page.locator("#keyboard-input").await;
    input.click(None).await.expect("Failed to click input");

    let keyboard = page.keyboard();
    let options = KeyboardOptions::builder().delay(10.0).build();
    keyboard
        .type_text("Hello", Some(options))
        .await
        .expect("Failed to type with delay");

    let value = input.input_value(None).await.unwrap();
    assert_eq!(value, "Hello");

    browser.close().await.expect("Failed to close browser");
    server.shutdown();
}

#[tokio::test]
async fn test_mouse_with_options() {
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

    page.goto(&format!("{}/keyboard_mouse.html", server.url()), None)
        .await
        .expect("Failed to navigate");

    // Click with mouse options
    let mouse = page.mouse();
    let options = MouseOptions::builder()
        .button(MouseButton::Left)
        .click_count(1)
        .build();
    mouse
        .click(150, 150, Some(options))
        .await
        .expect("Failed to click with options");

    // Verify click was registered
    let result = page
        .locator("#mouse-result")
        .await
        .inner_text()
        .await
        .unwrap();
    assert_eq!(result, "Clicked");

    browser.close().await.expect("Failed to close browser");
    server.shutdown();
}

#[tokio::test]
async fn test_action_options_firefox() {
    // Cross-browser test: Firefox
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

    page.goto(&format!("{}/input.html", server.url()), None)
        .await
        .expect("Failed to navigate");

    // Test fill with force option in Firefox
    let input = page.locator("#input").await;
    let options = FillOptions::builder().force(true).build();
    input
        .fill("Firefox Test", Some(options))
        .await
        .expect("Failed to fill in Firefox");

    let value = input.input_value(None).await.unwrap();
    assert_eq!(value, "Firefox Test");

    browser.close().await.expect("Failed to close browser");
    server.shutdown();
}

#[tokio::test]
async fn test_action_options_webkit() {
    // Cross-browser test: WebKit
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

    page.goto(&format!("{}/checkbox.html", server.url()), None)
        .await
        .expect("Failed to navigate");

    // Test check with options in WebKit
    let checkbox = page.locator("#checkbox").await;
    let options = CheckOptions::builder().force(true).build();
    checkbox
        .check(Some(options))
        .await
        .expect("Failed to check in WebKit");

    assert!(
        checkbox.is_checked().await.unwrap(),
        "Checkbox should be checked in WebKit"
    );

    browser.close().await.expect("Failed to close browser");
    server.shutdown();
}
