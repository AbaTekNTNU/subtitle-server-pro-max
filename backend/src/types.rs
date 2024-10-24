use rand::Rng;
use serde::Serialize;
use ts_rs::TS;

#[derive(Debug, Serialize, Clone, Default, TS)]
#[ts(export)]
pub struct LoadSong {
    title: String,
    lines: Vec<LineComp>,
}

#[derive(Debug, Serialize, Clone, Default, TS)]
#[ts(export)]
pub struct LineComp {
    line: String,
    position: Vector3,
    cam_look_at: Vector3,
    color: Option<Color>,
    cam_position: Option<Vector3>,

    // Animation values
    end_positiion: Option<Vector3>,
}

#[derive(Debug, Serialize, Clone, TS)]
#[ts(export)]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Default for Vector3 {
    fn default() -> Self {
        Self {
            x: rand::thread_rng().gen_range(1..50) as f64,
            y: rand::thread_rng().gen_range(1..50) as f64,
            z: rand::thread_rng().gen_range(1..50) as f64,
        }
    }
}

#[derive(Debug, Serialize, Clone, Default, TS)]
#[ts(export)]
pub struct Color {
    color: [&'static str; 6],
}

impl From<Vec<String>> for LoadSong {
    fn from(value: Vec<String>) -> Self {
        LoadSong {
            title: "Undefined".to_string(),
            lines: value.into_iter().map(LineComp::from).collect(),
        }
    }
}

impl From<String> for LineComp {
    fn from(value: String) -> Self {
        LineComp {
            line: value,
            cam_position: Some(Vector3 {
                x: 0.0,
                y: 10.0,
                z: 150.0,
            }),
            ..Default::default()
        }
    }
}
