const SECONDS_IN_EARTH_YEAR: u64 = 31557600;

macro_rules! impl_Planet {
    ($($name:ident: $earth_rel: expr),+) => {
        $(
            pub struct $name;

            impl Planet for $name {
                fn years_during(d: &Duration) -> f64 {
                    d.earth_years / $earth_rel as f64
                }
            }
        )*
    };
}

#[derive(Debug)]
pub struct Duration {
    earth_years: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self {
            earth_years: s as f64 / SECONDS_IN_EARTH_YEAR as f64,
        }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        unimplemented!(
            "convert a duration ({:?}) to the number of years on this planet for that duration",
            d,
        );
    }
}

impl_Planet!(
    Mercury: 0.2408467,
    Venus: 0.61519726,
    Earth: 1,
    Mars: 1.8808158,
    Jupiter: 11.862615,
    Saturn: 29.447498,
    Uranus: 84.016846,
    Neptune: 164.79132
);
