use planif::enums::TaskCreationFlags;
use planif::schedule::TaskScheduler;
use planif::schedule_builder::{Action, ScheduleBuilder};
use planif::task::Task;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ts = TaskScheduler::new()?;
    let com = ts.get_com();
    let sb = ScheduleBuilder::new(&com).unwrap();

    let folder = "planif";
    let task_name = "test enabled";

    sb.create_logon()
        .in_folder(folder)?
        .author("hanaTsuk1")?
        .trigger("test_enabled_trigger", true)?
        .action(Action::new("test_time_action", "notepad.exe", "", ""))?
        .user_id("")?
        .build()?
        .register(task_name, TaskCreationFlags::CreateOrUpdate as i32)?;

    let task = Task::new()?;
    println!("task enabled: {}", task.is_enabled(folder, task_name)?);
    println!("disable task...");
    task.disable(folder, task_name)?;
    println!("task enabled: {}", task.is_enabled(folder, task_name)?);
    println!("enable task...");
    task.enable(folder, task_name)?;
    println!("task enabled: {}", task.is_enabled(folder, task_name)?);

    Ok(())
}
