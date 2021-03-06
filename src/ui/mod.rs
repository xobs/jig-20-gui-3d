//! This module is used for sharing a few items between the `all_widgets.rs`, `glutin_glium.rs` and
//! `glutin_gfx.rs` examples.
//!
//! The module contains:
//!
//! - `pub struct CFTIApp` as a demonstration of some state we want to change.
//! - `pub fn gui` as a demonstration of all widgets, some of which mutate our `CFTIApp`.
//! - `pub struct Ids` - a set of all `widget::Id`s used in the `gui` fn.
//!
//! By sharing these items between these examples, we can test and ensure that the different events
//! and drawing backends behave in the same manner.

use conrod;
use std;
use cfti::testset::TestSet;

pub const WIN_W: u32 = 600;
pub const WIN_H: u32 = 420;


/// A demonstration of some application state we want to control with a conrod GUI.
pub struct CFTIApp<'a> {
    test_set: &'a TestSet,
}


impl<'a> CFTIApp<'a> {

    /// Simple constructor for the `CFTIApp`.
    pub fn new(test_set: &'a TestSet) -> Self {
        CFTIApp {
            test_set: test_set,
        }
    }

}


/// A set of reasonable stylistic defaults that works for the `gui` below.
pub fn theme() -> conrod::Theme {
    conrod::Theme {
        name: "Demo Theme".to_string(),
        padding: conrod::Padding::none(),
        x_position: conrod::Position::Align(conrod::Align::Start, None),
        y_position: conrod::Position::Direction(conrod::Direction::Backwards, 20.0, None),
        background_color: conrod::color::DARK_CHARCOAL,
        shape_color: conrod::color::LIGHT_CHARCOAL,
        border_color: conrod::color::BLACK,
        border_width: 0.0,
        label_color: conrod::color::WHITE,
        font_id: None,
        font_size_large: 26,
        font_size_medium: 18,
        font_size_small: 12,
        widget_styling: std::collections::HashMap::new(),
        mouse_drag_threshold: 0.0,
        double_click_threshold: std::time::Duration::from_millis(500),
    }
}


/// Create an image map that maps the `ids.rust_logo` to the `rust_logo` image.
/*
pub fn image_map<T>(ids: &Ids, rust_logo: T) -> conrod::image::Map<T> {
    image_map! {
        (ids.rust_logo, rust_logo)
    }
}
*/

// Generate a unique `WidgetId` for each widget.
widget_ids! {
    pub struct Ids {

        // The scrollable canvas.
        canvas,
        canvas_flow,

        // The title and introduction widgets.
        tests_title,
        device_title,

        // Test list
        tests_canvas,
        tests_list,

        // Device list
        device_canvas,
        device_list,
/*
        // Shapes.
        shapes_canvas,
        rounded_rectangle,
        shapes_left_col,
        shapes_right_col,
        shapes_title,
        line,
        point_path,
        rectangle_fill,
        rectangle_outline,
        trapezoid,
        oval_fill,
        oval_outline,
        circle,

        // Image.
        image_title,
        rust_logo,

        // Button, XyPad, Toggle.
        button_title,
        button,
        xy_pad,
        toggle,
        ball,

        // NumberDialer, PlotPath
        dialer_title,
        number_dialer,
        plot_path,
*/
        // Scrollbar
        canvas_scrollbar,
    }
}


/// Instantiate a GUI demonstrating every widget available in conrod.
pub fn gui(ui: &mut conrod::UiCell, ids: &Ids, app: &mut CFTIApp) {
    use conrod::{widget, Colorable, Labelable, Positionable, Sizeable, Widget};

    const MARGIN: conrod::Scalar = 30.0;

    //widget::Canvas::new().pad(MARGIN).scroll_kids_vertically().set(ids.canvas, ui);
    widget::Canvas::new().pad(MARGIN).set(ids.canvas, ui);
    //widget::Canvas::new().color(conrod::color::DARK_CHARCOAL).set(ids.canvas, ui);

    /// A canvas to hold the left widget, where tests go
    widget::Canvas::new()
        .w(200.0)
        .h_of(ids.canvas)
        .align_left()
        .color(conrod::color::PURPLE)
        .set(ids.tests_canvas, ui);

    /// A canvas to hold the main widget
    widget::Canvas::new()
        .w(ui.w_of(ids.canvas).unwrap() - ui.w_of(ids.tests_canvas).unwrap())
        .h_of(ids.canvas)
        .right_from(ids.tests_canvas, 1.0)
        .color(conrod::color::ORANGE)
        .set(ids.device_canvas, ui);

    /// Title label that tells what the left field consists of
    widget::Text::new("Test List")
        .middle_of(ids.tests_canvas)
        .w_of(ids.tests_canvas)
        .align_top_of(ids.tests_canvas)
        .align_text_middle()
        .line_spacing(5.0)
        .set(ids.tests_title, ui);
        /*
    widget::Text::new("Other List")
        .padded_w_of(ids.canvas, MARGIN)
        .down_from(ids.tests_title, 10.0)
        .align_text_middle()
        .line_spacing(5.0)
        .set(ids.device_list, ui);
        */
        
    const ITEM_HEIGHT: conrod::Scalar = 50.0;

    let tests = app.test_set.all_tests();
    let (mut test_list_widget, test_list_scrollbar) = widget::List::new(tests.len(), ITEM_HEIGHT)
        .scrollbar_on_top()
        .middle_of(ids.tests_canvas)
        .down_from(ids.tests_title, 10.0)
        .w_of(ids.tests_canvas)
        .h(500.0)
        .set(ids.tests_list, ui);
    while let Some(item) = test_list_widget.next(ui) {
        let i = item.i;
        let label = tests[i].name();
        let toggle = widget::Button::new()
            .label(&label)
            .label_color(conrod::color::WHITE)
            .color(conrod::color::LIGHT_BLUE);
        item.set(toggle, ui); // Instantiate the thing to the parent list.
    }
    if let Some(s) = test_list_scrollbar { s.set(ui) }

    /*

    ////////////////
    ///// TEXT /////
    ////////////////


    // We'll demonstrate the `Text` primitive widget by using it to draw a title and an
    // introduction to the example.
    widget::Text::new(TITLE).font_size(TITLE_SIZE).mid_top_of(ids.canvas).set(ids.title, ui);

    const INTRODUCTION: &'static str =
        "This example aims to demonstrate all widgets that are provided by conrod.\
        \n\nThe widget that you are currently looking at is the Text widget. The Text widget \
        is one of several special \"primitive\" widget types which are used to construct \
        all other widget types. These types are \"special\" in the sense that conrod knows \
        how to render them via `conrod::render::Primitive`s.\
        \n\nScroll down to see more widgets!";
    widget::Text::new(INTRODUCTION)
        .padded_w_of(ids.canvas, MARGIN)
        .down(60.0)
        .align_middle_x_of(ids.canvas)
        .align_text_middle()
        .line_spacing(5.0)
        .set(ids.introduction, ui);


    ////////////////////////////
    ///// Lines and Shapes /////
    ////////////////////////////


    widget::Text::new("Lines and Shapes")
        .down(70.0)
        .align_middle_x_of(ids.canvas)
        .font_size(SUBTITLE_SIZE)
        .set(ids.shapes_title, ui);

    // Lay out the shapes in two horizontal columns.
    //
    // TODO: Have conrod provide an auto-flowing, fluid-list widget that is more adaptive for these
    // sorts of situations.
    widget::Canvas::new()
        .down(0.0)
        .align_middle_x_of(ids.canvas)
        .kid_area_w_of(ids.canvas)
        .h(360.0)
        .color(conrod::color::TRANSPARENT)
        .pad(MARGIN)
        .flow_down(&[
            (ids.shapes_left_col, widget::Canvas::new()),
            (ids.shapes_right_col, widget::Canvas::new()),
        ])
        .set(ids.shapes_canvas, ui);

    let shapes_canvas_rect = ui.rect_of(ids.shapes_canvas).unwrap();
    let w = shapes_canvas_rect.w();
    let h = shapes_canvas_rect.h() * 5.0 / 6.0;
    let radius = 10.0;
    widget::RoundedRectangle::fill([w, h], radius)
        .color(conrod::color::CHARCOAL.alpha(0.25))
        .middle_of(ids.shapes_canvas)
        .set(ids.rounded_rectangle, ui);

    let start = [-40.0, -40.0];
    let end = [40.0, 40.0];
    widget::Line::centred(start, end).mid_left_of(ids.shapes_left_col).set(ids.line, ui);

    let left = [-40.0, -40.0];
    let top = [0.0, 40.0];
    let right = [40.0, -40.0];
    let points = once(left).chain(once(top)).chain(once(right));
    widget::PointPath::centred(points).right(SHAPE_GAP).set(ids.point_path, ui);

    widget::Rectangle::fill([80.0, 80.0]).right(SHAPE_GAP).set(ids.rectangle_fill, ui);

    widget::Rectangle::outline([80.0, 80.0]).right(SHAPE_GAP).set(ids.rectangle_outline, ui);

    let bl = [-40.0, -40.0];
    let tl = [-20.0, 40.0];
    let tr = [20.0, 40.0];
    let br = [40.0, -40.0];
    let points = once(bl).chain(once(tl)).chain(once(tr)).chain(once(br));
    widget::Polygon::centred_fill(points).mid_left_of(ids.shapes_right_col).set(ids.trapezoid, ui);

    widget::Oval::fill([40.0, 80.0]).right(SHAPE_GAP + 20.0).align_middle_y().set(ids.oval_fill, ui);

    widget::Oval::outline([80.0, 40.0]).right(SHAPE_GAP + 20.0).align_middle_y().set(ids.oval_outline, ui);

    widget::Circle::fill(40.0).right(SHAPE_GAP).align_middle_y().set(ids.circle, ui);


    /////////////////
    ///// Image /////
    /////////////////


    widget::Text::new("Image")
        .down_from(ids.shapes_canvas, MARGIN)
        .align_middle_x_of(ids.canvas)
        .font_size(SUBTITLE_SIZE)
        .set(ids.image_title, ui);

    const LOGO_SIDE: conrod::Scalar = 144.0;
    widget::Image::new()
        .w_h(LOGO_SIDE, LOGO_SIDE)
        .down(60.0)
        .align_middle_x_of(ids.canvas)
        .set(ids.rust_logo, ui);


    /////////////////////////////////
    ///// Button, XYPad, Toggle /////
    /////////////////////////////////


    widget::Text::new("Button, XYPad and Toggle")
        .down_from(ids.rust_logo, 60.0)
        .align_middle_x_of(ids.canvas)
        .font_size(SUBTITLE_SIZE)
        .set(ids.button_title, ui);

    let ball_x_range = ui.kid_area_of(ids.canvas).unwrap().w();
    let ball_y_range = ui.h_of(ui.window).unwrap() * 0.5;
    let min_x = -ball_x_range / 3.0;
    let max_x = ball_x_range / 3.0;
    let min_y = -ball_y_range / 3.0;
    let max_y = ball_y_range / 3.0;
    let side = 130.0;

    for _press in widget::Button::new()
        .label("PRESS ME")
        .mid_left_with_margin_on(ids.canvas, MARGIN)
        .down_from(ids.button_title, 60.0)
        .w_h(side, side)
        .set(ids.button, ui)
    {
        let x = 1.0;
        let y = 1.0;
        app.ball_xy = [x, y];
    }

    for (x, y) in widget::XYPad::new(app.ball_xy[0], min_x, max_x,
                                     app.ball_xy[1], min_y, max_y)
        .label("BALL XY")
        .wh_of(ids.button)
        .align_middle_y_of(ids.button)
        .align_middle_x_of(ids.canvas)
        .parent(ids.canvas)
        .set(ids.xy_pad, ui)
    {
        app.ball_xy = [x, y];
    }

    let is_white = app.ball_color == conrod::color::WHITE;
    let label = if is_white { "WHITE" } else { "BLACK" };
    for is_white in widget::Toggle::new(is_white)
        .label(label)
        .label_color(if is_white { conrod::color::WHITE } else { conrod::color::LIGHT_CHARCOAL })
        .mid_right_with_margin_on(ids.canvas, MARGIN)
        .align_middle_y_of(ids.button)
        .set(ids.toggle, ui)
    {
        app.ball_color = if is_white { conrod::color::WHITE } else { conrod::color::BLACK };
    }

    let ball_x = app.ball_xy[0];
    let ball_y = app.ball_xy[1] - max_y - side * 0.5 - MARGIN;
    widget::Circle::fill(20.0)
        .color(app.ball_color)
        .x_y_relative_to(ids.xy_pad, ball_x, ball_y)
        .set(ids.ball, ui);


    //////////////////////////////////
    ///// NumberDialer, PlotPath /////
    //////////////////////////////////


    widget::Text::new("NumberDialer and PlotPath")
        .down_from(ids.xy_pad, max_y - min_y + side * 0.5 + MARGIN)
        .align_middle_x_of(ids.canvas)
        .font_size(SUBTITLE_SIZE)
        .set(ids.dialer_title, ui);

    // Use a `NumberDialer` widget to adjust the frequency of the sine wave below.
    let min = 0.5;
    let max = 200.0;
    let decimal_precision = 1;
    for new_freq in widget::NumberDialer::new(app.sine_frequency, min, max, decimal_precision)
        .down(60.0)
        .align_middle_x_of(ids.canvas)
        .w_h(160.0, 40.0)
        .label("F R E Q")
        .set(ids.number_dialer, ui)
    {
        app.sine_frequency = new_freq;
    }

    // Use the `PlotPath` widget to display a sine wave.
    let min_x = 0.0;
    let max_x = std::f32::consts::PI * 2.0 * app.sine_frequency;
    let min_y = -1.0;
    let max_y = 1.0;
    widget::PlotPath::new(min_x, max_x, min_y, max_y, f32::sin)
        .kid_area_w_of(ids.canvas)
        .h(240.0)
        .down(60.0)
        .align_middle_x_of(ids.canvas)
        .set(ids.plot_path, ui);
*/

    /////////////////////
    ///// Scrollbar /////
    /////////////////////


    widget::Scrollbar::y_axis(ids.canvas).auto_hide(true).set(ids.canvas_scrollbar, ui);

}