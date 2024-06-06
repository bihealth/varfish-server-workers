//! Data structures for (de-)serialization as generated by `prost-build`.

/// Code generate for protobufs by `prost-build`.
pub mod clinvar {
    include!(concat!(env!("OUT_DIR"), "/varfish.v1.clinvar.rs"));
    include!(concat!(env!("OUT_DIR"), "/varfish.v1.clinvar.serde.rs"));
}

/// Code generate for protobufs by `prost-build`.
pub mod svs {
    include!(concat!(env!("OUT_DIR"), "/varfish.v1.svs.rs"));
    include!(concat!(env!("OUT_DIR"), "/varfish.v1.svs.serde.rs"));
}

/// Code generate for protobufs by `prost-build`.
pub mod worker {
    include!(concat!(env!("OUT_DIR"), "/varfish.v1.worker.rs"));
    include!(concat!(env!("OUT_DIR"), "/varfish.v1.worker.serde.rs"));
}
