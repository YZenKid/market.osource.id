use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActorRole {
    SuperAdmin,
    Seller,
    Guest,
}

#[derive(Debug, Clone)]
pub struct ActorContext {
    pub user_id: Option<Uuid>,
    pub role: ActorRole,
    pub brand_ids: Vec<Uuid>,
}

impl ActorContext {
    pub fn can_access_brand(&self, brand_id: Uuid) -> bool {
        matches!(self.role, ActorRole::SuperAdmin) || self.brand_ids.contains(&brand_id)
    }

    pub fn can_view_payment_proof_file(&self) -> bool {
        matches!(self.role, ActorRole::SuperAdmin)
    }

    pub fn can_view_payment_status_for_brand(&self, brand_id: Uuid) -> bool {
        self.can_access_brand(brand_id)
    }
}
