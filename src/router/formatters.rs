use crate::hevy::{ExerciseTemplate, Routine, RoutineFolder, Workout};

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

    output.push_str(&format!("\nExercises ({}):\n", workout.exercises.len()));

    for exercise in &workout.exercises {
        output.push_str(&format!(
            "  {}. Exercise Template ID: {}\n",
            exercise.index + 1,
            exercise.exercise_template_id
        ));
        if let Some(notes) = &exercise.notes {
            output.push_str(&format!("     Notes: {}\n", notes));
        }
        for set in &exercise.sets {
            let mut set_info = format!("     Set {}: ", set.index + 1);
            if let Some(w) = set.weight_kg {
                set_info.push_str(&format!("{:.1}kg ", w));
            }
            if let Some(r) = set.reps {
                set_info.push_str(&format!("x{} reps ", r));
            }
            if let Some(d) = set.duration_seconds {
                set_info.push_str(&format!("{}s ", d));
            }
            output.push_str(&format!("{}\n", set_info.trim()));
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

    output.push_str(&format!("\nExercises ({}):\n", routine.exercises.len()));

    for exercise in &routine.exercises {
        output.push_str(&format!(
            "  {}. Exercise Template ID: {}\n",
            exercise.index + 1,
            exercise.exercise_template_id
        ));
        if let Some(notes) = &exercise.notes {
            output.push_str(&format!("     Notes: {}\n", notes));
        }
        output.push_str(&format!("     Sets: {}\n", exercise.sets.len()));
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
