//! A workout

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

/// A workout. Part of the `/v1/workouts` response.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Workout {
    /// The workout ID.
    id: Uuid,
    /// The workout title.
    title: String,
    /// The workout description.
    description: String,
    /// The workout start time.
    #[serde(with = "time::serde::iso8601")]
    start_time: OffsetDateTime,
    /// The workout end time.
    #[serde(with = "time::serde::iso8601")]
    end_time: OffsetDateTime,
    /// The time the workout was last updated.
    #[serde(with = "time::serde::iso8601")]
    updated_at: OffsetDateTime,
    /// The time the workout was created.
    #[serde(with = "time::serde::iso8601")]
    created_at: OffsetDateTime,
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn deserialize_set() {
        let _workout_example = r#"
{
  "page": 1,
  "page_count": 125,
  "workouts": [
    {
      "id": "a0c8005e-5fd0-42ae-a7f8-564cc487d4c6",
      "title": "Midday Workout",
      "description": "",
      "start_time": "2024-07-26T12:33:10+00:00",
      "end_time": "2024-07-26T13:53:10+00:00",
      "updated_at": "2024-07-28T21:28:32.848Z",
      "created_at": "2024-07-28T21:28:32.848Z",
      "exercises": [
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
        },
        {
          "index": 1,
          "title": "Bench Press (Barbell)",
          "notes": "",
          "exercise_template_id": "79D0BB3A",
          "superset_id": null,
          "sets": [
            {
              "index": 0,
              "set_type": "normal",
              "weight_kg": 61.235042773811365,
              "reps": 5,
              "distance_meters": null,
              "duration_seconds": null,
              "rpe": null
            },
            {
              "index": 1,
              "set_type": "normal",
              "weight_kg": 74.84283005688056,
              "reps": 1,
              "distance_meters": null,
              "duration_seconds": null,
              "rpe": null
            },
            {
              "index": 2,
              "set_type": "normal",
              "weight_kg": 79.37875915123695,
              "reps": 1,
              "distance_meters": null,
              "duration_seconds": null,
              "rpe": null
            },
            {
              "index": 3,
              "set_type": "normal",
              "weight_kg": 81.64672369841516,
              "reps": 3,
              "distance_meters": null,
              "duration_seconds": null,
              "rpe": null
            },
            {
              "index": 4,
              "set_type": "normal",
              "weight_kg": 83.91468824559335,
              "reps": 1,
              "distance_meters": null,
              "duration_seconds": null,
              "rpe": null
            },
            {
              "index": 5,
              "set_type": "normal",
              "weight_kg": 70.30690096252415,
              "reps": 8,
              "distance_meters": null,
              "duration_seconds": null,
              "rpe": null
            }
          ]
        },
        {
          "index": 2,
          "title": "Deadlift (Barbell)",
          "notes": "",
          "exercise_template_id": "C6272009",
          "superset_id": null,
          "sets": [
            {
              "index": 0,
              "set_type": "normal",
              "weight_kg": 61.235042773811365,
              "reps": 1,
              "distance_meters": null,
              "duration_seconds": null,
              "rpe": null
            },
            {
              "index": 1,
              "set_type": "normal",
              "weight_kg": 83.91468824559335,
              "reps": 1,
              "distance_meters": null,
              "duration_seconds": null,
              "rpe": null
            },
            {
              "index": 2,
              "set_type": "normal",
              "weight_kg": 102.05840462301894,
              "reps": 1,
              "distance_meters": null,
              "duration_seconds": null,
              "rpe": null
            },
            {
              "index": 3,
              "set_type": "normal",
              "weight_kg": 124.73805009480093,
              "reps": 1,
              "distance_meters": null,
              "duration_seconds": null,
              "rpe": null
            },
            {
              "index": 4,
              "set_type": "normal",
              "weight_kg": 115.66619190608813,
              "reps": 6,
              "distance_meters": null,
              "duration_seconds": null,
              "rpe": null
            }
          ]
        },
        {
          "index": 3,
          "title": "Squat (Barbell)",
          "notes": "",
          "exercise_template_id": "D04AC939",
          "superset_id": null,
          "sets": [
            {
              "index": 0,
              "set_type": "normal",
              "weight_kg": 61.235042773811365,
              "reps": 5,
              "distance_meters": null,
              "duration_seconds": null,
              "rpe": null
            },
            {
              "index": 1,
              "set_type": "normal",
              "weight_kg": 83.91468824559335,
              "reps": 5,
              "distance_meters": null,
              "duration_seconds": null,
              "rpe": null
            },
            {
              "index": 2,
              "set_type": "normal",
              "weight_kg": 90.71858188712795,
              "reps": 3,
              "distance_meters": null,
              "duration_seconds": null,
              "rpe": null
            },
            {
              "index": 3,
              "set_type": "normal",
              "weight_kg": 99.79044007584075,
              "reps": 3,
              "distance_meters": null,
              "duration_seconds": null,
              "rpe": null
            },
            {
              "index": 4,
              "set_type": "normal",
              "weight_kg": 92.98654643430615,
              "reps": 3,
              "distance_meters": null,
              "duration_seconds": null,
              "rpe": null
            }
          ]
        }
      ]
    }
  ]
}"#;
        // let workout = serde_json::from_str(workout_example).unwrap();
    }
}
