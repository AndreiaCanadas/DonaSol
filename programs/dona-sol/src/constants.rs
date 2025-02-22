use std::fmt;

use anchor_lang::prelude::*;

pub const ANCHOR_DISCRIMINATOR: usize = 8;

// Reflection period of 72h = 72 h * 60 min/h * 60 s/min = 259200 s
pub const REFLECTION_PERIOD: u32 = 259200;

// [AC] make sense to declare enums here or in another file?

#[derive(Clone, Debug, PartialEq, AnchorSerialize, AnchorDeserialize)]
pub enum VerificationStatus {
    Pending,
    Verified,
    Rejected,
}

impl Space for VerificationStatus {
    const INIT_SPACE: usize = 1;
}

#[derive(Clone, Debug, PartialEq, AnchorSerialize, AnchorDeserialize)]
pub enum ProfileType {
    Education,
    Healthcare,
    Environment,
    PovertyAlleviation,
    AnimalWelfare,
    HumanRights,
    DisasterRelief,
    Culture,
    Other
}

impl Space for ProfileType {
    const INIT_SPACE: usize = 1;
}

#[derive(Clone, Debug, PartialEq, AnchorSerialize, AnchorDeserialize)]
pub enum UserMilestone {
    DonorNewbie,                // Making the first donation
    GenerosityGrasshopper,      // Donating to 2 different profiles
    CharityChampion,            // 
    BenevolenceBoss,
    AltruismAce,
    KindnessKnight,
    MagnanimityMaster,
    GenerosityGuru,
    PhilanthropyPhenom
}

impl fmt::Display for UserMilestone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let description = match self {
            UserMilestone::DonorNewbie => "Donor Newbie",
            UserMilestone::GenerosityGrasshopper => "Generosity Grasshopper",
            UserMilestone::CharityChampion => "Charity Champion",
            UserMilestone::BenevolenceBoss => "Benevolence Boss",
            UserMilestone::AltruismAce => "Altruism Ace",
            UserMilestone::KindnessKnight => "Kindness Knight",
            UserMilestone::MagnanimityMaster => "Magnanimity Master",
            UserMilestone::GenerosityGuru => "Generosity Guru",
            UserMilestone::PhilanthropyPhenom => "Philanthropy Phenom",
        };
        write!(f, "{}", description)
    }
}
/* TBD Roadmap
Donor Newbie: For making the first donation.
Generosity Grasshopper: For reaching 10 donations.
Philanthropy Padawan: For donating a total of 50 SOL.
Charity Champion: For consistently donating over 6 months.
Benevolence Boss: For being in the top 10 donors of the month.
Altruism Ace: For donating to 5 different causes.
Kindness Knight: For reaching a total of 200 SOL in donations.
Magnanimity Master: For supporting 10 fully funded projects.
Generosity Guru: For referring 5 new donors to the platform.
Philanthropy Phenom: For donating a total of 500 SOL.
 */