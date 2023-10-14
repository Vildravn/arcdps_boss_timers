use arc_util::{
    colors::WHITE,
    ui::{render, Component},
};
use arcdps::{
    extras::{ExtrasAddonInfo, UserInfoIter, UserRole},
    Agent, CombatEvent, StateChange,
    imgui::{self, Ui},
    exports::{self, CoreColor},
};
use log::info;
use imgui::Condition;

const FONT_SIZE: f32 = 2.0;

arcdps::export! {
    name: "Boss Timers",
    sig: 0xf1ff0e2c, // random sig
    init,
    combat,
    extras_init,
    extras_squad_update,
    imgui,
}

fn init() -> Result<(), String> {
    info!("plugin has been started");
    // for info level target "window" is the same as not specifying target
    info!(target: "window", "only window logging");
    info!(target: "file", "only file logging");
    info!(target: "both", "logging to file and window");
    Ok(())
}

fn render_text(ui: &Ui, text: &str) {
    let colors = exports::colors();
    let white = colors.core(CoreColor::White).unwrap_or(WHITE);

    let [cursor_x, cursor_y] = ui.cursor_pos();
    let [text_width, _] = ui.calc_text_size(text);
    let window_width = ui.window_content_region_width();
    ui.set_cursor_pos([cursor_x + 0.5 * (window_width - text_width), cursor_y]);

    ui.text_colored(white, text);
}

fn combat(
    event: Option<CombatEvent>,
    src: Option<Agent>,
    _dst: Option<Agent>,
    _skill_name: Option<&str>,
    _id: u64,
    _revision: u64,
) {
    if let (Some(event), Some(src)) = (event, src) {
        if let StateChange::EnterCombat = event.is_statechange {
            info!(
                "{} ({}) has entered combat",
                src.name.unwrap_or("unknown agent"),
                src.id
            );
        }
    }
}

fn extras_init(extras_info: ExtrasAddonInfo, account_name: Option<&str>) {
    info!(
        "extras version {} on account {}",
        extras_info.string_version.unwrap_or("unknown"),
        account_name.unwrap_or("unknown")
    );
}

fn extras_squad_update(users: UserInfoIter) {
    for user in users {
        if let UserRole::SquadLeader | UserRole::Lieutenant = user.role {
            info!(
                "{} can place markers",
                user.account_name.unwrap_or("unknown user")
            );
        }
    }
}

fn imgui(ui: &Ui, not_loading_or_character_selection: bool) {
    if (not_loading_or_character_selection) {
        render(ui, ())
    }
}

fn render(ui: &Ui, _: ()) {
    let [screen_width, screen_height] = ui.io().display_size;

    imgui::Window::new("##boss-timers-popup")
        .position(
            [0.5 * screen_width, 0.2 * screen_height],
            Condition::Always
        )
        .position_pivot([0.5, 0.5])
        .content_size([screen_width, 0.0])
        .always_auto_resize(true)
        .no_decoration()
        .draw_background(false)
        .no_inputs()
        .movable(false)
        .focus_on_appearing(false)
        .build(ui, || {
            ui.set_window_font_scale(FONT_SIZE);

            render_text(ui, "Test");
        });
}