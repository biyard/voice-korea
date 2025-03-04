#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::ApiModel;

#[derive(Debug, Clone, PartialEq, Eq, ApiModel, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum Role {
    Admin = 0,
    DeliberationAdmin = 1,
    Analyst = 2,
    Moderator = 3,
    Speaker = 4,
    #[default]
    None = 5,
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::Admin => write!(f, "admin"),
            Role::DeliberationAdmin => write!(f, "public_admin"),
            Role::Analyst => write!(f, "analyst"),
            Role::Moderator => write!(f, "mediator"),
            Role::Speaker => write!(f, "speaker"),
            Role::None => write!(f, "none"),
        }
    }
}
