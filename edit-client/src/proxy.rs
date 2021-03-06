use crossbeam_channel;

use crate::{
    Client,
    ClientController,
};

use self::crossbeam_channel::Sender;
use edit_common::commands::*;
use failure::Error;
use std::cell::{
    RefCell,
    RefMut,
};
use std::rc::Rc;

#[cfg(not(target_arch = "wasm32"))]
pub struct ProxyClientController {
    pub state: Rc<RefCell<Client>>,
    pub tx_client: Sender<FrontendCommand>,
    pub tx_sync: Sender<ServerCommand>,
}

#[cfg(not(target_arch = "wasm32"))]
impl ClientController for ProxyClientController {
    fn state(&mut self) -> RefMut<Client> {
        self.state.borrow_mut()
    }

    fn send_frontend(&self, req: &FrontendCommand) -> Result<(), Error> {
        log_wasm!(SendClient(req.clone()));
        self.tx_client.send(req.clone());
        Ok(())
    }

    fn send_server(&self, req: &ServerCommand) -> Result<(), Error> {
        log_wasm!(SendSync(req.clone()));
        self.tx_sync.send(req.clone());
        Ok(())
    }
}

// macro_rules! spawn_monkey_task {
//     ($alive:expr, $monkey:expr, $tx:expr, $wait_params:expr, $task:expr) => {{
//         let tx = $tx.clone();
//         let alive = $alive.clone();
//         let monkey = $monkey.clone();
//         thread::spawn::<_, Result<(), Error>>(move || {
//             let mut rng = rand::thread_rng();
//             while alive.load(Ordering::Relaxed) {
//                 thread::sleep(Duration::from_millis(
//                     rng.gen_range($wait_params.0, $wait_params.1),
//                 ));
//                 if monkey.load(Ordering::Relaxed) {
//                     tx.send(Task::ControllerCommand($task))?;
//                 }
//             }
//             Ok(())
//         })
//     }};
// }
