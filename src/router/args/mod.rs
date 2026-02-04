mod common;
mod extra;
mod routine;
mod routine_folder;
mod workout;

pub(crate) use common::{IdArgs, PaginationArgs, parse_args};
pub(crate) use extra::{CreateExerciseTemplateArgs, ExerciseHistoryArgs, WorkoutEventsArgs};
pub(crate) use routine::{CreateRoutineArgs, UpdateRoutineArgs};
pub(crate) use routine_folder::CreateRoutineFolderArgs;
pub(crate) use workout::{CreateWorkoutArgs, UpdateWorkoutArgs};
