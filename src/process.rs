use crate::{error::ProjectError, model::Model};
use psutil::process::{processes, Process, ProcessError as PProcessError};

#[derive(Debug)]
pub struct ProcessWrapper {
    pub name: String,
    pub path: String,
    process: Process,
}

impl ProcessWrapper {
    pub fn kill(&self) -> Result<(), ProjectError> {
        self.process.kill().map_err(|_| ProjectError::ProcessError)
    }
}

pub struct ProcessIterator {
    processes: Vec<Result<Process, PProcessError>>,
    index: usize,
}

impl<'a> ProcessIterator {
    pub(crate) fn new(processes: Vec<Result<Process, PProcessError>>) -> Self {
        ProcessIterator {
            processes,
            index: 0,
        }
    }

    pub(crate) fn xiterate(&mut self, model: &'a mut Model) {
        if let Ok(_) = model.load() {
            if let Ok(processes) = processes() {
                for process in &processes {
                    for item in &model.items {
                        item.rules.iter().for_each(|rule| {
                            if let Ok(process) = process {
                                if let Ok(name) = process.name() {
                                    if name == rule.path && item.state == true {
                                        if let Ok(_) = process.kill() {
                                            println!("Process killed {:?}", name);
                                        }
                                    }
                                }
                            }
                        })
                    }
                }
            }
        }
    }
}

impl<'a> Iterator for ProcessIterator {
    type Item = Result<ProcessWrapper, ProjectError>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.processes.len() {
            let processes = self.processes[self.index].as_ref();
            if let Ok(process) = processes {
                self.index += 1;
                if let Ok(path) = process.exe() {
                    if !path.to_string_lossy().to_string().is_empty() {
                        return Some(Ok(ProcessWrapper {
                            name: process.name().unwrap_or_default().to_owned(),
                            path: path.to_string_lossy().to_string(),
                            process: process.clone(),
                        }));
                    }
                } else {
                    return Some(Err(ProjectError::ProcessError));
                }
            } else {
                return Some(Err(ProjectError::ProcessError));
            }
        }
        None
    }
}
