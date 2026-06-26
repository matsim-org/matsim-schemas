pub mod matsim {
    pub mod simulation {
        pub mod io {
            pub mod types {
                include!(concat!(env!("OUT_DIR"), "/matsim.simulation.io.types.rs"));
            }
        }
    }
}
