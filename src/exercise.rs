use serde::{Deserialize, Serialize};

use crate::set::Set;

/// A specific exercise in a workout. This is used in the `/v1/workouts` response.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Exercise {
    /// The index of the exercise in the workout.
    index: u64,
    /// The title of the exercise.
    title: String,
    /// The notes for the workout.
    notes: String,
    /// The exercise template ID.
    #[serde(
        serialize_with = "hex::serde::serialize_upper",
        deserialize_with = "hex::serde::deserialize"
    )]
    exercise_template_id: [u8; 4],
    // TODO: just assuming this is the same id scheme as the rest
    /// The superset ID.
    superset_id: Option<SupersetId>,
    /// The sets in the workout
    sets: Vec<Set>,
}

/// A superset ID.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupersetId(
    #[serde(
        serialize_with = "hex::serde::serialize_upper",
        deserialize_with = "hex::serde::deserialize"
    )]
    pub [u8; 4],
);

#[cfg(test)]
mod tests {
    use crate::workout_type::SetType;

    use super::*;

    #[test]
    fn deserialize_exercise() {
        let exercise = r#"
{
  "index": 0,
  "title": "Lat Pulldown (Cable)",
  "notes": "",
  "exercise_template_id": "6A6C31A5",
  "superset_id": null,
  "sets": [
    {
      "index": 0,
      "set_type": "normal",
      "weight_kg": 58.96707822663317,
      "reps": 5,
      "distance_meters": null,
      "duration_seconds": null,
      "rpe": null
    },
    {
      "index": 1,
      "set_type": "normal",
      "weight_kg": 63.50300732098956,
      "reps": 5,
      "distance_meters": null,
      "duration_seconds": null,
      "rpe": null
    }
  ]
}
        "#;

        let got_exercise: Exercise = serde_json::from_str(exercise).unwrap();
        let expected_exercise = Exercise {
            index: 0,
            title: "Lat Pulldown (Cable)".into(),
            notes: "".into(),
            exercise_template_id: [0x6a, 0x6c, 0x31, 0xa5],
            superset_id: None,
            sets: vec![
                Set {
                    index: 0,
                    set_type: SetType::Normal,
                    weight_kg: Some(58.96707822663317),
                    reps: Some(5),
                    distance_meters: None,
                    duration_seconds: None,
                    rpe: None,
                },
                Set {
                    index: 1,
                    set_type: SetType::Normal,
                    weight_kg: Some(63.50300732098956),
                    reps: Some(5),
                    distance_meters: None,
                    duration_seconds: None,
                    rpe: None,
                },
            ],
        };

        assert_eq!(expected_exercise, got_exercise);
    }
}
