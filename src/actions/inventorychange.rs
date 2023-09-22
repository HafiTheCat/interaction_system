use super::Action;
struct InventoryChange {}
impl Action for InventoryChange {
  const ACTION_ID: &'static str = "action.inventorychange";
}