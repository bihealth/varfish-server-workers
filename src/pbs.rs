//! Data structures for (de-)serialization as generated by `prost-build`.

/// Code generate for protobufs by `prost-build`.
pub mod varfish {
    /// Code generate for protobufs by `prost-build`.
    pub mod v1 {
        /// Code generate for protobufs by `prost-build`.
        pub mod common {
            /// Code generate for protobufs by `prost-build`.
            pub mod misc {
                include!(concat!(env!("OUT_DIR"), "/varfish.v1.common.misc.rs"));
                include!(concat!(env!("OUT_DIR"), "/varfish.v1.common.misc.serde.rs"));
            }
        }

        /// Code generate for protobufs by `prost-build`.
        pub mod seqvars {
            /// Code generate for protobufs by `prost-build`.
            pub mod query {
                include!(concat!(env!("OUT_DIR"), "/varfish.v1.seqvars.query.rs"));
                include!(concat!(
                    env!("OUT_DIR"),
                    "/varfish.v1.seqvars.query.serde.rs"
                ));
            }

            /// Code generate for protobufs by `prost-build`.
            pub mod output {
                include!(concat!(env!("OUT_DIR"), "/varfish.v1.seqvars.output.rs"));
                include!(concat!(
                    env!("OUT_DIR"),
                    "/varfish.v1.seqvars.output.serde.rs"
                ));
            }
        }

        /// Code generate for protobufs by `prost-build`.
        pub mod strucvars {
            /// Code generate for protobufs by `prost-build`.
            pub mod clinvar {
                include!(concat!(env!("OUT_DIR"), "/varfish.v1.strucvars.clinvar.rs"));
                include!(concat!(
                    env!("OUT_DIR"),
                    "/varfish.v1.strucvars.clinvar.serde.rs"
                ));
            }

            /// Code generate for protobufs by `prost-build`.
            pub mod bgdb {
                include!(concat!(env!("OUT_DIR"), "/varfish.v1.strucvars.bgdb.rs"));
                include!(concat!(
                    env!("OUT_DIR"),
                    "/varfish.v1.strucvars.bgdb.serde.rs"
                ));
            }
        }
    }
}
