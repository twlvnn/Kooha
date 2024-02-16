use std::fmt;

use gtk::glib;
use num_traits::Signed;

use crate::pipeline::Framerate;

/// The available options for the framerate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, glib::Enum)]
#[enum_type(name = "KoohaFramerateOption")]
pub enum FramerateOption {
    _10,
    _20,
    _24,
    _25,
    _29_97,
    _30,
    _48,
    _50,
    _59_94,
    _60,
}

impl FramerateOption {
    fn all() -> [Self; 10] {
        [
            Self::_10,
            Self::_20,
            Self::_24,
            Self::_25,
            Self::_29_97,
            Self::_30,
            Self::_48,
            Self::_50,
            Self::_59_94,
            Self::_60,
        ]
    }

    /// Returns the corresponding `FramerateOption` for the given framerate.
    pub fn from_framerate(framerate: Framerate) -> Option<Self> {
        // This must be updated if an option is added or removed.
        let epsilon = Framerate::new_raw(1, 100);

        Self::all()
            .iter()
            .find(|o| (o.as_framerate() - framerate).abs() < epsilon)
            .copied()
    }

    /// Returns the closest `FramerateOption` for the given framerate.
    pub fn from_framerate_closest(framerate: Framerate) -> Self {
        if let Some(option) = Self::from_framerate(framerate) {
            return option;
        }

        tracing::error!("No match for {framerate}. Using closest instead.");

        Self::all()
            .iter()
            .min_by_key(|o| (o.as_framerate() - framerate).abs())
            .copied()
            .unwrap()
    }

    /// Converts a `FramerateOption` to a framerate.
    pub const fn as_framerate(self) -> Framerate {
        let (numer, denom) = match self {
            Self::_10 => (10, 1),
            Self::_20 => (20, 1),
            Self::_24 => (24, 1),
            Self::_25 => (25, 1),
            Self::_29_97 => (30_000, 1001),
            Self::_30 => (30, 1),
            Self::_48 => (48, 1),
            Self::_50 => (50, 1),
            Self::_59_94 => (60_000, 1001),
            Self::_60 => (60, 1),
        };
        Framerate::new_raw(numer, denom)
    }
}

impl fmt::Display for FramerateOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::_10 => "10",
            Self::_20 => "20",
            Self::_24 => "24",
            Self::_25 => "25",
            Self::_29_97 => "29.97",
            Self::_30 => "30",
            Self::_48 => "48",
            Self::_50 => "50",
            Self::_59_94 => "59.94",
            Self::_60 => "60",
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::pipeline::Framerate;

    use super::*;

    #[track_caller]
    fn test_framerate(
        framerate: Framerate,
        expected: Option<FramerateOption>,
        expected_closest: FramerateOption,
    ) {
        assert_eq!(
            FramerateOption::from_framerate(framerate),
            expected,
            "wrong expected"
        );
        assert_eq!(
            FramerateOption::from_framerate_closest(framerate),
            expected_closest,
            "wrong closest expected"
        );
    }

    #[test]
    fn framerate_option() {
        test_framerate(Framerate::from_integer(5), None, FramerateOption::_10);
        test_framerate(
            Framerate::from_integer(10),
            Some(FramerateOption::_10),
            FramerateOption::_10,
        );
        test_framerate(
            Framerate::from_integer(20),
            Some(FramerateOption::_20),
            FramerateOption::_20,
        );
        test_framerate(
            Framerate::from_integer(24),
            Some(FramerateOption::_24),
            FramerateOption::_24,
        );
        test_framerate(
            Framerate::from_integer(25),
            Some(FramerateOption::_25),
            FramerateOption::_25,
        );
        test_framerate(
            Framerate::approximate_float(29.97).unwrap(),
            Some(FramerateOption::_29_97),
            FramerateOption::_29_97,
        );
        test_framerate(
            Framerate::from_integer(30),
            Some(FramerateOption::_30),
            FramerateOption::_30,
        );
        test_framerate(
            Framerate::from_integer(48),
            Some(FramerateOption::_48),
            FramerateOption::_48,
        );
        test_framerate(
            Framerate::from_integer(50),
            Some(FramerateOption::_50),
            FramerateOption::_50,
        );
        test_framerate(
            Framerate::approximate_float(59.94).unwrap(),
            Some(FramerateOption::_59_94),
            FramerateOption::_59_94,
        );
        test_framerate(
            Framerate::from_integer(60),
            Some(FramerateOption::_60),
            FramerateOption::_60,
        );
        test_framerate(Framerate::from_integer(120), None, FramerateOption::_60);
    }
}
