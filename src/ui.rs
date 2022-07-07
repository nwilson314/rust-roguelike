use bevy::{
    prelude::Color as BevyColor, prelude::*, render::camera::ScalingMode, window::PresentMode,
};

use kayak_ui::{
    bevy::{BevyContext, BevyKayakUIPlugin, FontMapping, UICameraBundle},
    core::{
        bind, render, rsx,
        styles::{Edge, StyleProp},
        styles::{Style, Units},
        use_state, widget, Binding, Bound, Color, EventType, Index, MutableBound, OnEvent,
        OnLayout, WidgetProps,
    },
    widgets,
    widgets::Background,
    widgets::Window,
};

use crate::prelude::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(BevyKayakUIPlugin)
            .add_startup_system(game_ui);
    }
}

fn game_ui(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(UICameraBundle::new());
    font_mapping.set_default(asset_server.load("roboto.kayak_font"));

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
    let hp_bar_outer = Style {
        background_color: StyleProp::Value(Color::new(135.0/255.0, 35.0/255.0, 35.0/255.0, 1.0)),
        top: StyleProp::Value(Units::Pixels(10.0)),
        left: StyleProp::Value(Units::Pixels(10.0)),
        min_height: StyleProp::Value(Units::Pixels(50.0)),
        max_height: StyleProp::Value(Units::Pixels(50.0)),
        min_width: StyleProp::Value(Units::Pixels(SCREEN_WIDTH/2.0 - 40.0)),
        max_width: StyleProp::Value(Units::Pixels(SCREEN_WIDTH/2.0 - 40.0)),
        ..default()
    };
    let hp_bar_inner = Style {
        background_color: StyleProp::Value(bevy_color_to_color(BevyColor::RED)),
        top: StyleProp::Value(Units::Percentage(15.0)),
        left: StyleProp::Value(Units::Percentage(1.0)),
        right: StyleProp::Value(Units::Percentage(1.0)),
        bottom: StyleProp::Value(Units::Percentage(15.0)),
        // min_height: StyleProp::Value(Units::Percentage(50.0)),
        // max_height: StyleProp::Value(Units::Percentage(50.0)),
        // min_width: StyleProp::Value(Units::Pixels(SCREEN_WIDTH/2.0 - 40.0)),
        // max_width: StyleProp::Value(Units::Percentage(99.0)),
        ..default()
    };
    rsx! {
        <Background styles={Some(background)}>
            <Window position={(0.0,SCREEN_HEIGHT-200.0)} size={(SCREEN_WIDTH/2.0, 200.0)} styles={Some(window)}>
                <Background styles={Some(hp_bar_outer)}>
                    <Background styles={Some(hp_bar_inner)}>
                    </Background>
                </Background>
            </Window>
            <Window position={(SCREEN_WIDTH/2.0,SCREEN_HEIGHT-200.0)} size={(SCREEN_WIDTH/2.0, 200.0)} styles={Some(window)}>
            </Window>
        </Background>
    }
}

fn bevy_color_to_color(bevy_color: BevyColor) -> Color {
    Color { r: bevy_color.r(), g: bevy_color.g(), b: bevy_color.b(), a: bevy_color.a() }
}
