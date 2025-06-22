use bevy::prelude::*;
use bevy::window::WindowTheme;

use crate::{
    audio::AudioPlugin,
    game::GamePlugin,
    graphics::GraphicsPlugin,
    input::InputPlugin,
    network::NetworkPlugin,
    settings::SettingsPlugin,
    ui::UIPlugin,
};

use super::{
    states::GameState,
    resources::*,
    events::*,
};

pub struct ChessClientApp;

impl ChessClientApp {
    pub fn new() -> App {
        let mut app = App::new();

        // Bevy の基本プラグイン
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Chess 3D".into(),
                        resolution: (1280.0, 720.0).into(),
                        present_mode: bevy::window::PresentMode::AutoVsync,
                        window_theme: Some(WindowTheme::Dark),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .build()
        );

        // ゲーム状態
        app.add_state::<GameState>();

        // グローバルリソース
        app.insert_resource(Msaa::Sample4)
            .insert_resource(ClearColor(Color::rgb(0.08, 0.08, 0.12)))
            .init_resource::<GameSettings>()
            .init_resource::<UIState>()
            .init_resource::<CameraController>()
            .init_resource::<NetworkState>()
            .init_resource::<AudioSettings>();

        // イベント
        app.add_event::<PieceSelectedEvent>()
            .add_event::<MovePieceEvent>()
            .add_event::<SendNetworkMessageEvent>()
            .add_event::<NetworkResponseEvent>()
            .add_event::<UIStateChangeEvent>()
            .add_event::<CameraControlEvent>()
            .add_event::<GameActionEvent>()
            .add_event::<AudioEvent>();

        // カスタムプラグイン
        app.add_plugins((
            SettingsPlugin,    // 設定管理（最初に読み込み）
            GraphicsPlugin,    // 3Dグラフィック
            AudioPlugin,       // オーディオシステム
            InputPlugin,       // 入力処理
            GamePlugin,        // ゲームロジック
            NetworkPlugin,     // ネットワーク通信
            UIPlugin,          // ユーザーインターフェース
        ));

        // システムセット定義
        app.configure_sets(
            Update,
            (
                CoreSet::Input,
                CoreSet::Logic,
                CoreSet::Network,
                CoreSet::Graphics,
                CoreSet::Audio,
                CoreSet::UI,
            ).chain()
        );

        // スタートアップシステム
        app.add_systems(Startup, (
            setup_app,
            setup_debug_info,
        ));

        // コアシステム
        app.add_systems(Update, (
            handle_window_events,
            update_performance_stats,
        ).in_set(CoreSet::Input));

        app
    }

    pub fn run(self) -> ! {
        self.new().run();
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum CoreSet {
    Input,
    Logic,
    Network,
    Graphics,
    Audio,
    UI,
}

fn setup_app(
    mut commands: Commands,
    mut clear_color: ResMut<ClearColor>,
    settings: Res<GameSettings>,
) {
    info!("Initializing Chess 3D Client");
    
    // 背景色を設定に基づいて調整
    match settings.graphics_quality {
        crate::core::resources::GraphicsQuality::Low => {
            *clear_color = ClearColor(Color::rgb(0.05, 0.05, 0.08));
        },
        crate::core::resources::GraphicsQuality::Medium => {
            *clear_color = ClearColor(Color::rgb(0.08, 0.08, 0.12));
        },
        crate::core::resources::GraphicsQuality::High => {
            *clear_color = ClearColor(Color::rgb(0.1, 0.1, 0.15));
        },
    }

    // パフォーマンス統計用のリソースを初期化
    commands.insert_resource(PerformanceStats::default());
}

fn setup_debug_info(
    mut commands: Commands,
) {
    #[cfg(debug_assertions)]
    {
        // デバッグビルドでのみデバッグ情報を表示
        commands.spawn((
            TextBundle::from_sections([
                TextSection::new(
                    "Chess 3D - Debug Mode\n",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                TextSection::from_style(TextStyle {
                    font_size: 16.0,
                    color: Color::YELLOW,
                    ..default()
                }),
            ])
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(10.0),
                ..default()
            }),
            DebugInfoText,
        ));
    }
}

fn handle_window_events(
    mut exit_events: EventReader<bevy::window::WindowCloseRequested>,
    mut app_exit_events: EventWriter<bevy::app::AppExit>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    // ウィンドウが閉じられた時の処理
    for _ in exit_events.read() {
        info!("Window close requested");
        app_exit_events.send(bevy::app::AppExit);
    }

    // Alt+F4 や Cmd+Q でアプリケーション終了
    if keyboard_input.just_pressed(KeyCode::F4) && keyboard_input.pressed(KeyCode::AltLeft) {
        info!("Alt+F4 pressed, exiting application");
        app_exit_events.send(bevy::app::AppExit);
    }

    #[cfg(target_os = "macos")]
    if keyboard_input.just_pressed(KeyCode::Q) && keyboard_input.pressed(KeyCode::SuperLeft) {
        info!("Cmd+Q pressed, exiting application");
        app_exit_events.send(bevy::app::AppExit);
    }
}

fn update_performance_stats(
    time: Res<Time>,
    mut stats: ResMut<PerformanceStats>,
    diagnostics: Res<bevy::diagnostic::DiagnosticsStore>,
    mut debug_text: Query<&mut Text, With<DebugInfoText>>,
) {
    stats.frame_count += 1;
    stats.total_time += time.delta_seconds();

    // 1秒ごとに統計を更新
    if stats.total_time >= 1.0 {
        stats.fps = stats.frame_count as f32 / stats.total_time;
        stats.frame_count = 0;
        stats.total_time = 0.0;

        // メモリ使用量を取得（可能な場合）
        if let Some(memory_diagnostic) = diagnostics.get(bevy::diagnostic::SystemInformationDiagnosticsPlugin::SYSTEM_MEMORY) {
            if let Some(memory_value) = memory_diagnostic.smoothed() {
                stats.memory_usage_mb = memory_value / 1024.0 / 1024.0;
            }
        }

        // デバッグテキストを更新
        #[cfg(debug_assertions)]
        if let Ok(mut text) = debug_text.get_single_mut() {
            text.sections[1].value = format!(
                "FPS: {:.1}\nMemory: {:.1} MB\nFrame Time: {:.2} ms",
                stats.fps,
                stats.memory_usage_mb,
                1000.0 / stats.fps.max(1.0)
            );
        }
    }
}

#[derive(Component)]
struct DebugInfoText;