use crate::seqvars::query::schema::{CaseQuery, SequenceVariant};

/// Determine whether the `SequenceVariant` passes the frequency filter.
pub fn passes(query: &CaseQuery, s: &SequenceVariant) -> Result<bool, anyhow::Error> {
    let pop = &query.population_freqeuecy;
    let is_mtdna = annonars::common::cli::canonicalize(&s.chrom) == "MT";

    if is_mtdna {
        if pop.helixmtdb.enabled
            && (pop.helixmtdb.frequency.is_some()
                && s.helixmtdb_af() > pop.helixmtdb.frequency.expect("tested before")
                || pop.helixmtdb.heteroplasmic.is_some()
                    && s.helix_het > pop.helixmtdb.heteroplasmic.expect("tested before")
                || pop.helixmtdb.homoplasmic.is_some()
                    && s.helix_hom > pop.helixmtdb.homoplasmic.expect("tested before"))
        {
            tracing::trace!(
                "variant {:?} fails HelixMtDb frequency filter {:?}",
                s,
                &pop
            );
            return Ok(false);
        }
    } else if pop.gnomad.exomes_enabled
        && (pop.gnomad.exomes_frequency.is_some()
            && s.gnomad_exomes_af() > pop.gnomad.exomes_frequency.expect("tested before")
            || pop.gnomad.exomes_heterozygous.is_some()
                && s.gnomad_exomes_het > pop.gnomad.exomes_heterozygous.expect("tested before")
            || pop.gnomad.exomes_homozygous.is_some()
                && s.gnomad_exomes_hom > pop.gnomad.exomes_homozygous.expect("tested before")
            || pop.gnomad.exomes_hemizygous.is_some()
                && s.gnomad_exomes_hemi > pop.gnomad.exomes_hemizygous.expect("tested before"))
    {
        tracing::trace!(
            "variant {:?} fails gnomAD exomes frequency filter {:?}",
            s,
            &pop.gnomad.exomes_frequency
        );
        return Ok(false);
    }

    if pop.gnomad.genomes_enabled
        && (pop.gnomad.genomes_frequency.is_some()
            && s.gnomad_genomes_af() > pop.gnomad.genomes_frequency.expect("tested before")
            || pop.gnomad.genomes_heterozygous.is_some()
                && s.gnomad_genomes_het > pop.gnomad.genomes_heterozygous.expect("tested before")
            || pop.gnomad.genomes_homozygous.is_some()
                && s.gnomad_genomes_hom > pop.gnomad.genomes_homozygous.expect("tested before")
            || !is_mtdna
                && pop.gnomad.genomes_hemizygous.is_some()
                && s.gnomad_genomes_hemi > pop.gnomad.genomes_hemizygous.expect("tested before"))
    {
        tracing::trace!(
            "variant {:?} fails gnomAD genomes frequency filter {:?}",
            s,
            &pop.gnomad.genomes_frequency
        );
        return Ok(false);
    }

    Ok(true)
}

#[cfg(test)]
#[allow(clippy::too_many_arguments)]
mod test {
    use mehari::annotate::seqvars::ann::{AnnField, Consequence};
    use rstest::rstest;

    use crate::seqvars::query::schema::{CaseQuery, SequenceVariant};

    #[rstest]
    // -- frequency ---------------------------------------------------------
    // frequency: pass [het count] (no filter value)
    #[case(1000, 1, 0, 0, true, None, None, None, None, true)]
    // frequency: pass [het count]
    #[case(1000, 1, 0, 0, true, Some(0.001), None, None, None, true)]
    // frequency: fail [het count]
    #[case(1000, 2, 0, 0, true, Some(0.001), None, None, None, false)]
    // frequency: pass [het count] (fail but filter is disabled)
    #[case(1000, 2, 0, 0, false, Some(0.001), None, None, None, true)]
    // frequency: pass [hom count] (no filter value)
    #[case(1000, 0, 1, 0, true, None, None, None, None, true)]
    // frequency: pass [hom count]
    #[case(1000, 0, 1, 0, true, Some(0.002), None, None, None, true)]
    // frequency: fail [hom count]
    #[case(1000, 0, 2, 0, true, Some(0.002), None, None, None, false)]
    // frequency: pass [hom count] (fail but filter is disabled)
    #[case(1000, 0, 2, 0, false, Some(0.002), None, None, None, true)]
    // frequency: pass [hemi count] (no filter value)
    #[case(1000, 0, 0, 1, true, None, None, None, None, true)]
    // frequency: pass [hemi count]
    #[case(1000, 0, 0, 1, true, Some(0.001), None, None, None, true)]
    // frequency: fail [hemi count]
    #[case(1000, 0, 0, 2, true, Some(0.001), None, None, None, false)]
    // frequency: pass [hemi count] (fail but filter is disabled)
    #[case(1000, 0, 0, 2, false, Some(0.001), None, None, None, true)]
    // -- heterezygous count ------------------------------------------------
    // het. count: pass (no filter value)
    #[case(1000, 1, 0, 0, true, None, None, None, None, true)]
    // het. count: pass
    #[case(1000, 1, 0, 0, true, None, Some(1), None, None, true)]
    // het. count: fail
    #[case(1000, 2, 0, 0, true, None, Some(1), None, None, false)]
    // het. count: pass (fail but filter is disabled)
    #[case(1000, 2, 0, 0, false, None, Some(1), None, None, true)]
    // -- homozygous count --------------------------------------------------
    // hom. count: pass (no filter value)
    #[case(1000, 0, 1, 0, true, None, None, None, None, true)]
    // hom. count: pass
    #[case(1000, 0, 1, 0, true, None, None, Some(1), None, true)]
    // hom. count: fail
    #[case(1000, 0, 2, 0, true, None, None, Some(1), None, false)]
    // hom. count: pass (fail but filter is disabled)
    #[case(1000, 0, 2, 0, false, None, None, Some(1), None, true)]
    // -- hemizygous count --------------------------------------------------
    // hemi. count: pass (no filter value)
    #[case(1000, 0, 1, 0, true, None, None, None, None, true)]
    // hemi. count: pass
    #[case(1000, 0, 0, 1, true, None, None, None, Some(1), true)]
    // hemi. count: fail
    #[case(1000, 0, 0, 2, true, None, None, None, Some(1), false)]
    // hemi. count: pass (fail but filter is disabled)
    #[case(1000, 0, 0, 2, false, None, None, None, Some(1), true)]
    fn passes_frequency_gnomad_exomes_nuclear_dna(
        #[case] seqvar_gnomad_exomes_an: i32,
        #[case] seqvar_gnomad_exomes_het: i32,
        #[case] seqvar_gnomad_exomes_hom: i32,
        #[case] seqvar_gnomad_exomes_hemi: i32,
        #[case] query_gnomad_exomes_enabled: bool,
        #[case] query_gnomad_exomes_frequency: Option<f32>,
        #[case] query_gnomad_exomes_heterozygous: Option<i32>,
        #[case] query_gnomad_exomes_homozygous: Option<i32>,
        #[case] query_gnomad_exomes_hemizygous: Option<i32>,
        #[case] expected_pass_all: bool,
    ) -> Result<(), anyhow::Error> {
        use crate::seqvars::query::schema::{GnomadOptions, PopulationFrequencyOptions};

        let query = CaseQuery {
            population_freqeuecy: PopulationFrequencyOptions {
                gnomad: GnomadOptions {
                    exomes_enabled: query_gnomad_exomes_enabled,
                    exomes_frequency: query_gnomad_exomes_frequency,
                    exomes_heterozygous: query_gnomad_exomes_heterozygous,
                    exomes_homozygous: query_gnomad_exomes_homozygous,
                    exomes_hemizygous: query_gnomad_exomes_hemizygous,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        let seq_var = SequenceVariant {
            gnomad_exomes_an: seqvar_gnomad_exomes_an,
            gnomad_exomes_het: seqvar_gnomad_exomes_het,
            gnomad_exomes_hom: seqvar_gnomad_exomes_hom,
            gnomad_exomes_hemi: seqvar_gnomad_exomes_hemi,
            chrom: "X".to_string(),
            reference: "G".into(),
            alternative: "A".into(),
            ann_fields: vec![AnnField {
                allele: mehari::annotate::seqvars::ann::Allele::Alt {
                    alternative: "A".into(),
                },
                consequences: vec![Consequence::MissenseVariant],
                putative_impact: Consequence::MissenseVariant.impact(),
                gene_symbol: Default::default(),
                gene_id: Default::default(),
                feature_type: mehari::annotate::seqvars::ann::FeatureType::SoTerm {
                    term: mehari::annotate::seqvars::ann::SoFeature::Transcript,
                },
                feature_id: Default::default(),
                feature_biotype: vec![mehari::annotate::seqvars::ann::FeatureBiotype::Coding],
                rank: Default::default(),
                hgvs_t: Default::default(),
                hgvs_p: Default::default(),
                tx_pos: Default::default(),
                cds_pos: Default::default(),
                protein_pos: Default::default(),
                distance: Default::default(),
                messages: Default::default(),
            }],
            ..Default::default()
        };

        assert_eq!(super::passes(&query, &seq_var)?, expected_pass_all);

        Ok(())
    }

    #[rstest]
    // -- frequency ---------------------------------------------------------
    // frequency: pass [het count] (no filter value)
    #[case(1000, 1, 0, 0, true, None, None, None, None, true)]
    // frequency: pass [het count]
    #[case(1000, 1, 0, 0, true, Some(0.001), None, None, None, true)]
    // frequency: fail [het count]
    #[case(1000, 2, 0, 0, true, Some(0.001), None, None, None, false)]
    // frequency: pass [het count] (fail but filter is disabled)
    #[case(1000, 2, 0, 0, false, Some(0.001), None, None, None, true)]
    // frequency: pass [hom count] (no filter value)
    #[case(1000, 0, 1, 0, true, None, None, None, None, true)]
    // frequency: pass [hom count]
    #[case(1000, 0, 1, 0, true, Some(0.002), None, None, None, true)]
    // frequency: fail [hom count]
    #[case(1000, 0, 2, 0, true, Some(0.002), None, None, None, false)]
    // frequency: pass [hom count] (fail but filter is disabled)
    #[case(1000, 0, 2, 0, false, Some(0.002), None, None, None, true)]
    // frequency: pass [hemi count] (no filter value)
    #[case(1000, 0, 0, 1, true, None, None, None, None, true)]
    // frequency: pass [hemi count]
    #[case(1000, 0, 0, 1, true, Some(0.001), None, None, None, true)]
    // frequency: fail [hemi count]
    #[case(1000, 0, 0, 2, true, Some(0.001), None, None, None, false)]
    // frequency: pass [hemi count] (fail but filter is disabled)
    #[case(1000, 0, 0, 2, false, Some(0.001), None, None, None, true)]
    // -- heterezygous count ------------------------------------------------
    // het. count: pass (no filter value)
    #[case(1000, 1, 0, 0, true, None, None, None, None, true)]
    // het. count: pass
    #[case(1000, 1, 0, 0, true, None, Some(1), None, None, true)]
    // het. count: fail
    #[case(1000, 2, 0, 0, true, None, Some(1), None, None, false)]
    // het. count: pass (fail but filter is disabled)
    #[case(1000, 2, 0, 0, false, None, Some(1), None, None, true)]
    // -- homozygous count --------------------------------------------------
    // hom. count: pass (no filter value)
    #[case(1000, 0, 1, 0, true, None, None, None, None, true)]
    // hom. count: pass
    #[case(1000, 0, 1, 0, true, None, None, Some(1), None, true)]
    // hom. count: fail
    #[case(1000, 0, 2, 0, true, None, None, Some(1), None, false)]
    // hom. count: pass (fail but filter is disabled)
    #[case(1000, 0, 2, 0, false, None, None, Some(1), None, true)]
    // -- hemizygous count --------------------------------------------------
    // hemi. count: pass (no filter value)
    #[case(1000, 0, 1, 0, true, None, None, None, None, true)]
    // hemi. count: pass
    #[case(1000, 0, 0, 1, true, None, None, None, Some(1), true)]
    // hemi. count: fail
    #[case(1000, 0, 0, 2, true, None, None, None, Some(1), false)]
    // hemi. count: pass (fail but filter is disabled)
    #[case(1000, 0, 0, 2, false, None, None, None, Some(1), true)]
    fn passes_frequency_gnomad_genomes_nuclear_dna(
        #[case] seqvar_gnomad_genomes_an: i32,
        #[case] seqvar_gnomad_genomes_het: i32,
        #[case] seqvar_gnomad_genomes_hom: i32,
        #[case] seqvar_gnomad_genomes_hemi: i32,
        #[case] query_gnomad_genomes_enabled: bool,
        #[case] query_gnomad_genomes_frequency: Option<f32>,
        #[case] query_gnomad_genomes_heterozygous: Option<i32>,
        #[case] query_gnomad_genomes_homozygous: Option<i32>,
        #[case] query_gnomad_genomes_hemizygous: Option<i32>,
        #[case] expected_pass_all: bool,
    ) -> Result<(), anyhow::Error> {
        use crate::seqvars::query::schema::{GnomadOptions, PopulationFrequencyOptions};

        let query = CaseQuery {
            population_freqeuecy: PopulationFrequencyOptions {
                gnomad: GnomadOptions {
                    genomes_enabled: query_gnomad_genomes_enabled,
                    genomes_frequency: query_gnomad_genomes_frequency,
                    genomes_heterozygous: query_gnomad_genomes_heterozygous,
                    genomes_homozygous: query_gnomad_genomes_homozygous,
                    genomes_hemizygous: query_gnomad_genomes_hemizygous,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        let seq_var = SequenceVariant {
            gnomad_genomes_an: seqvar_gnomad_genomes_an,
            gnomad_genomes_het: seqvar_gnomad_genomes_het,
            gnomad_genomes_hom: seqvar_gnomad_genomes_hom,
            gnomad_genomes_hemi: seqvar_gnomad_genomes_hemi,
            chrom: "X".to_string(),
            reference: "G".into(),
            alternative: "A".into(),
            ann_fields: vec![AnnField {
                allele: mehari::annotate::seqvars::ann::Allele::Alt {
                    alternative: "A".into(),
                },
                consequences: vec![Consequence::MissenseVariant],
                putative_impact: Consequence::MissenseVariant.impact(),
                gene_symbol: Default::default(),
                gene_id: Default::default(),
                feature_type: mehari::annotate::seqvars::ann::FeatureType::SoTerm {
                    term: mehari::annotate::seqvars::ann::SoFeature::Transcript,
                },
                feature_id: Default::default(),
                feature_biotype: vec![mehari::annotate::seqvars::ann::FeatureBiotype::Coding],
                rank: Default::default(),
                hgvs_t: Default::default(),
                hgvs_p: Default::default(),
                tx_pos: Default::default(),
                cds_pos: Default::default(),
                protein_pos: Default::default(),
                distance: Default::default(),
                messages: Default::default(),
            }],
            ..Default::default()
        };

        assert_eq!(super::passes(&query, &seq_var)?, expected_pass_all);

        Ok(())
    }

    #[rstest]
    // -- frequency ---------------------------------------------------------
    // frequency: pass [het count] (no filter value)
    #[case(1000, 1, 0, true, None, None, None, true)]
    // frequency: pass [het count]
    #[case(1000, 1, 0, true, Some(0.001), None, None, true)]
    // frequency: fail [het count]
    #[case(1000, 2, 0, true, Some(0.001), None, None, false)]
    // frequency: pass [het count] (fail but filter is disabled)
    #[case(1000, 2, 0, false, Some(0.001), None, None, true)]
    // frequency: pass [hom count] (no filter value)
    #[case(1000, 0, 1, true, None, None, None, true)]
    // frequency: pass [hom count]
    #[case(1000, 0, 1, true, Some(0.002), None, None, true)]
    // frequency: fail [hom count]
    #[case(1000, 0, 2, true, Some(0.002), None, None, false)]
    // frequency: pass [hom count] (fail but filter is disabled)
    #[case(1000, 0, 2, false, Some(0.002), None, None, true)]
    // -- heteroplasmy count ------------------------------------------------
    // het. count: pass (no filter value)
    #[case(1000, 1, 0, true, None, None, None, true)]
    // het. count: pass
    #[case(1000, 1, 0, true, None, Some(1), None, true)]
    // het. count: fail
    #[case(1000, 2, 0, true, None, Some(1), None, false)]
    // het. count: pass (fail but filter is disabled)
    #[case(1000, 2, 0, false, None, Some(1), None, true)]
    // -- homoplasmy count --------------------------------------------------
    // hom. count: pass (no filter value)
    #[case(1000, 0, 1, true, None, None, None, true)]
    // hom. count: pass
    #[case(1000, 0, 1, true, None, None, Some(1), true)]
    // hom. count: fail
    #[case(1000, 0, 2, true, None, None, Some(1), false)]
    // hom. count: pass (fail but filter is disabled)
    #[case(1000, 0, 2, false, None, None, Some(1), true)]
    fn passes_frequency_helix_chrmt(
        #[case] seqvar_helix_an: i32,
        #[case] seqvar_helix_het: i32,
        #[case] seqvar_helix_hom: i32,
        #[case] query_helix_enabled: bool,
        #[case] query_helix_frequency: Option<f32>,
        #[case] query_helix_heteroplasmic: Option<i32>,
        #[case] query_helix_homoplasmic: Option<i32>,
        #[case] expected_pass_all: bool,
    ) -> Result<(), anyhow::Error> {
        use crate::seqvars::query::schema::{HelixMtDbOptions, PopulationFrequencyOptions};

        let query = CaseQuery {
            population_freqeuecy: PopulationFrequencyOptions {
                helixmtdb: HelixMtDbOptions {
                    enabled: query_helix_enabled,
                    frequency: query_helix_frequency,
                    heteroplasmic: query_helix_heteroplasmic,
                    homoplasmic: query_helix_homoplasmic,
                },
                ..Default::default()
            },
            ..Default::default()
        };
        let seq_var = SequenceVariant {
            helix_an: seqvar_helix_an,
            helix_het: seqvar_helix_het,
            helix_hom: seqvar_helix_hom,
            chrom: "MT".to_string(),
            reference: "G".into(),
            alternative: "A".into(),
            ann_fields: vec![AnnField {
                allele: mehari::annotate::seqvars::ann::Allele::Alt {
                    alternative: "A".into(),
                },
                consequences: vec![Consequence::MissenseVariant],
                putative_impact: Consequence::MissenseVariant.impact(),
                gene_symbol: Default::default(),
                gene_id: Default::default(),
                feature_type: mehari::annotate::seqvars::ann::FeatureType::SoTerm {
                    term: mehari::annotate::seqvars::ann::SoFeature::Transcript,
                },
                feature_id: Default::default(),
                feature_biotype: vec![mehari::annotate::seqvars::ann::FeatureBiotype::Coding],
                rank: Default::default(),
                hgvs_t: Default::default(),
                hgvs_p: Default::default(),
                tx_pos: Default::default(),
                cds_pos: Default::default(),
                protein_pos: Default::default(),
                distance: Default::default(),
                messages: Default::default(),
            }],
            ..Default::default()
        };

        assert_eq!(super::passes(&query, &seq_var)?, expected_pass_all);

        Ok(())
    }

    #[rstest]
    // -- frequency ---------------------------------------------------------
    // frequency: pass [het count] (no filter value)
    #[case(1000, 1, 0, true, None, None, None, true)]
    // frequency: pass [het count]
    #[case(1000, 1, 0, true, Some(0.001), None, None, true)]
    // frequency: fail [het count]
    #[case(1000, 2, 0, true, Some(0.001), None, None, false)]
    // frequency: pass [het count] (fail but filter is disabled)
    #[case(1000, 2, 0, false, Some(0.001), None, None, true)]
    // frequency: pass [hom count] (no filter value)
    #[case(1000, 0, 1, true, None, None, None, true)]
    // frequency: pass [hom count]
    #[case(1000, 0, 1, true, Some(0.002), None, None, true)]
    // frequency: fail [hom count]
    #[case(1000, 0, 2, true, Some(0.002), None, None, false)]
    // frequency: pass [hom count] (fail but filter is disabled)
    #[case(1000, 0, 2, false, Some(0.002), None, None, true)]
    // -- heteroplasmy count ------------------------------------------------
    // het. count: pass (no filter value)
    #[case(1000, 1, 0, true, None, None, None, true)]
    // het. count: pass
    #[case(1000, 1, 0, true, None, Some(1), None, true)]
    // het. count: fail
    #[case(1000, 2, 0, true, None, Some(1), None, false)]
    // het. count: pass (fail but filter is disabled)
    #[case(1000, 2, 0, false, None, Some(1), None, true)]
    // -- homoplasmy count --------------------------------------------------
    // hom. count: pass (no filter value)
    #[case(1000, 0, 1, true, None, None, None, true)]
    // hom. count: pass
    #[case(1000, 0, 1, true, None, None, Some(1), true)]
    // hom. count: fail
    #[case(1000, 0, 2, true, None, None, Some(1), false)]
    // hom. count: pass (fail but filter is disabled)
    #[case(1000, 0, 2, false, None, None, Some(1), true)]
    #[allow(clippy::too_many_arguments)]
    fn passes_frequency_gnomad_genomes_chrmt(
        #[case] seqvar_gnomad_genomes_an: i32,
        #[case] seqvar_gnomad_genomes_het: i32,
        #[case] seqvar_gnomad_genomes_hom: i32,
        #[case] query_gnomad_genomes_enabled: bool,
        #[case] query_gnomad_genomes_frequency: Option<f32>,
        #[case] query_gnomad_genomes_heteroplasmic: Option<i32>,
        #[case] query_gnomad_genomes_homoplasmic: Option<i32>,
        #[case] expected_pass_all: bool,
    ) -> Result<(), anyhow::Error> {
        use crate::seqvars::query::schema::{GnomadOptions, PopulationFrequencyOptions};

        let query = CaseQuery {
            population_freqeuecy: PopulationFrequencyOptions {
                gnomad: GnomadOptions {
                    genomes_enabled: query_gnomad_genomes_enabled,
                    genomes_frequency: query_gnomad_genomes_frequency,
                    genomes_heterozygous: query_gnomad_genomes_heteroplasmic,
                    genomes_homozygous: query_gnomad_genomes_homoplasmic,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        let seq_var = SequenceVariant {
            gnomad_genomes_an: seqvar_gnomad_genomes_an,
            gnomad_genomes_het: seqvar_gnomad_genomes_het,
            gnomad_genomes_hom: seqvar_gnomad_genomes_hom,
            chrom: "MT".to_string(),
            reference: "G".into(),
            alternative: "A".into(),
            ann_fields: vec![AnnField {
                allele: mehari::annotate::seqvars::ann::Allele::Alt {
                    alternative: "A".into(),
                },
                consequences: vec![Consequence::MissenseVariant],
                putative_impact: Consequence::MissenseVariant.impact(),
                gene_symbol: Default::default(),
                gene_id: Default::default(),
                feature_type: mehari::annotate::seqvars::ann::FeatureType::SoTerm {
                    term: mehari::annotate::seqvars::ann::SoFeature::Transcript,
                },
                feature_id: Default::default(),
                feature_biotype: vec![mehari::annotate::seqvars::ann::FeatureBiotype::Coding],
                rank: Default::default(),
                hgvs_t: Default::default(),
                hgvs_p: Default::default(),
                tx_pos: Default::default(),
                cds_pos: Default::default(),
                protein_pos: Default::default(),
                distance: Default::default(),
                messages: Default::default(),
            }],
            ..Default::default()
        };

        assert_eq!(super::passes(&query, &seq_var)?, expected_pass_all);

        Ok(())
    }
}
