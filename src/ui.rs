use bevy::{
    prelude::Color as BevyColor, prelude::*, render::camera::ScalingMode, window::PresentMode,
};

use kayak_ui::{
    bevy::{BevyContext, BevyKayakUIPlugin, FontMapping, UICameraBundle},
    core::{
        bind, render, rsx,
        styles::{Edge, StyleProp},
        styles::{Style, Units, LayoutType},
        use_state, widget, Binding, Bound, Color, EventType, Index, MutableBound, OnEvent,
        OnLayout, WidgetProps,
    },
    widgets,
    widgets::Background,
    widgets::Window,
    widgets::Element,
};

use crate::prelude::{SCREEN_HEIGHT, SCREEN_WIDTH, Health, Player, TurnState};

const HP_MAX: f32 = 10.0;
const HP_MIN: f32 = 640.0;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(BevyKayakUIPlugin)
            .add_startup_system(game_ui)
            .add_system(update_health)
            ;
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
    
    let binding = bind(health.clone());
    let count_binding = bind(Counter {
        count: 0
    }.clone());
    commands.insert_resource(binding);
    commands.insert_resource(count_binding);
    let context = BevyContext::new(|context| {
        render! {
            <widgets::App>
                <BottomUI />
            </widgets::App>
        }
    });

    commands.insert_resource(context);
}


#[widget]
fn BottomUI() {
    let health_binding = context.query_world::<Res<Binding<Health>>, _, _>(move |health| health.clone());
    // context.bind(&health_binding);
    let health = health_binding.get();
    
    println!("player_health: {:?}", health);

    let hp_width = (1.0 - (health.current as f32 / health.max as f32 )) * (HP_MIN-HP_MAX) + HP_MAX;

    println!("Bar width: {:?}", hp_width);

    context.bind(&health_binding);

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
        background_color: StyleProp::Value(Color::new(135.0/255.0, 35.0/255.0, 35.0/255.0, 1.0)),
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
            <Window position={(0.0,SCREEN_HEIGHT-200.0)} size={(SCREEN_WIDTH/2.0, 200.0)} styles={Some(window)}>
                <Element styles={Some(left_box)}>
                <widgets::Text content={format!("Hp: {}/{}", health.current, health.max)} size={20.0} styles={Some(hp_text)}/>
                <Background styles={Some(hp_bar_outer)}>
                    <Background styles={Some(hp_bar_inner)}>
                    </Background>
                </Background>
                </Element>
            </Window>
            <Window position={(SCREEN_WIDTH/2.0,SCREEN_HEIGHT-200.0)} size={(SCREEN_WIDTH/2.0, 200.0)} styles={Some(window)}>
            </Window>
        </Background>
    }
}

fn bevy_color_to_color(bevy_color: BevyColor) -> Color {
    Color { r: bevy_color.r(), g: bevy_color.g(), b: bevy_color.b(), a: bevy_color.a() }
}
#[derive(Clone, Copy, PartialEq, Debug)]
    pub struct Counter{
        count: i32
    }
fn update_health(
    mut player_query: Query<&mut Health, With<Player>>,
    health_binding: Res<Binding<Health>>,
    counter_binding: Res<Binding<Counter>>,
) {
    let mut player_health = player_query.single_mut();
    let mut new_counter = counter_binding.get();
    new_counter.count += 1;

    
    if new_counter.count % 100 == 0 {
        player_health.current -= 1;
    } 
    health_binding.set(player_health.clone());
    counter_binding.set(new_counter.clone());
    println!("Count: {:?}", new_counter);
    println!("Health: {:?}", player_health);
}
