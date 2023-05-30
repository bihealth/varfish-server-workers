//! Code related to the "db genes build" subcommand.

use std::{collections::HashMap, io::BufRead, time::Instant};

use clap::Parser;
use indicatif::ProgressIterator;
use prost::Message;
use tracing::info;

use crate::{
    db::genes::data,
    pheno::prepare::{indicatif_style, VERSION},
};

use super::data::{acmg_sf, gnomad_constraints, hgnc, ncbi};

/// Command line arguments for `db genes build` sub command.
#[derive(Parser, Debug)]
#[command(author, version, about = "Build genes database", long_about = None)]
pub struct Args {
    /// Path to the directory with the output of the download directory.
    #[arg(long, required = true)]
    pub path_in_download: String,
    /// Path to output RocksDB.
    #[arg(long, required = true)]
    pub path_out_rocksdb: String,
}

/// Load ACMG SF list.
///
/// # Result
///
/// A map from HGNC ID to ACMG SF record.
fn load_acmg(path: String) -> Result<HashMap<String, acmg_sf::Record>, anyhow::Error> {
    info!("  loading ACMG SF list from {}", path);
    let mut result = HashMap::new();

    let mut reader = csv::ReaderBuilder::new().delimiter(b'\t').from_path(path)?;
    for record in reader.deserialize::<acmg_sf::Record>() {
        let record = record?;
        result.insert(record.hgnc_id.clone(), record);
    }

    Ok(result)
}

/// Load gnomAD constraints.
///
/// # Result
///
/// A map from ENSEMBL gene ID to gnomAD constraints record.
fn load_gnomad_constraints(
    path: String,
) -> Result<HashMap<String, gnomad_constraints::Record>, anyhow::Error> {
    info!("  loading gnomAD constraints from {}", path);
    let mut result = HashMap::new();

    let mut reader = csv::ReaderBuilder::new().delimiter(b'\t').from_path(path)?;
    for record in reader.deserialize::<gnomad_constraints::Record>() {
        let record = record?;
        result.insert(record.ensembl_gene_id.clone(), record);
    }

    Ok(result)
}

/// Load HGNC information.
///
/// # Result
///
/// A map from HGNC ID to HGNC record.
fn load_hgnc(path: String) -> Result<HashMap<String, hgnc::Record>, anyhow::Error> {
    info!("  loading HGNC information from {}", path);
    let mut result = HashMap::new();

    let reader = std::fs::File::open(path).map(std::io::BufReader::new)?;
    for line in reader.lines() {
        let line = line?;
        let record = serde_json::from_str::<hgnc::Record>(&line)?;
        result.insert(record.hgnc_id.clone(), record);
    }

    Ok(result)
}

/// Load NCBI information.
///
/// # Result
///
/// A map from NCBI gene ID to NCBI record.
fn load_ncbi(path: String) -> Result<HashMap<String, ncbi::Record>, anyhow::Error> {
    info!("  loading NCBI information from {}", path);
    let mut result = HashMap::new();

    let reader = std::fs::File::open(path).map(std::io::BufReader::new)?;
    for line in reader.lines() {
        let line = line?;
        let record = serde_json::from_str::<ncbi::Record>(&line)?;
        result.insert(record.gene_id.clone(), record);
    }

    Ok(result)
}

/// Convert from `data::*` records to `pbs::*` records.
fn convert_record(record: data::Record) -> super::pbs::Record {
    let data::Record {
        acmg_sf,
        gnomad_constraints,
        hgnc,
        ncbi,
    } = record;

    let acmg_sf = acmg_sf.map(|acmg_sf| {
        let acmg_sf::Record {
            hgnc_id,
            ensembl_gene_id,
            ncbi_gene_id,
            gene_symbol,
            mim_gene_id,
            disease_phenotype,
            disorder_mim,
            phenotype_category,
            inheritance,
            sf_list_version,
            variants_to_report,
        } = acmg_sf;

        super::pbs::AcmgSecondaryFindingRecord {
            hgnc_id,
            ensembl_gene_id,
            ncbi_gene_id,
            gene_symbol,
            mim_gene_id,
            disease_phenotype,
            disorder_mim,
            phenotype_category,
            inheritance,
            sf_list_version,
            variants_to_report,
        }
    });

    let gnomad_constraints = gnomad_constraints.map(|gnomad_constraints| {
        let gnomad_constraints::Record {
            ensembl_gene_id,
            entrez_id,
            gene_symbol,
            exp_lof,
            exp_mis,
            exp_syn,
            mis_z,
            obs_lof,
            obs_mis,
            obs_syn,
            oe_lof,
            oe_lof_lower,
            oe_lof_upper,
            oe_mis,
            oe_mis_lower,
            oe_mis_upper,
            oe_syn,
            oe_syn_lower,
            oe_syn_upper,
            pli,
            syn_z,
            exac_pli,
            exac_obs_lof,
            exac_exp_lof,
            exac_oe_lof,
        } = gnomad_constraints;

        super::pbs::GnomadConstraintsRecord {
            ensembl_gene_id,
            entrez_id,
            gene_symbol,
            exp_lof,
            exp_mis,
            exp_syn,
            mis_z,
            obs_lof,
            obs_mis,
            obs_syn,
            oe_lof,
            oe_lof_lower,
            oe_lof_upper,
            oe_mis,
            oe_mis_lower,
            oe_mis_upper,
            oe_syn,
            oe_syn_lower,
            oe_syn_upper,
            pli,
            syn_z,
            exac_pli,
            exac_obs_lof,
            exac_exp_lof,
            exac_oe_lof,
        }
    });

    let hgnc = {
        let hgnc::Record {
            hgnc_id,
            symbol,
            name,
            locus_group,
            locus_type,
            status,
            location,
            location_sortable,
            alias_symbol,
            alias_name,
            prev_symbol,
            prev_name,
            gene_group,
            gene_group_id,
            date_approved_reserved,
            date_symbol_changed,
            date_name_changed,
            date_modified,
            entrez_id,
            ensembl_gene_id,
            vega_id,
            ucsc_id,
            ena,
            refseq_accession,
            ccds_id,
            uniprot_ids,
            pubmed_id,
            mgd_id,
            rgd_id,
            lsdb,
            cosmic,
            omim_id,
            mirbase,
            homeodb,
            snornabase,
            bioparadigms_slc,
            orphanet,
            pseudogene_org,
            horde_id,
            merops,
            imgt,
            iuphar,
            mamit_trnadb,
            cd,
            lncrnadb,
            enzyme_id,
            intermediate_filament_db,
            agr,
            mane_select,
        } = hgnc;

        Some(super::pbs::HgncRecord {
            hgnc_id,
            symbol,
            name,
            locus_group,
            locus_type,
            status: status as i32,
            location,
            location_sortable,
            alias_symbol: alias_symbol.unwrap_or_default(),
            alias_name: alias_name.unwrap_or_default(),
            prev_symbol: prev_symbol.unwrap_or_default(),
            prev_name: prev_name.unwrap_or_default(),
            gene_group: gene_group.unwrap_or_default(),
            gene_group_id: gene_group_id.unwrap_or_default(),
            date_approved_reserved: date_approved_reserved
                .map(|d| d.format("%Y-%m-%d").to_string()),
            date_symbol_changed: date_symbol_changed.map(|d| d.format("%Y-%m-%d").to_string()),
            date_name_changed: date_name_changed.map(|d| d.format("%Y-%m-%d").to_string()),
            date_modified: date_modified.map(|d| d.format("%Y-%m-%d").to_string()),
            entrez_id,
            ensembl_gene_id,
            vega_id,
            ucsc_id,
            ena: ena.unwrap_or_default(),
            refseq_accession: refseq_accession.unwrap_or_default(),
            ccds_id: ccds_id.unwrap_or_default(),
            uniprot_ids: uniprot_ids.unwrap_or_default(),
            pubmed_id: pubmed_id.unwrap_or_default(),
            mgd_id: mgd_id.unwrap_or_default(),
            rgd_id: rgd_id.unwrap_or_default(),
            lsdb: lsdb
                .map(|lsdb| {
                    lsdb.iter()
                        .map(|lsdb| super::pbs::HgncLsdb {
                            name: lsdb.name.clone(),
                            url: lsdb.url.clone(),
                        })
                        .collect()
                })
                .unwrap_or_default(),
            cosmic,
            omim_id: omim_id.unwrap_or_default(),
            mirbase,
            homeodb,
            snornabase,
            bioparadigms_slc,
            orphanet,
            pseudogene_org,
            horde_id,
            merops,
            imgt,
            iuphar,
            mamit_trnadb,
            cd,
            lncrnadb,
            enzyme_id: enzyme_id.unwrap_or_default(),
            intermediate_filament_db,
            agr,
            mane_select: mane_select.unwrap_or_default(),
        })
    };

    let ncbi = ncbi.map(|ncbi| {
        let ncbi::Record {
            gene_id,
            summary,
            rif_entries,
        } = ncbi;
        super::pbs::NcbiRecord {
            gene_id,
            summary,
            rif_entries: rif_entries
                .map(|rif_entries| {
                    rif_entries
                        .into_iter()
                        .map(|rif_entry| super::pbs::RifEntry {
                            pmids: rif_entry.pmids.unwrap_or_default(),
                            text: rif_entry.text.clone(),
                        })
                        .collect()
                })
                .unwrap_or_default(),
        }
    });

    super::pbs::Record {
        acmg_sf,
        gnomad_constraints,
        hgnc,
        ncbi,
    }
}

/// Write gene database to a RocksDB.
fn write_rocksdb(
    acmg_by_hgnc_id: HashMap<String, acmg_sf::Record>,
    constraints_by_ensembl_id: HashMap<String, gnomad_constraints::Record>,
    hgnc: HashMap<String, hgnc::Record>,
    ncbi_by_ncbi_id: HashMap<String, ncbi::Record>,
    args: &&Args,
) -> Result<(), anyhow::Error> {
    // Construct RocksDB options and open file for writing.
    let options = annonars::common::rocks_utils::tune_options(rocksdb::Options::default(), None);
    let db = rocksdb::DB::open_cf_with_opts(
        &options,
        &args.path_out_rocksdb,
        ["meta", "genes"]
            .iter()
            .map(|name| (name.to_string(), options.clone()))
            .collect::<Vec<_>>(),
    )?;

    let cf_meta = db.cf_handle("meta").unwrap();
    let cf_genes = db.cf_handle("genes").unwrap();

    tracing::info!("  writing meta data to database");
    db.put_cf(&cf_meta, "builder-version", VERSION)?;
    // TODO: read meta information about input data and write out

    tracing::info!("  compose genes data into database");
    let style = indicatif_style();
    for hgnc_record in hgnc.values().progress_with_style(style) {
        let hgnc_id = hgnc_record.hgnc_id.clone();
        let record = convert_record(data::Record {
            acmg_sf: acmg_by_hgnc_id.get(&hgnc_id).cloned(),
            gnomad_constraints: hgnc_record
                .ensembl_gene_id
                .as_ref()
                .map(|ensembl_gene_id| constraints_by_ensembl_id.get(ensembl_gene_id).cloned())
                .unwrap_or_default(),
            hgnc: hgnc_record.clone(),
            ncbi: hgnc_record
                .entrez_id
                .as_ref()
                .map(|entrez_id| ncbi_by_ncbi_id.get(entrez_id).cloned())
                .unwrap_or_default(),
        });
        db.put_cf(&cf_genes, hgnc_id, &record.encode_to_vec())?;
    }

    // Finally, compact manually.
    tracing::info!("  enforce manual compaction");
    annonars::common::rocks_utils::force_compaction_cf(&db, &["meta", "genes"], Some("  "), true)?;

    Ok(())
}

/// Main entry point for the `sv bg-db-to-bin` command.
pub fn run(common_args: &crate::common::Args, args: &Args) -> Result<(), anyhow::Error> {
    info!("Starting `db gene build`");
    info!("  common_args = {:?}", &common_args);
    info!("  args = {:?}", &args);

    let before_loading = Instant::now();
    info!("Loading genes data files...");
    let acmg_by_hgnc_id = load_acmg(format!("{}/genes/acmg/acmg.tsv", args.path_in_download))?;
    let constraints_by_ensembl_id = load_gnomad_constraints(format!(
        "{}/genes/gnomad_constraints/gnomad_constraints.tsv",
        args.path_in_download
    ))?;
    let hgnc = load_hgnc(format!(
        "{}/genes/hgnc/hgnc_info.jsonl",
        args.path_in_download
    ))?;
    let ncbi_by_ncbi_id = load_ncbi(format!(
        "{}/genes/ncbi/gene_info.jsonl",
        args.path_in_download
    ))?;
    info!(
        "... done loadin genes data files in {:?}",
        before_loading.elapsed()
    );

    let before_writing = Instant::now();
    info!("Writing genes database...");
    write_rocksdb(
        acmg_by_hgnc_id,
        constraints_by_ensembl_id,
        hgnc,
        ncbi_by_ncbi_id,
        &args,
    )?;
    info!(
        "... done writing genes database in {:?}",
        before_writing.elapsed()
    );

    Ok(())
}

#[cfg(test)]
pub mod test {
    use super::*;

    use crate::common::Args as CommonArgs;
    use clap_verbosity_flag::Verbosity;
    use temp_testdir::TempDir;

    #[test]
    fn smoke_test() -> Result<(), anyhow::Error> {
        let tmp_dir = TempDir::default();
        let common_args = CommonArgs {
            verbose: Verbosity::new(1, 0),
        };
        let args = Args {
            path_in_download: String::from("tests/db"),
            path_out_rocksdb: tmp_dir
                .to_path_buf()
                .into_os_string()
                .into_string()
                .unwrap(),
        };

        run(&common_args, &args)?;

        Ok(())
    }
}
