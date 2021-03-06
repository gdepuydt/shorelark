use crate::*;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Animal {
    pub position: Vector,
    pub rotation: f32,
    pub eye_cells: Vec<f32>,
}

impl From<&sim::Animal> for Animal {
    fn from(animal: &sim::Animal) -> Self {
        Self {
            position: Vector::from(animal.position()),
            rotation: animal.rotation().angle(),
            eye_cells: animal.eye().energies().to_owned(),
        }
    }
}
