use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
        title: "Bevy Playground".to_string(),
        canvas: Some("#playground".to_string()),
        fit_canvas_to_parent: true,
        ..default()
    })
    .add_plugins(DefaultPlugins);

    app.add_startup_system(setup).add_system(reactive).run();
}

#[derive(Debug, Component)]
struct Menu;

fn reactive(
    windows: Res<Windows>,
    mut root: Query<&mut Style, (Without<Parent>, Without<Menu>)>,
    mut menu: Query<&mut Style, With<Menu>>,
) {
    let window = windows.get_primary().unwrap();
    let ratio = window.width() / window.height();
    let mut root = root.single_mut();
    let mut menu = menu.single_mut();
    if ratio > 1.0 {
        root.flex_direction = FlexDirection::Row;
        menu.size = Size::new(Val::Percent(50.0), Val::Percent(100.0));
    } else {
        root.flex_direction = FlexDirection::Column;
        menu.size = Size::new(Val::Percent(100.0), Val::Percent(50.0));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                flex_basis: Val::Auto,
                                aspect_ratio: Some(0.8),
                                size: Size::new(Val::Percent(80.0), Val::Percent(80.0)),
                                margin: UiRect::all(Val::Auto),
                                border: UiRect::all(Val::Px(20.0)),
                                ..default()
                            },
                            color: Color::rgb(0.4, 0.4, 1.0).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            let size = 22.0;
                            parent
                                .spawn_bundle(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                        flex_direction: FlexDirection::ColumnReverse,
                                        justify_content: JustifyContent::SpaceBetween,
                                        ..default()
                                    },
                                    color: Color::rgb(0.8, 0.8, 1.0).into(),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    add_line(parent, size);
                                    add_line(parent, size);
                                    add_line(parent, size);
                                    add_line(parent, size);
                                });
                        });
                });
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::ColumnReverse,
                        justify_content: JustifyContent::Center,
                        size: Size::new(Val::Percent(50.0), Val::Percent(100.0)),
                        ..default()
                    },
                    color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..default()
                })
                .insert(Menu);
        });
}

fn add_line(parent: &mut ChildBuilder, size: f32) {
    parent
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(size)),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(size), Val::Percent(100.0)),
                    ..default()
                },
                color: Color::rgb(0.2, 0.2, 1.0).into(),
                ..default()
            });
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(size), Val::Percent(100.0)),
                    ..default()
                },
                color: Color::rgb(0.2, 0.2, 1.0).into(),
                ..default()
            });
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(size), Val::Percent(100.0)),
                    ..default()
                },
                color: Color::rgb(0.2, 0.2, 1.0).into(),
                ..default()
            });
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(size), Val::Percent(100.0)),
                    ..default()
                },
                color: Color::rgb(0.2, 0.2, 1.0).into(),
                ..default()
            });
        });
}
