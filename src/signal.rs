use crate::signal_source::SignalSource;

#[derive(Default,Debug,Clone)]
pub struct Signal<T>
{
    pub signal_source: SignalSource<T>,
}

impl Signal<T> {
    pub fn emit(&mut self, t: T) {
        let delegate_list_copy = self.signal_source.delegate_list.clone();
        for delegate in delegate_list_copy {
            self.signal_source.delegate(t);
        }
    }
}

// TODO
// impl Fn for Signal<T> {
//     extern "rust-call" fn call(&self, args: (T,)) -> Self::Output {
//         self.emit(args.0);
//     }
// }