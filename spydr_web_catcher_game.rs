use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, Document, Window};

#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    // Get window, document, and canvas context
    let window = web_sys::window().expect("Failed to get window");
    let document = window.document().expect("Failed to get document");
    let canvas = document
        .get_element_by_id("gameCanvas")
        .expect("Failed to get canvas")
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    let ctx = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    // Adjusted to fit the screen width and height
    let screen_width = window.inner_width()?.as_f64().unwrap();
    let screen_height = window.inner_height()?.as_f64().unwrap();
    canvas.set_width(screen_width as u32);
    canvas.set_height(screen_height as u32);

    let spider_size = 50.0; // Adjusted for better visibility on mobile screens
    let ant_size = 30.0; // Adjusted for better visibility on mobile screens
    let web_symbol = "🕸️";

    let spider_symbol = "🕷️";
    let mut spider_pos = (screen_width / 2.0, screen_height - 75.0); // Adjusted for better placement on mobile screens
    let mut ants = Vec::new();
    for _ in 0..12 {
        let ant = (
            (js_sys::Math::random() * (screen_width - ant_size)),
            (js_sys::Math::random() * (screen_height - ant_size)),
            false,
        );
        ants.push(ant);
    }

    let mut score = 0;

    // Drawing functions
    let draw_spider = || {
        ctx.set_fill_style(&JsValue::from_str("white"));
        ctx.set_font(&format!("{}px serif", spider_size));
        ctx.fill_text_with_max_width(
            spider_symbol,
            spider_pos.0,
            spider_pos.1,
            f64::INFINITY,
        )
        .expect("Failed to draw spider");
    };

    let draw_ants = || {
        ctx.set_fill_style(&JsValue::from_str("white"));
        ctx.set_font(&format!("{}px serif", ant_size));
        for ant in ants.iter() {
            if !ant.2 {
                ctx.fill_text_with_max_width(
                    ant_symbol,
                    ant.0,
                    ant.1,
                    f64::INFINITY,
                )
                .expect("Failed to draw ant");
            }
        }
    };

    let draw_score = || {
        ctx.set_fill_style(&JsValue::from_str("green"));
        ctx.set_font("40px Impact");
        ctx.fill_text_with_max_width(
            &format!("Score: {}", score),
            10.0,
            30.0,
            f64::INFINITY,
        )
        .expect("Failed to draw score");
    };

    let clear_screen = || {
        ctx.set_fill_style(&JsValue::from_str("#1a1a1a"));
        ctx.fill_rect(0.0, 0.0, screen_width, screen_height);
    };

    let check_collision = |x: f64, y: f64| {
        for ant in ants.iter_mut() {
            if !ant.2
                && x >= ant.0
                && x <= ant.0 + ant_size
                && y >= ant.1
                && y <= ant.1 + ant_size
            {
                ant.2 = true;
                score += 1;
                if score == 10 {
                    document
                        .set_title("🕷️ WINNER! 🕷️")
                        .expect("Failed to set title");
                    window.alert_with_message("🥳🕷️ YOU WIN 1 SPYDR!! 🕷️🥳")?;
                    // Add code to handle winning SPYDR token

                    // Create an image element
                    let image = document.create_element("img")?;
                    // Set the source of the image
                    image.set_attribute("src", "file:///storage/emulated/0/Android/data/com.teejay.trebedit/files/app2/SPYDRgame/Picsart_23-11-26_22-43-51-831.png")?;
                    // Set any additional attributes or styles for the image
                    image.set_attribute("style", "width: 5px; height: 5px;")?;
                    // Append the image to the body of the document
                    document.body().unwrap().append_child(&image)?;
                }
                break;
            }
        }
    };

    // Event listener for mouse click
    let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        let x = event.client_x() as f64;
        let y = event.client_y() as f64;
        check_collision(x, y);
    }) as Box<dyn FnMut(_)>);
    canvas
        .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
    closure.forget();

    // Main game loop
    fn game_loop(
        ctx: &CanvasRenderingContext2d,
        clear_screen: &dyn Fn(),
        draw_spider: &dyn Fn(),
        draw_ants: &dyn Fn(),
        draw_score: &dyn Fn(),
        ants: &mut Vec<(f64, f64, bool)>,
        screen_width: f64,
        screen_height: f64,
        window: &Window,
        document: &Document,
    ) -> Result<(), JsValue> {
        clear_screen();
        draw_spider();
        draw_ants();
        draw_score();
        move_ants(ants, screen_width, screen_height);
        window.request_animation_frame(move |_| {
            game_loop(ctx, clear_screen, draw_spider, draw_ants, draw_score, ants, screen_width, screen_height, window, document)
        })?;
        Ok(())
    }

    // Function to move ants randomly
    fn move_ants(ants: &mut Vec<(f64, f64, bool)>, screen_width: f64, screen_height: f64) {
        for ant in ants.iter_mut() {
            if !ant.2 {
                ant.0 += js_sys::Math::random() * 3.0 - 1.5;
                ant.1 += js_sys::Math::random() * 3.0 - 1.5;
            }
        }
    }

    // Call game loop
    game_loop(&ctx, &clear_screen, &draw_spider, &draw_ants, &draw_score, &mut ants, screen_width, screen_height, &window, &document)?;
    Ok(())
}
