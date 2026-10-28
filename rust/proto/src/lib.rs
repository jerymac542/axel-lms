//! Apexos gRPC Protocol Definitions
//!
//! Generated protocol buffer definitions for Axel LMS services.

// Common types
pub mod apexos {
    pub mod common {
        pub mod v1 {
            include!("apexos.common.v1.rs");
        }
    }
    
    pub mod auth {
        pub mod v1 {
            include!("apexos.auth.v1.rs");
        }
    }
    
    pub mod tenant {
        pub mod v1 {
            include!("apexos.tenant.v1.rs");
        }
    }
    
    pub mod course {
        pub mod v1 {
            include!("apexos.course.v1.rs");
        }
    }
    
    pub mod enrollment {
        pub mod v1 {
            include!("apexos.enrollment.v1.rs");
        }
    }
    
    pub mod assignment {
        pub mod v1 {
            include!("apexos.assignment.v1.rs");
        }
    }
    
    pub mod media {
        pub mod v1 {
            include!("apexos.media.v1.rs");
        }
    }
    
    pub mod search {
        pub mod v1 {
            include!("apexos.search.v1.rs");
        }
    }
    
    pub mod realtime {
        pub mod v1 {
            include!("apexos.realtime.v1.rs");
        }
    }
    
    pub mod ai {
        pub mod v1 {
            include!("apexos.ai.v1.rs");
        }
    }
    
    pub mod lti {
        pub mod v1 {
            include!("apexos.lti.v1.rs");
        }
    }
    
    pub mod scorm {
        pub mod v1 {
            include!("apexos.scorm.v1.rs");
        }
    }
}

// Re-export commonly used types
pub use apexos::common::v1::*;