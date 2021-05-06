pub trait TQueue {
    fn present(&self, swap_chain: &impl super::swap_chain::TSwapChain);

    fn flush(&self);

    fn sync(&self);
}
