use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobLocation {
    Remote,
    Onsite,
    Hybrid,
}

impl JobLocation {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Remote => "REMOTE",
            Self::Onsite => "ONSITE",
            Self::Hybrid => "HYBRID",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobType {
    FullTime,
    PartTime,
    Contract,
    Freelance,
    Internship,
}

impl JobType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::FullTime => "FULL_TIME",
            Self::PartTime => "PART_TIME",
            Self::Contract => "CONTRACT",
            Self::Freelance => "FREELANCE",
            Self::Internship => "INTERNSHIP",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobCategory {
    Programming,
    Blockchain,
    Design,
    Marketing,
    CustomerSupport,
    Writing,
    Product,
    Service,
    HumanResource,
    Others,
}

impl JobCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Programming => "PROGRAMMING",
            Self::Blockchain => "BLOCKCHAIN",
            Self::Design => "DESIGN",
            Self::Marketing => "MARKETING",
            Self::CustomerSupport => "CUSTOMER_SUPPORT",
            Self::Writing => "WRITING",
            Self::Product => "PRODUCT",
            Self::Service => "SERVICE",
            Self::HumanResource => "HUMAN_RESOURCE",
            Self::Others => "ELSE",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobExperienceLevel {
    All,
    Junior,
    Mid,
    Senior,
    NoExperienceRequired,
}

impl JobExperienceLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::All => "ALL",
            Self::Junior => "JUNIOR",
            Self::Mid => "MID",
            Self::Senior => "SENIOR",
            Self::NoExperienceRequired => "NO_EXPERIENCE_REQUIRED",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobStatus {
    Draft,
    Paid,
    Confirmed,
    Hold,
    Review,
    Closed,
}

impl JobStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Draft => "DRAFT",
            Self::Paid => "PAID",
            Self::Confirmed => "CONFIRMED",
            Self::Hold => "HOLD",
            Self::Review => "REVIEW",
            Self::Closed => "CLOSED",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserJobCreatedByUser {
    pub slug: String,
}
