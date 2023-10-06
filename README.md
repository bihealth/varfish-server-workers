[![Crates.io](https://img.shields.io/crates/d/varfish-server-worker.svg)](https://crates.io/crates/varfish-server-worker)
[![CI](https://github.com/bihealth/varfish-server-worker/actions/workflows/rust.yml/badge.svg)](https://github.com/bihealth/varfish-server-worker/actions/workflows/rust.yml)
[![codecov](https://codecov.io/gh/bihealth/varfish-server-worker/branch/main/graph/badge.svg?token=t5oheMdukf)](https://codecov.io/gh/bihealth/varfish-server-worker)
[![DOI](https://zenodo.org/badge/590461457.svg)](https://zenodo.org/badge/latestdoi/590461457)

# VarFish Server Worker

> [!NOTE]
> This repository contains code that runs inside a VarFish Server.
> If you are looking to run your own VarFish Server, look [here at bihealth/varfish-server](https://github.com/bihealth/varfish-server).

This repository contains the worker used by VarFish Server to execute certain background task.
They are written in the Rust programming language to speed up the execution of certain tasks.
At the moment, the following sub commands exist:

- `db` -- subcommands to build binary (protobuf) database files
    - `db to-bin` -- convert text files downloaded by [varfish-db-downloader](https://github.com/bihealth/varfish-db-downloader/) to binary for fast use in query sub commands
    - `db mk-inhouse` -- compile per-case structural variant into an in-house database previously created by `db compile`
- `seqvars` -- subcommands for processing sequence (aka small/SNV/indel) variants
    - `seqvars ingest` -- convert single VCF file into internal format for use with `seqvars query`
    - `seqvars query` -- perform sequence variant filtration and on-the-fly annotation
- `strucvars` -- subcommands for processing structural (aka large variants, CNVs, etc.) variants
    - `strucvars ingest` -- convert one or more structural variant files for use with `strucvars query`
    - `strucvars query` -- perform structural variant filtration and on-the-fly annotation

## Overall Design

For running queries, the worker tool is installed into the VarFish Server image and are run as executables.
Internally, VarFish server works on VCF files stored in an S3 storage.

For import, the user gives the server access to the VCF files to import.
The server will then use the worker executable to ingest the data into the internal format using `{seqvars,strucvars} ingest`.
These files are then stored in the internal S3 storage.

For queries, the server will create a query JSON file and then pass this query JSON file together with the internal file to the worker executable.
The worker will create a result file that can be directly imported by the server to be displayed to the user.

Future versions may provide persistently running HTTP/REST servers that provide functionality without startup cost.

## The `db to-bin` Command

Convert output of [varfish-db-downloader](https://github.com/bihealth/varfish-db-downloader/) to a directory with databases to be used by query commands such as `strucvars query`.

```
$ varfish-server-worker db to-bin \
    --input-type {ClinvarSv,StrucvarInhouse,...} \
    --path-input IN.txt \
    --path-output-bin DST.bin
```

## The `db mk-inhouse` Command

Import multiple files created by `strucvars ingest` into a database previously created by `db compile`.
You can specify the files individually.
Paths starting with an at (`@`) character are interpreted as files with lists of paths.
You can mix paths with `@` and without.

```
$ varfish-server-worker db mk-inhouse \
    --genome-release {Grch37,Grch38} \
    --path-output-tsv OUT.tsv \
    --path-input-tsvs IN/file1.gts.tsv.gz \
    [--path-input-tsv IN/file1.gts.tsv.gz] \

# OR:

$ varfish-server-worker db mk-inhouse \
    --genome-release {Grch37,Grch38} \
    --path-output-tsv OUT.tsv \
    --path-input-tsvs @IN/path-list.txt \
    [--path-input-tsvs @IN/path-list2.txt]
```

## The `seqvars ingest` Command

This command takes as the input a single VCF file from a (supported) variant caller and converts it into a file for further querying.
The command interprets the following fields which are written out by the commonly used variant callers such as GATK UnifiedGenotyper, GATK HaplotypeCaller, and Illumina Dragen.

- `FORMAT/GT` -- genotype
- `FORMAT/GQ` -- genotype quality
- `FORMAT/DP` -- total read coverage
- `FORMAT/AD` -- allelic depth, one value per allele (including reference0)
- `FORMAT/PID` -- physical phasing information as written out by GATK HaplotypeCaller in GVCF workflow
- `FORMAT/PS` -- physical phasing information as written out by Dragen variant caller
    - this field fill be written as `FORMAT/PID`
- `FORMAT/SQ` -- "somatic quality" for each alternate allele, as written out by Illumina Dragen variant caller
    - this field will be written as `FORMAT/GQ`

The `seqvars ingest` command will annotate the variants with the following information:

- gnomAD genomes and exomes allele frequencies
- gnomAD-mtDNA and HelixMtDb allele frequencies
- functional annotation following the [VCF ANN field standard](https://pcingola.github.io/SnpEff/adds/VCFannotationformat_v1.0.pdf)

The command will emit one output line for each variant allele from the input and each affected gene.
That is, if two variant alleles affect two genes, four records will be written to the output file.
The annotation will be written out for one highest impact.

Overall, the command will emit the following header rows in addition to the `##contig=<ID=.,length=.>` lines.

```
##fileformat=VCFv4.2
##FILTER=<ID=PASS,Description="All filters passed">
##INFO=<ID=gnomad_exomes_an,Number=1,Type=Integer,Description="Number of samples in gnomAD exomes">
##INFO=<ID=gnomad_exomes_hom,Number=1,Type=Integer,Description="Number of hom. alt. carriers in gnomAD exomes">
##INFO=<ID=gnomad_exomes_het,Number=1,Type=Integer,Description="Number of het. alt. carriers in gnomAD exomes">
##INFO=<ID=gnomad_exomes_hemi,Number=1,Type=Integer,Description="Number of hemi. alt. carriers in gnomAD exomes">
##INFO=<ID=gnomad_genomes_an,Number=1,Type=Integer,Description="Number of samples in gnomAD genomes">
##INFO=<ID=gnomad_genomes_hom,Number=1,Type=Integer,Description="Number of hom. alt. carriers in gnomAD genomes">
##INFO=<ID=gnomad_genomes_het,Number=1,Type=Integer,Description="Number of het. alt. carriers in gnomAD genomes">
##INFO=<ID=gnomad_genomes_hemi,Number=1,Type=Integer,Description="Number of hemi. alt. carriers in gnomAD genomes">
##INFO=<ID=helix_an,Number=1,Type=Integer,Description="Number of samples in HelixMtDb">
##INFO=<ID=helix_hom,Number=1,Type=Integer,Description="Number of hom. alt. carriers in HelixMtDb">
##INFO=<ID=helix_het,Number=1,Type=Integer,Description="Number of het. alt. carriers in HelixMtDb">
##INFO=<ID=ANN,Number=.,Type=String,Description="Functional annotations: 'Allele | Annotation | Annotation_Impact | Gene_Name | Gene_ID | Feature_Type | Feature_ID | Transcript_BioType | Rank | HGVS.c | HGVS.p | cDNA.pos / cDNA.length | CDS.pos / CDS.length | AA.pos / AA.length | Distance | ERRORS / WARNINGS / INFO'">
##FORMAT=<ID=AD,Number=R,Type=Integer,Description="Allelic depths for the ref and alt alleles in the order listed">
##FORMAT=<ID=DP,Number=1,Type=Integer,Description="Approximate read depth (reads with MQ=255 or with bad mates are filtered)">
##FORMAT=<ID=GQ,Number=1,Type=Integer,Description="Genotype Quality">
##FORMAT=<ID=GT,Number=1,Type=String,Description="Genotype">
##FORMAT=<ID=PID,Number=1,Type=String,Description="Physical phasing ID information, where each unique ID within a given sample (but not across samples) connects records within a phasing group">
##x-varfish-version=<ID=varfish-server-worker,Version=x.y.z>
##x-varfish-version=<ID=orig-caller,Name=Dragen,Version=SW: 07.021.624.3.10.9, HW: 07.021.624>
##x-varfish-version=<ID=orig-caller,Name=GatkHaplotypeCaller,Version=4.4.0.0>
```

> [!NOTE]
> The gnomad-mtDNA information is written to the `INFO/gnomdad_genome_*` fields.

> [!NOTE]
> Future versions of the worker will annotate the worst effect on a MANE select or MANE Clinical transcript.

# Developer Information

This section is only relevant for developers of `varfish-server-worker`.

## Development Setup

You will need a recent version of protocolbuffers, e.g.:

```
# bash utils/install-protoc.sh
# export PATH=$PATH:$HOME/.local/share/protoc/bin
```

## GitHub Project Management

We use Terraform for managing the GitHub project settings (as applicable):

```
$ export GITHUB_OWNER=bihealth
$ export GITHUB_TOKEN=ghp_<thetoken>

$ cd utils/terraform
$ terraform init
$ terraform import github_repository.varfish-sever-worker varfish-sever-worker
$ terraform validate
$ terraform fmt
$ terraform plan
$ terraform apply
```
