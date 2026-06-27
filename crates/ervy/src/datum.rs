use crate::color::Style;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Datum {
    pub key: String,
    pub value: DatumValue,
    pub style: Option<Style>,
    pub sides: Option<[usize; 2]>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum DatumValue {
    Scalar(f64),
    Point([f64; 2]),
}

impl Datum {
    pub fn scalar(key: impl Into<String>, value: f64) -> Self {
        Datum {
            key: key.into(),
            value: DatumValue::Scalar(value),
            style: None,
            sides: None,
        }
    }

    pub fn point(key: impl Into<String>, x: f64, y: f64) -> Self {
        Datum {
            key: key.into(),
            value: DatumValue::Point([x, y]),
            style: None,
            sides: None,
        }
    }

    pub fn styled(key: impl Into<String>, value: f64, style: Style) -> Self {
        Datum {
            key: key.into(),
            value: DatumValue::Scalar(value),
            style: Some(style),
            sides: None,
        }
    }

    pub fn value_as_scalar(&self) -> Option<f64> {
        match &self.value {
            DatumValue::Scalar(v) => Some(*v),
            DatumValue::Point(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scalar_datum() {
        let d = Datum::scalar("A", 5.0);
        assert_eq!(d.key, "A");
        assert!(d.style.is_none());
        match d.value {
            DatumValue::Scalar(v) => assert_eq!(v, 5.0),
            _ => panic!("expected scalar"),
        }
    }

    #[test]
    fn point_datum() {
        let d = Datum::point("B", 3.0, 4.0);
        assert_eq!(d.key, "B");
        match d.value {
            DatumValue::Point([x, y]) => {
                assert_eq!(x, 3.0);
                assert_eq!(y, 4.0);
            }
            _ => panic!("expected point"),
        }
    }

    #[test]
    fn styled_datum() {
        let s = crate::color::fg(crate::color::Color::Red, '*');
        let d = Datum::styled("C", 10.0, s);
        assert_eq!(d.key, "C");
        assert!(d.style.is_some());
    }

    #[test]
    fn deserialize_scalar() {
        let json = r#"{"key": "A", "value": 5.0}"#;
        let d: Datum = serde_json::from_str(json).unwrap();
        assert_eq!(d.key, "A");
        match d.value {
            DatumValue::Scalar(v) => assert_eq!(v, 5.0),
            _ => panic!("expected scalar"),
        }
    }

    #[test]
    fn deserialize_point() {
        let json = r#"{"key": "B", "value": [3.0, 4.0]}"#;
        let d: Datum = serde_json::from_str(json).unwrap();
        assert_eq!(d.key, "B");
        match d.value {
            DatumValue::Point([x, y]) => {
                assert_eq!(x, 3.0);
                assert_eq!(y, 4.0);
            }
            _ => panic!("expected point"),
        }
    }
}
