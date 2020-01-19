pub mod fuel_economy{
    pub mod mileage_calculator{
        pub fn km_per_litre(kms: f64, fuel_consumed: f64) -> f64{
            kms/fuel_consumed
        }
    }
}
