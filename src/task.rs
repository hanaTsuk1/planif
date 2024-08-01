use windows::{
    core::BSTR,
    Win32::{
        Foundation::VARIANT_BOOL,
        System::{
            Com::{CoCreateInstance, CLSCTX_ALL, VARIANT},
            TaskScheduler::{IRegisteredTask, ITaskService, TaskScheduler},
        },
    },
};

/// get task state
pub struct Task {
    task_service: ITaskService,
}

impl Task {
    /// Create a new Task
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        unsafe {
            let task_service: ITaskService = CoCreateInstance(&TaskScheduler, None, CLSCTX_ALL)?;
            task_service.Connect(
                VARIANT::default(),
                VARIANT::default(),
                VARIANT::default(),
                VARIANT::default(),
            )?;
            Ok(Self {
                task_service,
            })
        }
    }

    fn get_task(
        &self,
        folder: &str,
        task_name: &str,
    ) -> Result<IRegisteredTask, Box<dyn std::error::Error>> {
        unsafe {
            let folder = self.task_service.GetFolder(&BSTR::from(folder))?;
            let task = folder.GetTask(&BSTR::from(task_name))?;
            Ok(task)
        }
    }

    /// check task enabled
    pub fn is_enabled(
        &self,
        folder: &str,
        task_name: &str,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        unsafe {
            let task = self.get_task(folder, task_name)?;
            let enabled = task.Enabled()?.as_bool();
            Ok(enabled)
        }
    }

    fn set_task(
        &self,
        folder: &str,
        task_name: &str,
        enabled: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            let task = self.get_task(folder, task_name)?;
            Ok(task.SetEnabled(VARIANT_BOOL::from(enabled))?)
        }
    }

    /// enable task
    pub fn enable(&self, folder: &str, task_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.set_task(folder, task_name, true)
    }

    /// disable task
    pub fn disable(&self, folder: &str, task_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.set_task(folder, task_name, false)
    }
}
