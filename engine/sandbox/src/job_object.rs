use std::process::Child;

pub struct ProcessGuard {
    child: Option<Child>,
}

impl ProcessGuard {
    pub fn new(child: Child) -> Self {
        Self { child: Some(child) }
    }

    pub fn id(&self) -> Option<u32> {
        self.child.as_ref().map(|c| c.id())
    }

    pub fn kill(&mut self) {
        if let Some(mut child) = self.child.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }

    pub fn wait_with_output(mut self) -> std::io::Result<std::process::Output> {
        if let Some(child) = self.child.take() {
            child.wait_with_output()
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "进程已被回收",
            ))
        }
    }
}

impl Drop for ProcessGuard {
    fn drop(&mut self) {
        // RAII 保护：超出作用域立即杀死子进程，杜绝后台僵尸进程滞留
        self.kill();
    }
}
