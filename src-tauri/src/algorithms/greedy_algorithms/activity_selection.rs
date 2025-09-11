use crate::types::Activity;

pub fn activity_selection(activities: &mut [Activity]) -> Vec<Activity> {
    activities.sort_by_key(|a| a.finish);

    let mut selected = Vec::new();
    let mut last_finish_time = 0;

    for activity in activities.iter() {
        if activity.start >= last_finish_time {
            selected.push(activity.clone());
            last_finish_time = activity.finish;
        }
    }

    selected
}
