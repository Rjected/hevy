use crate::workout_type::SetType;
use serde::{Deserialize, Serialize};

/// The description of a specific set in a routine.
///
/// The same as the `PostRoutinesRequestSet` type in the hevy API docs:
/// <https://api.hevyapp.com/docs>
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RoutineSet {
    /// The type of set
    #[serde(rename = "type")]
    set_type: SetType,
    /// The weight in kilograms
    weight_kg: Option<f64>,
    /// The number of reps in the set
    reps: Option<u64>,
    /// The distance of the set in meters
    distance_meters: Option<f64>,
    /// The duration of the set in seconds
    duration_seconds: Option<f64>,
}

/// The description of a set in a specific workout.
///
/// This is included in the `/v1/workouts` response.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Set {
    /// The index for the set in the workout
    pub(crate) index: u32,
    /// The type of set
    pub(crate) set_type: SetType,
    /// The weight in kilograms
    pub(crate) weight_kg: Option<f64>,
    /// The number of reps in the set
    pub(crate) reps: Option<u64>,
    /// The distance of the set in meters
    pub(crate) distance_meters: Option<f64>,
    /// The duration of the set in seconds
    pub(crate) duration_seconds: Option<f64>,
    /// The RPE of the set
    pub(crate) rpe: Option<f32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_routine() {
        let _example_routine = r#"
{
  "routine": {
    "title": "April Leg Day 🔥",
    "notes": "Focus on form over weight. Remember to stretch.",
    "exercises": [
      {
        "exercise_template_id": "D04AC939",
        "superset_id": null,
        "rest_seconds": 90,
        "notes": "Stay slow and controlled.",
        "sets": [
          {
            "type": "normal",
            "weight_kg": 100,
            "reps": 10,
            "distance_meters": null,
            "duration_seconds": null
          }
        ]
      }
    ]
  }
}
            "#;
    }

    #[test]
    fn deser_routine_set() {
        let example_set = r#"
          {
            "type": "normal",
            "weight_kg": 100,
            "reps": 10,
            "distance_meters": null,
            "duration_seconds": null
          }
            "#;

        let got_set = serde_json::from_str(example_set).unwrap();

        let example_set_concrete = RoutineSet {
            set_type: SetType::Normal,
            weight_kg: Some(100.0),
            reps: Some(10),
            distance_meters: None,
            duration_seconds: None,
        };

        assert_eq!(example_set_concrete, got_set);
    }

    #[test]
    fn deser_workout_set() {
        let example_set = r#"
{
  "index": 0,
  "set_type": "normal",
  "weight_kg": 58.96707822663317,
  "reps": 5,
  "distance_meters": null,
  "duration_seconds": null,
  "rpe": null
}
        "#;

        let got_set = serde_json::from_str(example_set).unwrap();

        let expected_set = Set {
            index: 0,
            set_type: SetType::Normal,
            weight_kg: Some(58.96707822663317),
            reps: Some(5),
            distance_meters: None,
            duration_seconds: None,
            rpe: None,
        };

        assert_eq!(expected_set, got_set);
    }
}
