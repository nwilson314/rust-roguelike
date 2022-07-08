use bevy::{core::Zeroable, prelude::Color as BevyColor, prelude::*, render::camera::RenderTarget};

use kayak_ui::{
    bevy::{BevyContext, BevyKayakUIPlugin, FontMapping, UICameraBundle},
    core::{
        bind, render,
        render_command::RenderCommand,
        rsx,
        styles::{Edge, StyleProp},
        styles::{LayoutType, Style, Units},
        use_state, widget, Binding, Bound, Color, EventType, Index, MutableBound, OnEvent,
        OnLayout, WidgetProps,
    },
    widgets,
    widgets::Background,
    widgets::Element,
    widgets::Window,
};

use crate::prelude::{
    cursor_to_world, Health, MainCamera, Player, ToolTipInfo, ToolTipPos, TurnState, SCREEN_HEIGHT,
    SCREEN_WIDTH,
};

const HP_MAX: f32 = 10.0;
const HP_MIN: f32 = 640.0;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(BevyKayakUIPlugin)
            .add_startup_system(game_ui)
            .add_system(update_health)
            .add_system(update_tool_tip)
            .add_system_set(
                SystemSet::on_exit(TurnState::AwaitingInput).with_system(remove_tool_tip),
            );
    }
}

fn game_ui(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(UICameraBundle::new());
    font_mapping.set_default(asset_server.load("roboto.kayak_font"));
    let health = Health {
        current: 20,
        max: 20,
    };

    let tooltip_pos = ToolTipPos {
        pos: Vec2::new(0.0, 0.0),
        color: BevyColor::rgba(0.0, 0.0, 0.0, 0.0),
        info: ToolTipInfo {
            name: Name::new(""),
            ..default()
        },
    };

    let health_binding = bind(health.clone());
    let tooltip_binding = bind(tooltip_pos.clone());
    commands.insert_resource(health_binding);
    commands.insert_resource(tooltip_binding);

    let context = BevyContext::new(|context| {
        let app_styles = Style {
            layout_type: StyleProp::Value(LayoutType::Column),
            row_between: StyleProp::Value(Units::Pixels(5.0)),
            ..default()
        };
        render! {
            <widgets::App styles={Some(app_styles)}>
                <ToolTipUi />
                <BottomUI />

            </widgets::App>
        }
    });

    commands.insert_resource(context);
}

#[widget]
fn BottomUI() {
    let health_binding =
        context.query_world::<Res<Binding<Health>>, _, _>(move |health| health.clone());
    context.bind(&health_binding);
    let health = health_binding.get();

    // println!("player_health: {:?}", health);

    let hp_width = (1.0 - (health.current as f32 / health.max as f32)) * (HP_MIN - HP_MAX) + HP_MAX;

    // println!("Bar width: {:?}", hp_width);

    let background = Style {
        bottom: StyleProp::Value(Units::Pixels(0.0)),
        min_width: StyleProp::Value(Units::Pixels(SCREEN_WIDTH)),
        min_height: StyleProp::Value(Units::Pixels(200.0)),
        ..default()
    };
    let window = Style {
        background_color: StyleProp::Value(Color::new(0.125, 0.125, 0.125, 1.0)),
        border_color: StyleProp::Value(Color::new(0.0781, 0.0898, 0.101, 1.0)),
        color: StyleProp::Value(Color::new(0.5, 1.0, 1.0, 1.0)),
        ..default()
    };
    let left_box = Style {
        layout_type: StyleProp::Value(LayoutType::Row),
        col_between: StyleProp::Value(Units::Pixels(5.0)),
        ..default()
    };
    let hp_bar_outer = Style {
        background_color: StyleProp::Value(Color::new(
            135.0 / 255.0,
            35.0 / 255.0,
            35.0 / 255.0,
            1.0,
        )),
        top: StyleProp::Value(Units::Pixels(10.0)),
        left: StyleProp::Value(Units::Pixels(10.0)),
        right: StyleProp::Value(Units::Pixels(10.0)),
        min_height: StyleProp::Value(Units::Pixels(50.0)),
        max_height: StyleProp::Value(Units::Pixels(50.0)),
        // min_width: StyleProp::Value(Units::Percentage(95.0)),
        // max_width: StyleProp::Value(Units::Percentage(95.0)),
        ..default()
    };
    let hp_bar_inner = Style {
        background_color: StyleProp::Value(bevy_color_to_color(BevyColor::RED)),
        top: StyleProp::Value(Units::Percentage(15.0)),
        left: StyleProp::Value(Units::Pixels(10.0)),
        right: StyleProp::Value(Units::Pixels(hp_width)),
        bottom: StyleProp::Value(Units::Percentage(15.0)),
        // min_height: StyleProp::Value(Units::Percentage(50.0)),
        // max_height: StyleProp::Value(Units::Percentage(50.0)),
        // min_width: StyleProp::Value(Units::Pixels(SCREEN_WIDTH/2.0 - 40.0)),
        // max_width: StyleProp::Value(Units::Percentage(99.0)),
        ..default()
    };
    let hp_text = Style {
        color: StyleProp::Value(Color::WHITE),
        top: StyleProp::Value(Units::Pixels(20.0)),
        left: StyleProp::Value(Units::Pixels(5.0)),
        // min_width: StyleProp::Value(Units::Percentage(15.0)),
        // max_width: StyleProp::Value(Units::Percentage(15.0)),
        ..default()
    };

    rsx! {
        <Background styles={Some(background)}>
            <Window position={(0.0,0.0)} size={(SCREEN_WIDTH/2.0, 200.0)} styles={Some(window)}>
                <Element styles={Some(left_box)}>
                    <widgets::Text content={format!("Hp: {}/{}", health.current, health.max)} size={20.0} styles={Some(hp_text)}/>
                    <Background styles={Some(hp_bar_outer)}>
                        <Background styles={Some(hp_bar_inner)}>
                        </Background>
                    </Background>
                </Element>
            </Window>
            <Window position={(SCREEN_WIDTH/2.0,0.0)} size={(SCREEN_WIDTH/2.0, 200.0)} styles={Some(window)}>
            </Window>
        </Background>
    }
}

#[widget]
fn ToolTipUi() {
    let tooltip_binding =
        context.query_world::<Res<Binding<ToolTipPos>>, _, _>(|tooltip| tooltip.clone());
    context.bind(&tooltip_binding);

    let tooltip = tooltip_binding.get();
    let text_content = format!(
        "{}, HP: {}/{}",
        tooltip.info.name.to_string(),
        tooltip.info.health.current,
        tooltip.info.health.max
    );

    let tool = Style {
        top: StyleProp::Value(Units::Pixels(0.0)),
        left: StyleProp::Value(Units::Pixels(0.0)),
        max_height: StyleProp::Value(Units::Pixels(SCREEN_HEIGHT - 200.0)),
        min_height: StyleProp::Value(Units::Pixels(SCREEN_HEIGHT - 200.0)),
        max_width: StyleProp::Value(Units::Pixels(SCREEN_WIDTH)),
        min_width: StyleProp::Value(Units::Pixels(SCREEN_WIDTH)),
        ..default()
    };
    let tool_box = Style {
        background_color: StyleProp::Value(bevy_color_to_color(tooltip.color)),
        max_height: StyleProp::Value(Units::Pixels(30.0)),
        min_height: StyleProp::Value(Units::Pixels(30.0)),
        max_width: StyleProp::Value(Units::Pixels(100.0)),
        min_width: StyleProp::Value(Units::Pixels(100.0)),
        top: StyleProp::Value(Units::Pixels(SCREEN_HEIGHT - tooltip.pos.y)),
        left: StyleProp::Value(Units::Pixels(tooltip.pos.x)),
        ..default()
    };
    let text = Style { ..default() };

    rsx! {
        <Element styles={Some(tool)}>
            <Background styles={Some(tool_box)}>
                <widgets::Text content={text_content} styles={Some(text)}/>
            </Background>
        </Element>
    }
}

fn bevy_color_to_color(bevy_color: BevyColor) -> Color {
    Color {
        r: bevy_color.r(),
        g: bevy_color.g(),
        b: bevy_color.b(),
        a: bevy_color.a(),
    }
}

fn update_health(player_query: Query<&Health, With<Player>>, health_binding: Res<Binding<Health>>) {
    let player_health = player_query.single();
    health_binding.set(player_health.clone());
}

fn update_tool_tip(
    cursor: Res<Input<MouseButton>>,
    context: Res<BevyContext>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    entity_query: Query<(&Transform, &Health, &Name)>,
    windows: Res<Windows>,
    tooltip_binding: Res<Binding<ToolTipPos>>,
) {
    if !cursor.just_pressed(MouseButton::Left) {
        // Only run this system when the mouse button is clicked
        return;
    }

    if context.contains_cursor() {
        // This is the important bit:
        // If the cursor is over a part of the UI, then we should not allow clicks to pass through to the world
        return;
    }
    let (camera, camera_transform) = camera_query.single();
    let wnd = if let RenderTarget::Window(id) = camera.target {
        windows.get(id).unwrap()
    } else {
        windows.get_primary().unwrap()
    };

    let cursor_pos = wnd.cursor_position();

    if let Some(cursor_pos) = cursor_pos {
        let world_pos = cursor_to_world(&windows, camera, camera_transform);
        for (position, health, name) in entity_query.iter() {
            if position.translation.truncate().distance(world_pos) < 15.0 {
                tooltip_binding.set(ToolTipPos {
                    pos: cursor_pos,
                    color: BevyColor::GRAY,
                    info: ToolTipInfo {
                        name: Name::new(name.to_string()),
                        health: *health,
                    },
                });
                return;
            }
        }

        // No entity at clicked position so remove tooltip
        tooltip_binding.set(ToolTipPos {
            pos: Vec2::new(0.0, 0.0),
            color: BevyColor::rgba(0.0, 0.0, 0.0, 0.0),
            ..default()
        });
    }
}

fn remove_tool_tip(tooltip_binding: Res<Binding<ToolTipPos>>) {
    tooltip_binding.set(ToolTipPos {
        pos: Vec2::new(0.0, 0.0),
        color: BevyColor::rgba(0.0, 0.0, 0.0, 0.0),
        ..default()
    });
}
