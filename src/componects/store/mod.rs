pub mod bundle;
mod state;
mod store_weapon;
mod subscription;
mod update;
mod view;

#[derive(Clone, Debug, Default)]
pub struct Store {
    pub weapons: Vec<state::QuickShowStore>,
    pub bundles: Vec<state::Bundle>,
}
