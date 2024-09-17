use serde::{Deserialize, Serialize};

// TODO: paginated wrapper?
/// An exercise template response.
///
/// The response for `/v1/exercise_templates`.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExerciseTemplateResponse {
    /// The current page number
    page: u64,
    /// The total number of pages
    page_count: u64,
    /// The exercise templates
    exercise_templates: Vec<ExerciseTemplate>,
}

// NOTE: the documentation for this does not match up to the current version.
// Differences: addition of an `equipment` flag, short ID instead of UUID
/// An exercise template, used in the `/v1/exercise_templates` response.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExerciseTemplate {
    /// An exercise id
    #[serde(
        serialize_with = "hex::serde::serialize_upper",
        deserialize_with = "hex::serde::deserialize"
    )]
    id: [u8; 4],
    /// An exercise title
    title: String,
    // TODO: this is probably really an enum, so should check out the possibilities. Maybe
    // `distance` / `duration` / `rpe`?
    /// The exercise type, for example `weight_reps`.
    #[serde(rename = "type")]
    exercise_type: String,
    // TODO: why is the example here `weight_reps`, seems like it should be `chest`. Or another
    // enum, which would also be the value of `secondary_muscle_groups`.
    /// The primary muscle group
    primary_muscle_group: String,
    /// The secondary muscle groups.
    secondary_muscle_groups: Vec<String>,
    // TODO: another that might be possibly an enum.
    /// The equipment needed for the exercise.
    equipment: String,
    /// Whether or not the template is a custom template.
    is_custom: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn template_response_deser() {
        let example_templates_response = r#"
{
  "page": 1,
  "page_count": 423,
  "exercise_templates": [
    {
      "id": "3BC06AD3",
      "title": "21s Bicep Curl",
      "type": "weight_reps",
      "primary_muscle_group": "biceps",
      "secondary_muscle_groups": [],
      "equipment": "barbell",
      "is_custom": false
    }
  ]
}
        "#;

        let got_response: ExerciseTemplateResponse =
            serde_json::from_str(example_templates_response).unwrap();
        let expected_response = ExerciseTemplateResponse {
            page: 1,
            page_count: 423,
            exercise_templates: vec![ExerciseTemplate {
                id: [0x3b, 0xc0, 0x6a, 0xd3],
                title: "21s Bicep Curl".into(),
                exercise_type: "weight_reps".into(),
                primary_muscle_group: "biceps".into(),
                secondary_muscle_groups: vec![],
                equipment: "barbell".into(),
                is_custom: false,
            }],
        };

        assert_eq!(expected_response, got_response);
    }

    #[test]
    fn template_deser() {
        // this is from the docs page example, wrong because of uuid and equipment difference
        //         let template = r#"
        // {
        //   "id": "b459cba5-cd6d-463c-abd6-54f8eafcadcb",
        //   "title": "Bench Press (Barbell)",
        //   "type": "weight_reps",
        //   "primary_muscle_group": "weight_reps",
        //   "secondary_muscle_groups": [
        //     "string"
        //   ],
        //   "is_custom": false
        // }
        //         "#;
        //
        //         let _got_template: ExerciseTemplate = serde_json::from_str(template).unwrap();

        let template = r#"
{
  "id": "3BC06AD3",
  "title": "21s Bicep Curl",
  "type": "weight_reps",
  "primary_muscle_group": "biceps",
  "secondary_muscle_groups": [],
  "equipment": "barbell",
  "is_custom": false
}
        "#;

        let got_template: ExerciseTemplate = serde_json::from_str(template).unwrap();
        let expected_template = ExerciseTemplate {
            id: [0x3b, 0xc0, 0x6a, 0xd3],
            title: "21s Bicep Curl".into(),
            exercise_type: "weight_reps".into(),
            primary_muscle_group: "biceps".into(),
            secondary_muscle_groups: vec![],
            equipment: "barbell".into(),
            is_custom: false,
        };

        assert_eq!(got_template, expected_template);
    }
}
