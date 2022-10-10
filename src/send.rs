unsafe impl Send for crate::Device {}
unsafe impl Send for crate::Enumerator {}
unsafe impl Send for crate::Event {}
unsafe impl Send for crate::MonitorBuilder {}
unsafe impl Send for crate::MonitorSocket {}
unsafe impl Send for crate::Udev {}

unsafe impl<'a, T, E> Send for crate::List<'a, T, E> {}

unsafe impl Sync for crate::Device {}
unsafe impl Sync for crate::Enumerator {}
unsafe impl Sync for crate::Event {}
unsafe impl Sync for crate::MonitorBuilder {}
unsafe impl Sync for crate::MonitorSocket {}
unsafe impl Sync for crate::Udev {}

unsafe impl<'a, T, E> Sync for crate::List<'a, T, E> {}
