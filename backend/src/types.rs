use pgvector::Vector;
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
    pub line: String,
    pub position: Vector3,
    pub cam_look_at: Vector3,
    pub cam_position: Option<Vector3>,
    pub color: Option<Color>,

    // Animation values
    pub end_positiion: Option<Vector3>,
    pub cam_end_position: Option<Vector3>,
    pub cam_end_look_at: Option<Vector3>,
}

#[derive(Debug, Serialize, Clone, TS)]
#[ts(export)]
pub struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Default for Vector3 {
    fn default() -> Self {
        Self {
            x: rand::thread_rng().gen_range(-40..40) as f32,
            y: rand::thread_rng().gen_range(1..50) as f32,
            z: rand::thread_rng().gen_range(-25..25) as f32,
        }
    }
}

impl From<Vector3> for pgvector::Vector {
    fn from(value: Vector3) -> Self {
        Vector::from(vec![value.x, value.y, value.z])
    }
}

impl TryFrom<Vector> for Vector3 {
    type Error = &'static str;

    fn try_from(value: Vector) -> Result<Self, Self::Error> {
        let val = value.to_vec();
        if val.len() != 3 {
            return Err("Vector must have 3 values");
        }

        Ok(Vector3 {
            x: val[0],
            y: val[1],
            z: val[2],
        })
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
