//! Implementation of I/O for `sv build-bgdb`.

use serde::{de::IntoDeserializer, Deserialize, Deserializer, Serialize};

use crate::sv_query::schema::{StrandOrientation, SvType};

/// Representation of the fields from the `StructuralVariant` table from VarFish Server
/// that we need for building the background records.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FileRecord {
    /// genome build
    pub release: String,
    /// chromosome name
    pub chromosome: String,
    /// UCSC bin
    pub bin: i32,
    /// start position, 1-based
    pub start: i32,
    /// chromosome2 name
    pub chromosome2: String,
    /// end position, 1-based
    pub end: i32,
    /// paired-end orientation
    #[serde(deserialize_with = "from_varfish_pe_orientation")]
    pub pe_orientation: StrandOrientation,
    /// SV type of the record
    #[serde(deserialize_with = "from_varfish_sv_type")]
    pub sv_type: SvType,
    /// number of hom. alt. carriers
    pub num_hom_alt: u32,
    /// number of hom. ref. carriers
    pub num_hom_ref: u32,
    /// number of het. carriers
    pub num_het: u32,
    /// number of hemi. alt. carriers
    pub num_hemi_alt: u32,
    /// number of hemi. ref. carriers
    pub num_hemi_ref: u32,
}

impl Default for FileRecord {
    fn default() -> Self {
        Self {
            release: "".to_owned(),
            chromosome: "".to_owned(),
            bin: 0,
            start: 0,
            chromosome2: "".to_owned(),
            end: 0,
            pe_orientation: StrandOrientation::NotApplicable,
            sv_type: SvType::Bnd,
            num_hom_alt: 0,
            num_hom_ref: 0,
            num_het: 0,
            num_hemi_alt: 0,
            num_hemi_ref: 0,
        }
    }
}

/// Deserialize "sv_type" from VarFish database.
///
/// This function will strip everything after the first underscore.
fn from_varfish_sv_type<'de, D>(deserializer: D) -> Result<SvType, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    let end = s.find('_').unwrap_or(s.len());
    SvType::deserialize(s[..end].into_deserializer())
}

/// Deserialize "pe_orientation" from VarFish database.
///
/// This function will convert `"."` to `StrandOrientation::NotApplicable`
fn from_varfish_pe_orientation<'de, D>(deserializer: D) -> Result<StrandOrientation, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    if s.eq(".") {
        Ok(StrandOrientation::NotApplicable)
    } else {
        StrandOrientation::deserialize(s.into_deserializer())
    }
}