use crate::hevy::types::SetType;
use crate::hevy::{BodyMeasurement, ExerciseTemplate, Routine, RoutineFolder, Workout};

pub(crate) struct SetMetrics<'a> {
    pub set_type: Option<&'a SetType>,
    pub weight_kg: Option<f64>,
    pub reps: Option<u32>,
    pub rep_range: Option<(u32, u32)>,
    pub duration_seconds: Option<u32>,
    pub distance_meters: Option<f64>,
    pub rpe: Option<f64>,
    pub custom_metric: Option<f64>,
}

pub(crate) fn format_set_metrics(metrics: &SetMetrics<'_>) -> String {
    let mut parts = Vec::new();

    if let Some(set_type) = metrics.set_type
        && !matches!(set_type, SetType::Normal)
    {
        parts.push(format!("[{:?}]", set_type));
    }
    if let Some(w) = metrics.weight_kg {
        parts.push(format!("{:.1}kg", w));
    }
    match (metrics.rep_range, metrics.reps) {
        (Some((start, end)), _) => parts.push(format!("x{}-{} reps", start, end)),
        (None, Some(r)) => parts.push(format!("x{} reps", r)),
        (None, None) => {}
    }
    if let Some(d) = metrics.duration_seconds {
        parts.push(format!("{}s", d));
    }
    if let Some(d) = metrics.distance_meters {
        parts.push(format!("{:.0}m", d));
    }
    if let Some(rpe) = metrics.rpe {
        parts.push(format!("RPE {}", rpe));
    }
    if let Some(c) = metrics.custom_metric {
        parts.push(format!("custom {}", c));
    }

    if parts.is_empty() {
        return "no target set".to_string();
    }

    parts.join(" ")
}

pub(crate) fn format_workout(workout: &Workout) -> String {
    let mut output = format!(
        "Workout: {}\nID: {}\nStart: {}\n",
        workout.title, workout.id, workout.start_time
    );

    if let Some(routine_id) = &workout.routine_id {
        output.push_str(&format!("Routine ID: {}\n", routine_id));
    }
    if let Some(end) = &workout.end_time {
        output.push_str(&format!("End: {}\n", end));
    }
    if let Some(desc) = &workout.description {
        output.push_str(&format!("Description: {}\n", desc));
    }
    if let Some(updated) = &workout.updated_at {
        output.push_str(&format!("Updated: {}\n", updated));
    }

    output.push_str(&format!("\nExercises ({}):\n", workout.exercises.len()));

    for exercise in &workout.exercises {
        output.push_str(&format!(
            "  {}. {} (Template ID: {})\n",
            exercise.index + 1,
            exercise.title.as_deref().unwrap_or("Unknown exercise"),
            exercise.exercise_template_id
        ));
        if let Some(superset_id) = exercise.superset_id {
            output.push_str(&format!("     Superset: {}\n", superset_id));
        }
        if let Some(notes) = &exercise.notes
            && !notes.trim().is_empty()
        {
            output.push_str(&format!("     Notes: {}\n", notes));
        }
        for set in &exercise.sets {
            let metrics = format_set_metrics(&SetMetrics {
                set_type: Some(&set.set_type),
                weight_kg: set.weight_kg,
                reps: set.reps,
                rep_range: None,
                duration_seconds: set.duration_seconds,
                distance_meters: set.distance_meters,
                rpe: set.rpe,
                custom_metric: set.custom_metric,
            });
            output.push_str(&format!("     Set {}: {}\n", set.index + 1, metrics));
        }
    }

    output
}

pub(crate) fn format_routine(routine: &Routine) -> String {
    let mut output = format!("Routine: {}\nID: {}\n", routine.title, routine.id);

    if let Some(folder_id) = &routine.folder_id {
        output.push_str(&format!("Folder ID: {}\n", folder_id));
    }
    if let Some(notes) = &routine.notes {
        output.push_str(&format!("Notes: {}\n", notes));
    }
    if let Some(created) = &routine.created_at {
        output.push_str(&format!("Created: {}\n", created));
    }
    if let Some(updated) = &routine.updated_at {
        output.push_str(&format!("Updated: {}\n", updated));
    }

    output.push_str(&format!("\nExercises ({}):\n", routine.exercises.len()));

    for exercise in &routine.exercises {
        output.push_str(&format!(
            "  {}. {} (Template ID: {})\n",
            exercise.index + 1,
            exercise.title.as_deref().unwrap_or("Unknown exercise"),
            exercise.exercise_template_id
        ));
        if let Some(superset_id) = exercise.superset_id {
            output.push_str(&format!("     Superset: {}\n", superset_id));
        }
        if let Some(rest) = exercise.rest_seconds {
            output.push_str(&format!("     Rest: {}s\n", rest));
        }
        if let Some(notes) = &exercise.notes
            && !notes.trim().is_empty()
        {
            output.push_str(&format!("     Notes: {}\n", notes));
        }
        output.push_str(&format!("     Sets ({}):\n", exercise.sets.len()));
        for set in &exercise.sets {
            let metrics = format_set_metrics(&SetMetrics {
                set_type: set.set_type.as_ref(),
                weight_kg: set.weight_kg,
                reps: set.reps,
                rep_range: set.rep_range.as_ref().map(|r| (r.start, r.end)),
                duration_seconds: set.duration_seconds,
                distance_meters: set.distance_meters,
                rpe: None,
                custom_metric: set.custom_metric,
            });
            output.push_str(&format!("       Set {}: {}\n", set.index + 1, metrics));
        }
    }

    output
}

pub(crate) fn format_exercise_template(template: &ExerciseTemplate) -> String {
    let mut output = format!(
        "Exercise: {}\nID: {}\nType: {:?}\nPrimary Muscle: {:?}\n",
        template.title, template.id, template.exercise_type, template.primary_muscle_group
    );

    if !template.secondary_muscle_groups.is_empty() {
        output.push_str(&format!(
            "Secondary Muscles: {:?}\n",
            template.secondary_muscle_groups
        ));
    }

    if let Some(equipment) = &template.equipment {
        output.push_str(&format!("Equipment: {:?}\n", equipment));
    }

    output.push_str(&format!(
        "Custom Exercise: {}\n",
        if template.is_custom { "Yes" } else { "No" }
    ));

    output
}

pub(crate) fn format_body_measurement(measurement: &BodyMeasurement) -> String {
    let mut output = format!("Date: {}\n", measurement.date);

    let values = measurement.values.labelled();
    if values.is_empty() {
        output.push_str("No values recorded.\n");
        return output;
    }

    for (label, value, unit) in values {
        output.push_str(&format!("{}: {}{}\n", label, value, unit));
    }
    output
}

pub(crate) fn format_folder(folder: &RoutineFolder) -> String {
    let mut output = format!(
        "Folder: {}\nID: {}\nIndex: {}\n",
        folder.title, folder.id, folder.index
    );

    if let Some(created) = &folder.created_at {
        output.push_str(&format!("Created: {}\n", created));
    }
    if let Some(updated) = &folder.updated_at {
        output.push_str(&format!("Updated: {}\n", updated));
    }

    output
}
