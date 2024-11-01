use diesel::{
    prelude::{Associations, Insertable, Queryable},
    Selectable,
};
use pgvector::Vector;
use rand::Rng;
use serde::Serialize;
use ts_rs::TS;

use crate::schema::{lines, song};

#[derive(Debug, Serialize, Clone, Default, TS)]
#[ts(export)]
pub struct LoadSong {
    pub title: String,
    pub lines: Vec<LineComp>,
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = song)]
pub struct DbLoadSong {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Clone, Default, TS)]
#[ts(export)]
pub struct LineComp {
    pub line: String,
    pub position: Vector3,
    pub cam_look_at: Vector3,
    pub cam_position: Vector3,
    pub color: Option<Color>,

    // Animation values
    pub end_position: Option<Vector3>,
    pub cam_end_position: Option<Vector3>,
    pub cam_end_look_at: Option<Vector3>,
}

#[derive(Debug, Queryable, Selectable, Associations)]
#[diesel(table_name = lines)]
#[diesel(belongs_to(DbLoadSong, foreign_key = song_id))]
pub struct DbLineComp {
    pub id: i32,
    pub line: String,
    pub song_id: i32,
    pub position: Vector,
    pub cam_position: Vector,
    pub cam_look_at: Vector,
    pub end_position: Option<Vector>,
    pub cam_end_position: Option<Vector>,
    pub cam_end_look_at: Option<Vector>,
}

#[derive(Debug, Insertable, Associations)]
#[diesel(table_name = lines)]
#[diesel(belongs_to(LoadSong, foreign_key = song_id))]
pub struct NewDbLineComp {
    pub line: String,
    pub song_id: i32,
    pub position: Vector,
    pub cam_position: Vector,
    pub cam_look_at: Vector,
    pub end_position: Option<Vector>,
    pub cam_end_position: Option<Vector>,
    pub cam_end_look_at: Option<Vector>,
}

impl From<DbLineComp> for LineComp {
    fn from(value: DbLineComp) -> Self {
        LineComp {
            line: value.line,
            position: value.position.into(),
            cam_look_at: value.cam_look_at.into(),
            cam_position: value.cam_position.into(),
            end_position: value.end_position.map(|v| v.into()),
            cam_end_position: value.cam_end_position.map(|v| v.into()),
            cam_end_look_at: value.cam_end_look_at.map(|v| v.into()),
            color: None,
        }
    }
}

impl From<LineComp> for NewDbLineComp {
    fn from(value: LineComp) -> Self {
        NewDbLineComp {
            line: value.line,
            song_id: 0,
            position: value.position.into(),
            cam_look_at: value.cam_look_at.into(),
            cam_position: value.cam_position.into(),
            end_position: value.end_position.map(|v| v.into()),
            cam_end_position: value.cam_end_position.map(|v| v.into()),
            cam_end_look_at: value.cam_end_look_at.map(|v| v.into()),
        }
    }
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

impl From<Vector> for Vector3 {
    fn from(value: Vector) -> Self {
        let val = value.as_slice();
        Vector3 {
            x: val[0],
            y: val[1],
            z: val[2],
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
            cam_position: Vector3 {
                x: 0.0,
                y: 10.0,
                z: 150.0,
            },
            ..Default::default()
        }
    }
}
