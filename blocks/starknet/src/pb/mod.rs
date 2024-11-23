// @generated
// @@protoc_insertion_point(attribute:parquet)
pub mod parquet {
    include!("parquet.rs");
    // @@protoc_insertion_point(parquet)
}
pub mod pinax {
    // @@protoc_insertion_point(attribute:pinax.starknet)
    pub mod starknet {
        include!("pinax.starknet.rs");
        // @@protoc_insertion_point(pinax.starknet)
    }
}
pub mod sf {
    pub mod starknet {
        pub mod r#type {
            // @@protoc_insertion_point(attribute:sf.starknet.type.v1)
            pub mod v1 {
                include!("sf.starknet.type.v1.rs");
                // @@protoc_insertion_point(sf.starknet.type.v1)
            }
        }
    }
    // @@protoc_insertion_point(attribute:sf.substreams)
    pub mod substreams {
        include!("sf.substreams.rs");
        // @@protoc_insertion_point(sf.substreams)
        pub mod index {
            // @@protoc_insertion_point(attribute:sf.substreams.index.v1)
            pub mod v1 {
                include!("sf.substreams.index.v1.rs");
                // @@protoc_insertion_point(sf.substreams.index.v1)
            }
        }
        pub mod rpc {
            // @@protoc_insertion_point(attribute:sf.substreams.rpc.v2)
            pub mod v2 {
                include!("sf.substreams.rpc.v2.rs");
                // @@protoc_insertion_point(sf.substreams.rpc.v2)
            }
        }
        pub mod sink {
            pub mod service {
                // @@protoc_insertion_point(attribute:sf.substreams.sink.service.v1)
                pub mod v1 {
                    include!("sf.substreams.sink.service.v1.rs");
                    // @@protoc_insertion_point(sf.substreams.sink.service.v1)
                }
            }
        }
        // @@protoc_insertion_point(attribute:sf.substreams.v1)
        pub mod v1 {
            include!("sf.substreams.v1.rs");
            // @@protoc_insertion_point(sf.substreams.v1)
        }
    }
}
