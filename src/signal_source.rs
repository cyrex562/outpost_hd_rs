use std::ops::Index;

pub struct SignalSource<T>
{
    pub delegate_list: Vec<Delegate<T>>
}

impl SignalSource<T>
{
    pub fn empty(&mut self) -> bool {
        self.delegate_list.is_empty()
    }

    pub fn clear(&mut self) {
        self.delegate_list.clear();
    }

    pub fn connect(&mut self, delegate: Delegate<T>) {
        if self.delegate_list.contains(delegate) == false {
            self.delegate_list.push(delegate);
        }
    }

    pub fn disconnect(&mut self, delegate: Delegate<T>) {
        for i in 0 .. self.delegate_list.len() {
            if self.delegate_list[i] == delegate {
                self.delegate_list.remove(i);
            }
        }
    }
}