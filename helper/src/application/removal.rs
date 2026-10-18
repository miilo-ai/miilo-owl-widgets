use crate::{Result, domain::removal::Plan};

pub trait RemovalPort {
    fn inspect(&self) -> Result<Plan>;
    fn remove(&self, plan: &Plan) -> Result<()>;
}
pub fn run(port: &impl RemovalPort, confirmation: Option<&str>) -> Result<Plan> {
    let mut plan = port.inspect()?;
    if let Some(id) = confirmation {
        if plan.remove.is_empty() {
            plan.status = "already_absent".into();
            return Ok(plan);
        }
        if id != plan.plan_id {
            return Err("Removal plan changed; inspect a fresh plan before confirming".into());
        }
        port.remove(&plan)?;
        plan.status = if plan.remove.is_empty() {
            "already_absent"
        } else {
            "removed"
        }
        .into();
    }
    Ok(plan)
}
